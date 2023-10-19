/*!
 * Helpers for parquet files
*/

use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Result};

use parquet::{
    file::metadata::RowGroupMetaData,
    format::FileMetaData,
    schema::types::{from_thrift, SchemaDescriptor},
};

use crate::model::{
    manifest::{AvroMap, Content, DataFile, FileFormat},
    partition::{PartitionField, Transform},
    schema::Schema,
    types::{PrimitiveType, Type},
    values::{Struct, Value},
};

/// Read datafile statistics from parquetfile
pub fn parquet_to_datafile(
    location: &str,
    file_metadata: &FileMetaData,
    schema: &Schema,
    partition_spec: &[PartitionField],
) -> Result<DataFile> {
    let mut partition = partition_spec
        .iter()
        .map(|x| {
            let field = schema
                .fields
                .get(x.source_id as usize)
                .ok_or_else(|| anyhow!("Column with id {} is missing in schema.", x.source_id))?;
            Ok((field.name.clone(), None))
        })
        .collect::<Result<Struct>>()?;
    let transforms = partition_spec
        .iter()
        .map(|x| {
            let field = schema
                .fields
                .get(x.source_id as usize)
                .ok_or_else(|| anyhow!("Column with id {} is missing in schema.", x.source_id))?;
            Ok((field.name.clone(), x.transform.clone()))
        })
        .collect::<Result<HashMap<String, Transform>>>()?;
    let parquet_schema = Arc::new(SchemaDescriptor::new(from_thrift(&file_metadata.schema)?));

    let mut file_size = 0;

    let mut column_sizes = Some(AvroMap(HashMap::new()));
    let mut value_counts = Some(AvroMap(HashMap::new()));
    let mut null_value_counts = Some(AvroMap(HashMap::new()));
    let mut distinct_counts = Some(AvroMap(HashMap::new()));
    let mut lower_bounds: Option<HashMap<i32, Value>> = Some(HashMap::new());
    let mut upper_bounds: Option<HashMap<i32, Value>> = Some(HashMap::new());

    for row_group in &file_metadata.row_groups {
        let row_group = RowGroupMetaData::from_thrift(parquet_schema.clone(), row_group.clone())?;

        file_size += row_group.compressed_size();

        for column in row_group.columns() {
            let column_name = column.column_descr().name();
            let id = schema.fields
                    .get_name(column_name)
                    .ok_or_else(|| anyhow!("Error: Failed to add Parquet file to table. Colummn {} doesn't exist in schema.", column_name))?.id;
            if let Some(column_sizes) = &mut column_sizes {
                if let Some(entry) = column_sizes.0.get_mut(&id) {
                    *entry += column.compressed_size()
                }
            }
            if let Some(value_counts) = &mut value_counts {
                if let Some(entry) = value_counts.0.get_mut(&id) {
                    *entry += row_group.num_rows()
                }
            }
            if let Some(statistics) = column.statistics() {
                if let Some(null_value_counts) = &mut null_value_counts {
                    if let Some(entry) = null_value_counts.0.get_mut(&id) {
                        *entry += statistics.null_count() as i64
                    }
                }
                if let Some(distinct_count) = &mut distinct_counts {
                    if let (Some(entry), Some(distinct_count)) =
                        (distinct_count.0.get_mut(&id), statistics.distinct_count())
                    {
                        *entry += distinct_count as i64
                    }
                }
                if let Some(lower_bounds) = &mut lower_bounds {
                    if let Some(entry) = lower_bounds.get_mut(&id) {
                        let data_type = &schema.fields.get(id as usize).ok_or_else(|| anyhow!("Error: Failed to add Parquet file to table. Colummn {} doesn't exist in schema.", column_name))?.field_type;

                        let new = Value::try_from_bytes(statistics.min_bytes(), data_type)?;
                        match (&entry, &new) {
                            (Value::Int(current), Value::Int(new_val)) => {
                                if *current > *new_val {
                                    *entry = new
                                }
                            }
                            (Value::LongInt(current), Value::LongInt(new_val)) => {
                                if *current > *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Float(current), Value::Float(new_val)) => {
                                if *current > *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Double(current), Value::Double(new_val)) => {
                                if *current > *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Date(current), Value::Date(new_val)) => {
                                if *current > *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Time(current), Value::Time(new_val)) => {
                                if *current > *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Timestamp(current), Value::Timestamp(new_val)) => {
                                if *current > *new_val {
                                    *entry = new
                                }
                            }
                            (Value::TimestampTZ(current), Value::TimestampTZ(new_val)) => {
                                if *current > *new_val {
                                    *entry = new
                                }
                            }
                            _ => (),
                        }
                    }
                }
                if let Some(upper_bounds) = &mut upper_bounds {
                    if let Some(entry) = upper_bounds.get_mut(&id) {
                        let data_type = &schema.fields.get(id as usize).ok_or_else(|| anyhow!("Error: Failed to add Parquet file to table. Colummn {} doesn't exist in schema.", column_name))?.field_type;
                        let new = Value::try_from_bytes(statistics.min_bytes(), data_type)?;
                        match (&entry, &new) {
                            (Value::Int(current), Value::Int(new_val)) => {
                                if *current < *new_val {
                                    *entry = new
                                }
                            }
                            (Value::LongInt(current), Value::LongInt(new_val)) => {
                                if *current < *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Float(current), Value::Float(new_val)) => {
                                if *current < *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Double(current), Value::Double(new_val)) => {
                                if *current < *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Date(current), Value::Date(new_val)) => {
                                if *current < *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Time(current), Value::Time(new_val)) => {
                                if *current < *new_val {
                                    *entry = new
                                }
                            }
                            (Value::Timestamp(current), Value::Timestamp(new_val)) => {
                                if *current < *new_val {
                                    *entry = new
                                }
                            }
                            (Value::TimestampTZ(current), Value::TimestampTZ(new_val)) => {
                                if *current < *new_val {
                                    *entry = new
                                }
                            }
                            _ => (),
                        }
                    }
                }
                if let Some(partition_value) = partition.get_mut(column_name) {
                    if partition_value.is_none() {
                        let data_type = &schema.fields.get(id as usize).ok_or_else(|| anyhow!("Error: Failed to add Parquet file to table. Colummn {} doesn't exist in schema.", column_name))?.field_type;
                        match data_type {
                            Type::Primitive(prim) => match prim {
                                PrimitiveType::Date => {
                                    let transform =
                                        transforms.get(column_name).ok_or_else(|| {
                                            anyhow!(
                                                "Transform for column {} doesn't exist",
                                                column_name
                                            )
                                        })?;
                                    let min =
                                        Value::try_from_bytes(statistics.min_bytes(), data_type)?
                                            .tranform(transform)?;
                                    let max =
                                        Value::try_from_bytes(statistics.max_bytes(), data_type)?
                                            .tranform(transform)?;
                                    if min == max {
                                        *partition_value = Some(min)
                                    }
                                }
                                PrimitiveType::Double => {
                                    let transform =
                                        transforms.get(column_name).ok_or_else(|| {
                                            anyhow!(
                                                "Transform for column {} doesn't exist",
                                                column_name
                                            )
                                        })?;
                                    let min =
                                        Value::try_from_bytes(statistics.min_bytes(), data_type)?
                                            .tranform(transform)?;
                                    let max =
                                        Value::try_from_bytes(statistics.max_bytes(), data_type)?
                                            .tranform(transform)?;
                                    if min == max {
                                        *partition_value = Some(min)
                                    }
                                }
                                PrimitiveType::Float => {
                                    let transform =
                                        transforms.get(column_name).ok_or_else(|| {
                                            anyhow!(
                                                "Transform for column {} doesn't exist",
                                                column_name
                                            )
                                        })?;
                                    let min =
                                        Value::try_from_bytes(statistics.min_bytes(), data_type)?
                                            .tranform(transform)?;
                                    let max =
                                        Value::try_from_bytes(statistics.max_bytes(), data_type)?
                                            .tranform(transform)?;
                                    if min == max {
                                        *partition_value = Some(min)
                                    }
                                }
                                PrimitiveType::Int => {
                                    let transform =
                                        transforms.get(column_name).ok_or_else(|| {
                                            anyhow!(
                                                "Transform for column {} doesn't exist",
                                                column_name
                                            )
                                        })?;
                                    let min =
                                        Value::try_from_bytes(statistics.min_bytes(), data_type)?
                                            .tranform(transform)?;
                                    let max =
                                        Value::try_from_bytes(statistics.max_bytes(), data_type)?
                                            .tranform(transform)?;
                                    if min == max {
                                        *partition_value = Some(min)
                                    }
                                }
                                PrimitiveType::Long => {
                                    let transform =
                                        transforms.get(column_name).ok_or_else(|| {
                                            anyhow!(
                                                "Transform for column {} doesn't exist",
                                                column_name
                                            )
                                        })?;
                                    let min =
                                        Value::try_from_bytes(statistics.min_bytes(), data_type)?
                                            .tranform(transform)?;
                                    let max =
                                        Value::try_from_bytes(statistics.max_bytes(), data_type)?
                                            .tranform(transform)?;
                                    if min == max {
                                        *partition_value = Some(min)
                                    }
                                }
                                PrimitiveType::Time => {
                                    let transform =
                                        transforms.get(column_name).ok_or_else(|| {
                                            anyhow!(
                                                "Transform for column {} doesn't exist",
                                                column_name
                                            )
                                        })?;
                                    let min =
                                        Value::try_from_bytes(statistics.min_bytes(), data_type)?
                                            .tranform(transform)?;
                                    let max =
                                        Value::try_from_bytes(statistics.max_bytes(), data_type)?
                                            .tranform(transform)?;
                                    if min == max {
                                        *partition_value = Some(min)
                                    }
                                }
                                PrimitiveType::Timestamp => {
                                    let transform =
                                        transforms.get(column_name).ok_or_else(|| {
                                            anyhow!(
                                                "Transform for column {} doesn't exist",
                                                column_name
                                            )
                                        })?;
                                    let min =
                                        Value::try_from_bytes(statistics.min_bytes(), data_type)?
                                            .tranform(transform)?;
                                    let max =
                                        Value::try_from_bytes(statistics.max_bytes(), data_type)?
                                            .tranform(transform)?;
                                    if min == max {
                                        *partition_value = Some(min)
                                    }
                                }
                                PrimitiveType::Timestampz => {
                                    let transform =
                                        transforms.get(column_name).ok_or_else(|| {
                                            anyhow!(
                                                "Transform for column {} doesn't exist",
                                                column_name
                                            )
                                        })?;
                                    let min =
                                        Value::try_from_bytes(statistics.min_bytes(), data_type)?
                                            .tranform(transform)?;
                                    let max =
                                        Value::try_from_bytes(statistics.max_bytes(), data_type)?
                                            .tranform(transform)?;
                                    if min == max {
                                        *partition_value = Some(min)
                                    }
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                }
            }
        }
    }
    let content = DataFile {
        content: Content::Data,
        file_path: location.to_string(),
        file_format: FileFormat::Parquet,
        partition,
        record_count: file_metadata.num_rows,
        file_size_in_bytes: file_size,
        column_sizes,
        value_counts,
        null_value_counts,
        nan_value_counts: None,
        distinct_counts,
        lower_bounds,
        upper_bounds,
        key_metadata: None,
        split_offsets: None,
        equality_ids: None,
        sort_order_id: None,
    };
    Ok(content)
}
