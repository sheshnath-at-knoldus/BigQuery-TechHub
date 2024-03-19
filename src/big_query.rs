// Import necessary crates and modules
use crate::model::Person;
use crate::utility::config::CONFIG;
use gcp_bigquery_client::error::BQError;
use gcp_bigquery_client::model::dataset::Dataset;
use gcp_bigquery_client::model::field_type::FieldType::{Integer, Numeric, String};
use gcp_bigquery_client::model::table::Table;
use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;
use gcp_bigquery_client::model::table_data_insert_all_response::TableDataInsertAllResponse;
use gcp_bigquery_client::model::table_data_insert_all_response_insert_errors::TableDataInsertAllResponseInsertErrors;
use gcp_bigquery_client::model::table_field_schema::TableFieldSchema;
use gcp_bigquery_client::model::table_schema::TableSchema;
use gcp_bigquery_client::Client;
use log::info;

// Function to create a data set in BigQuery
pub async fn create_data_set(bgq_client: Client) -> (Dataset, bool) {
    // Check if the dataset already exists
    let dataset_exists = bgq_client
        .dataset()
        .get(&CONFIG.project_id, &CONFIG.data_set_id)
        .await
        .is_ok();
    let mut check_data_set = false;
    let mut data_set = Default::default();
    if !dataset_exists {
        // Create a new data set
        data_set = bgq_client
            .dataset()
            .create(
                Dataset::new(&CONFIG.project_id, &CONFIG.data_set_id)
                    .location("US")
                    .friendly_name("Just Demo DataSet")
                    .label("owner", "me"),
            )
            .await
            .expect("unable to create dataset");
        check_data_set = true;
    } else {
        check_data_set = true;
        info!("DataSet already exists");
    }
    (data_set, check_data_set) // Return the created data set and a flag indicating whether it was newly created or not
}

// Function to create a table in BigQuery
pub async fn create_table(
    bgq_client: Client,
    table_schema: Vec<TableFieldSchema>,
) -> (Table, bool) {
    let (data_set, _check_data_set) = create_data_set(bgq_client.clone()).await;
    // Check if the table already exists
    let check_table = bgq_client
        .table()
        .get(
            &CONFIG.project_id,
            &CONFIG.data_set_id,
            &CONFIG.table_id,
            None,
        )
        .await
        .is_ok();
    let mut table_is_created = false;
    let mut table = Default::default();
    if !check_table {
        // Create a new table
        table = data_set
            .create_table(
                &bgq_client,
                Table::from_dataset(&data_set, &CONFIG.table_id, TableSchema::new(table_schema))
                    .friendly_name("Demo table")
                    .description("Description of your table")
                    .label("owner", "me"),
            )
            .await
            .expect("unable to create  Table");
        table_is_created = true;
    } else {
        table_is_created = true;
        info!("Table already exists");
    }
    (table, table_is_created) // Return the created table and a flag indicating whether it was newly created or not
}

// Function to create a table schema
pub fn create_schema() -> Vec<TableFieldSchema> {
    // Define the table fields and their properties
    vec![
        TableFieldSchema {
            name: "Name".to_string(),
            r#type: String,
            mode: None,
            fields: None,
            description: None,
            categories: None,
            policy_tags: None,
        },
        TableFieldSchema {
            name: "Age".to_string(),
            r#type: Integer,
            mode: None,
            fields: None,
            description: None,
            categories: None,
            policy_tags: None,
        },
        TableFieldSchema {
            name: "Gender".to_string(),
            r#type: String,
            mode: None,
            fields: None,
            description: None,
            categories: None,
            policy_tags: None,
        },
    ]
}

// Function to insert data into a BigQuery table
pub async fn insert_data(
    data: Person,
    big_query_client: Client,
    mut insert_request: TableDataInsertAllRequest,
) -> Option<Vec<TableDataInsertAllResponseInsertErrors>> {
    // Add data to the insert request
    insert_request
        .add_row(None, data)
        .expect("unable to insert ");
    // Insert data into the table
    let response = big_query_client
        .tabledata()
        .insert_all(
            &CONFIG.project_id,
            &CONFIG.data_set_id,
            &CONFIG.table_id,
            insert_request,
        )
        .await
        .expect("unable to insert data ");
    response.insert_errors // Return any insert errors, if present
}
