// Import necessary crates and modules
extern crate lazy_static;
mod big_query;
mod model;
mod utility;
use crate::big_query::{create_schema, create_table, insert_data};
use crate::model::Person;
use crate::utility::env_vars::{env_vars, get_gcp_credentials};
use dotenv::dotenv;
use gcp_bigquery_client::model::table_data_insert_all_request::TableDataInsertAllRequest;
use gcp_bigquery_client::Client;
use log::info;
use serde_json::{json, Value};
use yup_oauth2::ServiceAccountKey;

// Define the main function to be executed asynchronously with Tokio runtime
#[tokio::main]
async fn main() {
    // Initialize logging based on environment variables
    env_logger::init();
    // Load environment variables from a .env file
    dotenv().ok();

    // Parse the environment variable containing Google Cloud service account key into a JSON value
    let gcp_sa_key: Value =
        serde_json::from_str(&env_vars()).expect(" unable to convert string to json");

    // Create a new TableDataInsertAllRequest
    let insert_request = TableDataInsertAllRequest::new();

    // Get Google Cloud service account credentials from the parsed JSON value
    let gcs_key: ServiceAccountKey = get_gcp_credentials(gcp_sa_key);

    // Create a BigQuery client using the obtained credentials
    let big_query_client = Client::from_service_account_key(gcs_key, false)
        .await
        .expect("unable to create BigQuery Client");

    // Create a table in BigQuery using the client and schema
    let (_table, table_is_created) = create_table(big_query_client.clone(), create_schema()).await;

    // Create an instance of Person struct with sample data
    let person_data = Person {
        name: "Sheshnath".to_string(),
        age: 24,
        gender: "Male".to_string(),
    };

    // Insert the sample data into the table if the table was successfully created
    if table_is_created {
        info!(
            "{:?}",
            insert_data(person_data, big_query_client.clone(), insert_request).await
        );
    }
}
