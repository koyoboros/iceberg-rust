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

use iceberg_rust::{
    catalog::create::{CreateTable, CreateView},
    spec::view_metadata::Materialization,
};
use reqwest;

use super::{configuration, Error};
use crate::{apis::ResponseContent, models};

/// struct for typed errors of method [`commit_transaction`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommitTransactionError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status409(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status500(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status502(models::IcebergErrorResponse),
    Status504(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_namespace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateNamespaceError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status406(models::ErrorModel),
    Status409(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTableError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status409(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_view`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateViewError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::ErrorModel),
    Status409(models::ErrorModel),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drop_namespace`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropNamespaceError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drop_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropTableError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`drop_view`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DropViewError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::ErrorModel),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_namespaces`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListNamespacesError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_tables`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTablesError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_views`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListViewsError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::ErrorModel),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`load_namespace_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoadNamespaceMetadataError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`load_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoadTableError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`load_view`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoadViewError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::ErrorModel),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`namespace_exists`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NamespaceExistsError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`register_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RegisterTableError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status409(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rename_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenameTableError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status406(models::ErrorModel),
    Status409(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`rename_view`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenameViewError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::ErrorModel),
    Status406(models::ErrorModel),
    Status409(models::ErrorModel),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`replace_view`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplaceViewError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::ErrorModel),
    Status409(models::ErrorModel),
    Status419(models::IcebergErrorResponse),
    Status500(models::ErrorModel),
    Status503(models::IcebergErrorResponse),
    Status502(models::ErrorModel),
    Status504(models::ErrorModel),
    Status5XX(models::ErrorModel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`report_metrics`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReportMetricsError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`table_exists`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TableExistsError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_properties`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdatePropertiesError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status406(models::ErrorModel),
    Status422(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_table`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateTableError {
    Status400(models::IcebergErrorResponse),
    Status401(models::IcebergErrorResponse),
    Status403(models::IcebergErrorResponse),
    Status404(models::IcebergErrorResponse),
    Status409(models::IcebergErrorResponse),
    Status419(models::IcebergErrorResponse),
    Status500(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status502(models::IcebergErrorResponse),
    Status504(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`view_exists`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ViewExistsError {
    Status400(),
    Status401(),
    Status404(),
    Status419(models::IcebergErrorResponse),
    Status503(models::IcebergErrorResponse),
    Status5XX(models::IcebergErrorResponse),
    UnknownValue(serde_json::Value),
}

async fn fetch<R, T, E>(
    configuration: &configuration::Configuration,
    method: reqwest::Method,
    prefix: Option<&str>,
    uri_str: &str,
    request: &R,
    headers: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
) -> Result<T, Error<E>>
where
    R: serde::Serialize + ?Sized,
    T: for<'a> serde::Deserialize<'a>,
    E: for<'a> serde::Deserialize<'a>,
{
    let uri_base = match prefix {
        Some(prefix) => format!(
            "{}/v1/{prefix}/",
            configuration.base_path,
            prefix = crate::apis::urlencode(prefix)
        ),
        None => format!("{}/v1/", configuration.base_path,),
    };
    let client = &configuration.client;

    let mut req_builder = client.request(method.clone(), &(uri_base + uri_str));

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    for (key, value) in headers.unwrap_or_default() {
        req_builder = req_builder.header(key, value);
    }
    for (key, value) in query_params.unwrap_or_default() {
        req_builder = req_builder.query(&[(key, value)]);
    }
    if let &reqwest::Method::POST | &reqwest::Method::PUT = &method {
        req_builder = req_builder.json(request);
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        serde_json::from_str(&content).map_err(Error::from)
    } else {
        let entity: Option<E> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::ResponseError(error))
    }
}

async fn fetch_empty<R, E>(
    configuration: &configuration::Configuration,
    method: reqwest::Method,
    prefix: Option<&str>,
    uri_str: &str,
    request: &R,
    headers: Option<HashMap<String, String>>,
    query_params: Option<HashMap<String, String>>,
) -> Result<(), Error<E>>
where
    R: serde::Serialize + ?Sized,
    E: for<'a> serde::Deserialize<'a>,
{
    let uri_base = match prefix {
        Some(prefix) => format!(
            "{}/v1/{prefix}/",
            configuration.base_path,
            prefix = crate::apis::urlencode(prefix)
        ),
        None => format!("{}/v1/", configuration.base_path,),
    };
    let client = &configuration.client;

    let mut req_builder = client.request(method.clone(), &(uri_base + uri_str));

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    for (key, value) in headers.unwrap_or_default() {
        req_builder = req_builder.header(key, value);
    }
    for (key, value) in query_params.unwrap_or_default() {
        req_builder = req_builder.query(&[(key, value)]);
    }
    if let &reqwest::Method::POST | &reqwest::Method::PUT = &method {
        req_builder = req_builder.json(request);
    }

    let req = req_builder.build()?;
    let resp = client.execute(req).await?;

    let status = resp.status();
    let content = resp.text().await?;

    if !status.is_client_error() && !status.is_server_error() {
        Ok(())
    } else {
        let entity: Option<E> = serde_json::from_str(&content).ok();
        let error = ResponseContent {
            status,
            content,
            entity,
        };
        Err(Error::ResponseError(error))
    }
}

pub async fn commit_transaction(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    commit_transaction_request: models::CommitTransactionRequest,
) -> Result<(), Error<CommitTransactionError>> {
    let uri_str = format!("transactions/commit");
    let method = reqwest::Method::POST;

    fetch_empty(
        configuration,
        method,
        prefix,
        &uri_str,
        &commit_transaction_request,
        None,
        None,
    )
    .await
}

/// Create a namespace, with an optional set of properties. The server might also add properties, such as `last_modified_time` etc.
pub async fn create_namespace(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    create_namespace_request: models::CreateNamespaceRequest,
) -> Result<models::CreateNamespaceResponse, Error<CreateNamespaceError>> {
    let uri_str = format!("namespaces",);
    let method = reqwest::Method::POST;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &create_namespace_request,
        None,
        None,
    )
    .await
}

/// Create a table or start a create transaction, like atomic CTAS.  If `stage-create` is false, the table is created immediately.  If `stage-create` is true, the table is not created, but table metadata is initialized and returned. The service should prepare as needed for a commit to the table commit endpoint to complete the create transaction. The client uses the returned metadata to begin a transaction. To commit the transaction, the client sends all create and subsequent changes to the table commit route. Changes from the table create operation include changes like AddSchemaUpdate and SetCurrentSchemaUpdate that set the initial table state.
pub async fn create_table(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    create_table_request: CreateTable,
    x_iceberg_access_delegation: Option<&str>,
) -> Result<models::LoadTableResult, Error<CreateTableError>> {
    let mut headers = HashMap::new();
    if let Some(param_value) = x_iceberg_access_delegation {
        headers.insert(
            "X-Iceberg-Access-Delegation".to_owned(),
            param_value.to_string(),
        );
    }

    let uri_str = format!(
        "namespaces/{namespace}/tables",
        namespace = crate::apis::urlencode(namespace)
    );
    let method = reqwest::Method::POST;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &create_table_request,
        Some(headers),
        None,
    )
    .await
}

/// Create a view in the given namespace.
pub async fn create_view<T: Materialization + serde::Serialize>(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    create_view_request: CreateView<T>,
) -> Result<models::LoadViewResult, Error<CreateViewError>> {
    let uri_str = format!(
        "namespaces/{namespace}/views",
        namespace = crate::apis::urlencode(namespace)
    );
    let method = reqwest::Method::POST;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &create_view_request,
        None,
        None,
    )
    .await
}

pub async fn drop_namespace(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
) -> Result<(), Error<DropNamespaceError>> {
    let uri_str = format!(
        "namespaces/{namespace}",
        namespace = crate::apis::urlencode(namespace)
    );

    let method = reqwest::Method::DELETE;

    fetch_empty(configuration, method, prefix, &uri_str, &(), None, None).await
}

/// Remove a table from the catalog
pub async fn drop_table(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    table: &str,
    purge_requested: Option<bool>,
) -> Result<(), Error<DropTableError>> {
    let mut query_params = HashMap::new();
    if let Some(purge_requested) = purge_requested {
        query_params.insert("purgeRequested".to_owned(), purge_requested.to_string());
    }

    let uri_str = format!(
        "namespaces/{namespace}/tables/{table}",
        namespace = crate::apis::urlencode(namespace),
        table = crate::apis::urlencode(table)
    );
    let method = reqwest::Method::DELETE;

    fetch_empty(
        configuration,
        method,
        prefix,
        &uri_str,
        &(),
        None,
        Some(query_params),
    )
    .await
}

/// Remove a view from the catalog
pub async fn drop_view(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    view: &str,
) -> Result<(), Error<DropViewError>> {
    let uri_str = format!(
        "namespaces/{namespace}/views/{view}",
        namespace = crate::apis::urlencode(namespace),
        view = crate::apis::urlencode(view)
    );
    let method = reqwest::Method::DELETE;

    fetch_empty(configuration, method, prefix, &uri_str, &(), None, None).await
}

/// List all namespaces at a certain level, optionally starting from a given parent namespace. If table accounting.tax.paid.info exists, using 'SELECT NAMESPACE IN accounting' would translate into `GET /namespaces?parent=accounting` and must return a namespace, [\"accounting\", \"tax\"] only. Using 'SELECT NAMESPACE IN accounting.tax' would translate into `GET /namespaces?parent=accounting%1Ftax` and must return a namespace, [\"accounting\", \"tax\", \"paid\"]. If `parent` is not provided, all top-level namespaces should be listed.
pub async fn list_namespaces(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    page_token: Option<&str>,
    page_size: Option<i32>,
    parent: Option<&str>,
) -> Result<models::ListNamespacesResponse, Error<ListNamespacesError>> {
    let mut query_params = HashMap::new();
    if let Some(page_token) = page_token {
        query_params.insert("pageToken".to_owned(), page_token.to_string());
    }
    if let Some(page_size) = page_size {
        query_params.insert("pageSize".to_owned(), page_size.to_string());
    }
    if let Some(parent) = parent {
        query_params.insert("parent".to_owned(), parent.to_string());
    }

    let uri_str = format!("namespaces",);

    let method = reqwest::Method::GET;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &(),
        None,
        Some(query_params),
    )
    .await
}

/// Return all table identifiers under this namespace
pub async fn list_tables(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    page_token: Option<&str>,
    page_size: Option<i32>,
) -> Result<models::ListTablesResponse, Error<ListTablesError>> {
    let mut query_params = HashMap::new();
    if let Some(page_token) = page_token {
        query_params.insert("pageToken".to_owned(), page_token.to_string());
    }
    if let Some(page_size) = page_size {
        query_params.insert("pageSize".to_owned(), page_size.to_string());
    }

    let uri_str = format!(
        "namespaces/{namespace}/tables",
        namespace = crate::apis::urlencode(namespace)
    );
    let method = reqwest::Method::GET;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &(),
        None,
        Some(query_params),
    )
    .await
}

/// Return all view identifiers under this namespace
pub async fn list_views(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    page_token: Option<&str>,
    page_size: Option<i32>,
) -> Result<models::ListTablesResponse, Error<ListViewsError>> {
    let mut query_params = HashMap::new();
    if let Some(page_token) = page_token {
        query_params.insert("pageToken".to_owned(), page_token.to_string());
    }
    if let Some(page_size) = page_size {
        query_params.insert("pageSize".to_owned(), page_size.to_string());
    }

    let uri_str = format!(
        "namespaces/{namespace}/views",
        namespace = crate::apis::urlencode(namespace)
    );
    let method = reqwest::Method::GET;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &(),
        None,
        Some(query_params),
    )
    .await
}

/// Return all stored metadata properties for a given namespace
pub async fn load_namespace_metadata(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
) -> Result<models::GetNamespaceResponse, Error<LoadNamespaceMetadataError>> {
    let uri_str = format!(
        "namespaces/{namespace}",
        namespace = crate::apis::urlencode(namespace)
    );
    let method = reqwest::Method::GET;

    fetch(configuration, method, prefix, &uri_str, &(), None, None).await
}

/// Load a table from the catalog.  The response contains both configuration and table metadata. The configuration, if non-empty is used as additional configuration for the table that overrides catalog configuration. For example, this configuration may change the FileIO implementation to be used for the table.  The response also contains the table's full metadata, matching the table metadata JSON file.  The catalog configuration may contain credentials that should be used for subsequent requests for the table. The configuration key \"token\" is used to pass an access token to be used as a bearer token for table requests. Otherwise, a token may be passed using a RFC 8693 token type as a configuration key. For example, \"urn:ietf:params:oauth:token-type:jwt=<JWT-token>\".
pub async fn load_table(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    table: &str,
    x_iceberg_access_delegation: Option<&str>,
    snapshots: Option<&str>,
) -> Result<models::LoadTableResult, Error<LoadTableError>> {
    let mut headers = HashMap::new();
    if let Some(param_value) = x_iceberg_access_delegation {
        headers.insert(
            "X-Iceberg-Access-Delegation".to_owned(),
            param_value.to_string(),
        );
    }
    let mut query_params = HashMap::new();
    if let Some(snapshots) = snapshots {
        query_params.insert("snapshots".to_owned(), snapshots.to_string());
    }

    let uri_str = format!(
        "namespaces/{namespace}/tables/{table}",
        namespace = crate::apis::urlencode(namespace),
        table = crate::apis::urlencode(table)
    );
    let method = reqwest::Method::GET;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &(),
        Some(headers),
        Some(query_params),
    )
    .await
}

/// Load a view from the catalog.  The response contains both configuration and view metadata. The configuration, if non-empty is used as additional configuration for the view that overrides catalog configuration.  The response also contains the view's full metadata, matching the view metadata JSON file.  The catalog configuration may contain credentials that should be used for subsequent requests for the view. The configuration key \"token\" is used to pass an access token to be used as a bearer token for view requests. Otherwise, a token may be passed using a RFC 8693 token type as a configuration key. For example, \"urn:ietf:params:oauth:token-type:jwt=<JWT-token>\".
pub async fn load_view(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    view: &str,
) -> Result<models::LoadViewResult, Error<LoadViewError>> {
    let uri_str = format!(
        "namespaces/{namespace}/views/{view}",
        namespace = crate::apis::urlencode(namespace),
        view = crate::apis::urlencode(view)
    );
    let method = reqwest::Method::GET;

    fetch(configuration, method, prefix, &uri_str, &(), None, None).await
}

/// Check if a namespace exists. The response does not contain a body.
pub async fn namespace_exists(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
) -> Result<(), Error<NamespaceExistsError>> {
    let uri_str = format!(
        "namespaces/{namespace}",
        namespace = crate::apis::urlencode(namespace)
    );

    let method = reqwest::Method::HEAD;

    fetch_empty(configuration, method, prefix, &uri_str, &(), None, None).await
}

/// Register a table using given metadata file location.
pub async fn register_table(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    register_table_request: models::RegisterTableRequest,
) -> Result<models::LoadTableResult, Error<RegisterTableError>> {
    let uri_str = format!(
        "namespaces/{namespace}/register",
        namespace = crate::apis::urlencode(namespace)
    );

    let method = reqwest::Method::POST;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &register_table_request,
        None,
        None,
    )
    .await
}

/// Rename a table from one identifier to another. It's valid to move a table across namespaces, but the server implementation is not required to support it.
pub async fn rename_table(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    rename_table_request: models::RenameTableRequest,
) -> Result<(), Error<RenameTableError>> {
    let uri_str = format!("tables/rename",);
    let method = reqwest::Method::POST;

    fetch_empty(
        configuration,
        method,
        prefix,
        &uri_str,
        &rename_table_request,
        None,
        None,
    )
    .await
}

/// Rename a view from one identifier to another. It's valid to move a view across namespaces, but the server implementation is not required to support it.
pub async fn rename_view(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    rename_table_request: models::RenameTableRequest,
) -> Result<(), Error<RenameViewError>> {
    let uri_str = format!("views/rename",);
    let method = reqwest::Method::POST;

    fetch_empty(
        configuration,
        method,
        prefix,
        &uri_str,
        &rename_table_request,
        None,
        None,
    )
    .await
}

/// Commit updates to a view.
pub async fn replace_view<T: Materialization + serde::Serialize>(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    view: &str,
    commit_view_request: models::CommitViewRequest<T>,
) -> Result<models::LoadViewResult, Error<ReplaceViewError>> {
    let uri_str = format!(
        "namespaces/{namespace}/views/{view}",
        namespace = crate::apis::urlencode(namespace),
        view = crate::apis::urlencode(view)
    );
    let method = reqwest::Method::POST;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &commit_view_request,
        None,
        None,
    )
    .await
}

pub async fn report_metrics(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    table: &str,
    report_metrics_request: models::ReportMetricsRequest,
) -> Result<(), Error<ReportMetricsError>> {
    let uri_str = format!(
        "namespaces/{namespace}/tables/{table}/metrics",
        namespace = crate::apis::urlencode(namespace),
        table = crate::apis::urlencode(table)
    );
    let method = reqwest::Method::POST;

    fetch_empty(
        configuration,
        method,
        prefix,
        &uri_str,
        &report_metrics_request,
        None,
        None,
    )
    .await
}

/// Check if a table exists within a given namespace. The response does not contain a body.
pub async fn table_exists(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    table: &str,
) -> Result<(), Error<TableExistsError>> {
    let uri_str = format!(
        "namespaces/{namespace}/tables/{table}",
        namespace = crate::apis::urlencode(namespace),
        table = crate::apis::urlencode(table)
    );

    let method = reqwest::Method::HEAD;

    fetch_empty(configuration, method, prefix, &uri_str, &(), None, None).await
}

/// Set and/or remove properties on a namespace. The request body specifies a list of properties to remove and a map of key value pairs to update. Properties that are not in the request are not modified or removed by this call. Server implementations are not required to support namespace properties.
pub async fn update_properties(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    update_namespace_properties_request: models::UpdateNamespacePropertiesRequest,
) -> Result<models::UpdateNamespacePropertiesResponse, Error<UpdatePropertiesError>> {
    let uri_str = format!(
        "namespaces/{namespace}/properties",
        namespace = crate::apis::urlencode(namespace)
    );
    let method = reqwest::Method::POST;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &update_namespace_properties_request,
        None,
        None,
    )
    .await
}

/// Commit updates to a table.  Commits have two parts, requirements and updates. Requirements are assertions that will be validated before attempting to make and commit changes. For example, `assert-ref-snapshot-id` will check that a named ref's snapshot ID has a certain value.  Updates are changes to make to table metadata. For example, after asserting that the current main ref is at the expected snapshot, a commit may add a new child snapshot and set the ref to the new snapshot id.  Create table transactions that are started by createTable with `stage-create` set to true are committed using this route. Transactions should include all changes to the table, including table initialization, like AddSchemaUpdate and SetCurrentSchemaUpdate. The `assert-create` requirement is used to ensure that the table was not created concurrently.
pub async fn update_table(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    table: &str,
    commit_table_request: models::CommitTableRequest,
) -> Result<models::CommitTableResponse, Error<UpdateTableError>> {
    let uri_str = format!(
        "namespaces/{namespace}/tables/{table}",
        namespace = crate::apis::urlencode(namespace),
        table = crate::apis::urlencode(table)
    );
    let method = reqwest::Method::POST;

    fetch(
        configuration,
        method,
        prefix,
        &uri_str,
        &commit_table_request,
        None,
        None,
    )
    .await
}

/// Check if a view exists within a given namespace. This request does not return a response body.
pub async fn view_exists(
    configuration: &configuration::Configuration,
    prefix: Option<&str>,
    namespace: &str,
    view: &str,
) -> Result<(), Error<ViewExistsError>> {
    let uri_str = format!(
        "namespaces/{namespace}/views/{view}",
        namespace = crate::apis::urlencode(namespace),
        view = crate::apis::urlencode(view)
    );

    let method = reqwest::Method::HEAD;

    fetch_empty(configuration, method, prefix, &uri_str, &(), None, None).await
}
