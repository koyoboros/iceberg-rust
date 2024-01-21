use std::sync::Arc;

use datafusion::{
    arrow::error::ArrowError,
    datasource::{empty::EmptyTable, TableProvider},
    prelude::SessionContext,
};
use futures::TryStreamExt;
use iceberg_rust::{
    arrow::write::write_parquet_partitioned,
    catalog::CatalogList,
    materialized_view::{MaterializedView, STORAGE_POSTFIX},
};
use iceberg_rust_spec::spec::materialized_view_metadata::{
    BaseTable, MaterializedViewRepresentation,
};
use itertools::Itertools;

use crate::{
    error::Error,
    sql::{transform_name, transform_relations},
    DataFusionTable,
};

pub async fn refresh_materialized_view(
    matview: &MaterializedView,
    catalog_list: Arc<dyn CatalogList>,
    branch: Option<&str>,
) -> Result<(), Error> {
    let ctx = SessionContext::new();

    let sql = match &matview.metadata().current_version(branch)?.representations[0] {
        MaterializedViewRepresentation::SqlMaterialized {
            sql,
            dialect: _,
            format_version: _,
            storage_table: _,
        } => sql,
    };

    let version_id = matview.metadata().current_version_id;

    let mut storage_table = matview.storage_table(branch).await?;

    let branch = branch.map(ToString::to_string);

    let base_tables = if storage_table.version_id(branch.clone())? == Some(version_id) {
        storage_table
            .base_tables(catalog_list, branch.clone())
            .await?
    } else {
        storage_table
            .base_tables(catalog_list, branch.clone())
            .await?
    };

    // Full refresh

    let new_tables = base_tables
        .into_iter()
        .flat_map(|(catalog_name, base_table, _)| {
            let identifier = base_table
                .identifier()
                .to_string()
                .trim_end_matches(STORAGE_POSTFIX)
                .to_owned();

            let snapshot_id = base_table.metadata().current_snapshot_id.unwrap_or(-1);

            let table = Arc::new(DataFusionTable::new_table(
                base_table,
                None,
                None,
                branch.as_deref(),
            )) as Arc<dyn TableProvider>;
            let schema = table.schema().clone();

            vec![
                (catalog_name.clone(), identifier.clone(), snapshot_id, table),
                (
                    catalog_name,
                    identifier + "__delta__",
                    snapshot_id,
                    Arc::new(EmptyTable::new(schema)) as Arc<dyn TableProvider>,
                ),
            ]
        })
        .map(|(catalog_name, identifier, snapshot_id, table)| {
            ctx.register_table(&transform_name(&identifier), table)?;
            Ok::<_, Error>((catalog_name.clone() + "." + &identifier, snapshot_id))
        })
        .filter_ok(|(identifier, _)| !identifier.ends_with("__delta__"))
        .map_ok(|(identifier, snapshot_id)| BaseTable {
            identifier,
            snapshot_id,
        })
        .collect::<Result<_, _>>()?;

    let sql_statements = transform_relations(sql)?;

    let logical_plan = ctx.state().create_logical_plan(&sql_statements[0]).await?;

    let batches = ctx
        .execute_logical_plan(logical_plan)
        .await?
        .execute_stream()
        .await?
        .map_err(ArrowError::from);

    let files = write_parquet_partitioned(&storage_table, batches, branch.as_deref()).await?;

    storage_table
        .full_refresh(files, version_id, new_tables, branch)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use datafusion::{arrow::array::Int64Array, prelude::SessionContext};
    use iceberg_catalog_sql::SqlCatalogList;
    use iceberg_rust::{
        catalog::CatalogList,
        materialized_view::materialized_view_builder::MaterializedViewBuilder,
        table::table_builder::TableBuilder,
    };
    use iceberg_rust_spec::spec::{
        partition::{PartitionField, PartitionSpecBuilder, Transform},
        schema::Schema,
        types::{PrimitiveType, StructField, StructTypeBuilder, Type},
    };
    use object_store::{memory::InMemory, ObjectStore};
    use std::sync::Arc;

    use crate::{catalog::catalog::IcebergCatalog, materialized_view::refresh_materialized_view};

    #[tokio::test]
    pub async fn test_datafusion_refresh_materialized_view() {
        let object_store: Arc<dyn ObjectStore> = Arc::new(InMemory::new());

        let catalog_list = Arc::new(
            SqlCatalogList::new("sqlite://", object_store.clone())
                .await
                .unwrap(),
        );

        let catalog = catalog_list.catalog("iceberg").await.unwrap();

        let schema = Schema {
            schema_id: 1,
            identifier_field_ids: None,
            fields: StructTypeBuilder::default()
                .with_struct_field(StructField {
                    id: 1,
                    name: "id".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Long),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 2,
                    name: "customer_id".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Long),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 3,
                    name: "product_id".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Long),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 4,
                    name: "date".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Date),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 5,
                    name: "amount".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Int),
                    doc: None,
                })
                .build()
                .unwrap(),
        };
        let partition_spec = PartitionSpecBuilder::default()
            .spec_id(1)
            .with_partition_field(PartitionField {
                source_id: 4,
                field_id: 1000,
                name: "day".to_string(),
                transform: Transform::Day,
            })
            .build()
            .expect("Failed to create partition spec");

        let mut builder = TableBuilder::new("test.orders", catalog.clone())
            .expect("Failed to create table builder");
        builder
            .location("/test/orders")
            .with_schema((1, schema.clone()))
            .current_schema_id(1)
            .with_partition_spec((1, partition_spec))
            .default_spec_id(1);

        builder.build().await.expect("Failed to create table.");

        let matview_schema = Schema {
            schema_id: 1,
            identifier_field_ids: None,
            fields: StructTypeBuilder::default()
                .with_struct_field(StructField {
                    id: 1,
                    name: "product_id".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Long),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 2,
                    name: "amount".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Int),
                    doc: None,
                })
                .build()
                .unwrap(),
        };

        let mut builder = MaterializedViewBuilder::new(
            "select product_id, amount from iceberg.test.orders where product_id < 3;",
            "test.orders_view",
            matview_schema,
            catalog.clone(),
        )
        .expect("Failed to create filesystem view builder.");
        builder.location("test/orders_view");
        let matview = builder
            .build()
            .await
            .expect("Failed to create filesystem view");

        let total_matview_schema = Schema {
            schema_id: 1,
            identifier_field_ids: None,
            fields: StructTypeBuilder::default()
                .with_struct_field(StructField {
                    id: 1,
                    name: "product_id".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Long),
                    doc: None,
                })
                .with_struct_field(StructField {
                    id: 2,
                    name: "amount".to_string(),
                    required: true,
                    field_type: Type::Primitive(PrimitiveType::Long),
                    doc: None,
                })
                .build()
                .unwrap(),
        };

        let mut total_builder = MaterializedViewBuilder::new(
            "select product_id, sum(amount) from iceberg.test.orders_view group by product_id;",
            "test.total_orders",
            total_matview_schema,
            catalog.clone(),
        )
        .expect("Failed to create filesystem view builder.");
        total_builder.location("test/total_orders");
        let total_matview = total_builder
            .build()
            .await
            .expect("Failed to create filesystem view");

        // Datafusion

        let datafusion_catalog = Arc::new(
            IcebergCatalog::new(catalog, None)
                .await
                .expect("Failed to create datafusion catalog"),
        );

        let ctx = SessionContext::new();

        ctx.register_catalog("iceberg", datafusion_catalog);

        ctx.sql(
            "INSERT INTO iceberg.test.orders (id, customer_id, product_id, date, amount) VALUES 
                (1, 1, 1, '2020-01-01', 1),
                (2, 2, 1, '2020-01-01', 1),
                (3, 3, 1, '2020-01-01', 3),
                (4, 1, 2, '2020-02-02', 1),
                (5, 1, 1, '2020-02-02', 2),
                (6, 3, 3, '2020-02-02', 3);",
        )
        .await
        .expect("Failed to create query plan for insert")
        .collect()
        .await
        .expect("Failed to insert values into table");

        refresh_materialized_view(&matview, catalog_list.clone(), None)
            .await
            .expect("Failed to refresh materialized view");

        let batches = ctx
            .sql(
                "select product_id, sum(amount) from iceberg.test.orders_view group by product_id;",
            )
            .await
            .expect("Failed to create plan for select")
            .collect()
            .await
            .expect("Failed to execute select query");

        for batch in batches {
            if batch.num_rows() != 0 {
                let (order_ids, amounts) = (
                    batch
                        .column(0)
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .unwrap(),
                    batch
                        .column(1)
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .unwrap(),
                );
                for (order_id, amount) in order_ids.iter().zip(amounts) {
                    if order_id.unwrap() == 1 {
                        assert_eq!(amount.unwrap(), 7)
                    } else if order_id.unwrap() == 2 {
                        assert_eq!(amount.unwrap(), 1)
                    } else {
                        panic!("Unexpected order id")
                    }
                }
            }
        }

        ctx.sql(
            "INSERT INTO iceberg.test.orders (id, customer_id, product_id, date, amount) VALUES 
                (7, 1, 3, '2020-01-03', 1),
                (8, 2, 1, '2020-01-03', 2),
                (9, 2, 2, '2020-01-03', 1);",
        )
        .await
        .expect("Failed to create query plan for insert")
        .collect()
        .await
        .expect("Failed to insert values into table");

        refresh_materialized_view(&matview, catalog_list.clone(), None)
            .await
            .expect("Failed to refresh materialized view");

        let batches = ctx
            .sql(
                "select product_id, sum(amount) from iceberg.test.orders_view group by product_id;",
            )
            .await
            .expect("Failed to create plan for select")
            .collect()
            .await
            .expect("Failed to execute select query");

        for batch in batches {
            if batch.num_rows() != 0 {
                let (order_ids, amounts) = (
                    batch
                        .column(0)
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .unwrap(),
                    batch
                        .column(1)
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .unwrap(),
                );
                for (order_id, amount) in order_ids.iter().zip(amounts) {
                    if order_id.unwrap() == 1 {
                        assert_eq!(amount.unwrap(), 9)
                    } else if order_id.unwrap() == 2 {
                        assert_eq!(amount.unwrap(), 2)
                    } else {
                        panic!("Unexpected order id")
                    }
                }
            }
        }

        refresh_materialized_view(&total_matview, catalog_list.clone(), None)
            .await
            .expect("Failed to refresh materialized view");

        let batches = ctx
            .sql("select product_id, amount from iceberg.test.total_orders;")
            .await
            .expect("Failed to create plan for select")
            .collect()
            .await
            .expect("Failed to execute select query");

        for batch in batches {
            if batch.num_rows() != 0 {
                let (order_ids, amounts) = (
                    batch
                        .column(0)
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .unwrap(),
                    batch
                        .column(1)
                        .as_any()
                        .downcast_ref::<Int64Array>()
                        .unwrap(),
                );
                for (order_id, amount) in order_ids.iter().zip(amounts) {
                    if order_id.unwrap() == 1 {
                        assert_eq!(amount.unwrap(), 9)
                    } else if order_id.unwrap() == 2 {
                        assert_eq!(amount.unwrap(), 2)
                    } else {
                        panic!("Unexpected order id")
                    }
                }
            }
        }
    }
}
