/*
 * Apache Iceberg REST Catalog API
 *
 * Defines the specification for the first version of the REST Catalog API. Implementations should ideally support both Iceberg table specs v1 and v2, with priority given to v2.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use std::collections::HashMap;

use iceberg_rust::spec::{
    schema::Schema,
    view_metadata::{Materialization, Version},
};

use crate::models;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateViewRequest<T: Materialization> {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "schema")]
    pub schema: Box<Schema>,
    #[serde(rename = "view-version")]
    pub view_version: Box<Version<T>>,
    #[serde(rename = "properties")]
    pub properties: HashMap<String, String>,
}

impl<T: Materialization> CreateViewRequest<T> {
    pub fn new(
        name: String,
        schema: Schema,
        view_version: Version<T>,
        properties: HashMap<String, String>,
    ) -> CreateViewRequest<T> {
        CreateViewRequest {
            name,
            location: None,
            schema: Box::new(schema),
            view_version: Box::new(view_version),
            properties,
        }
    }
}
