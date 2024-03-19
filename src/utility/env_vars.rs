use dotenv::dotenv;
use std::env;
use serde_json::Value;
use yup_oauth2::ServiceAccountKey;

// Function to load environment variables
pub fn env_vars() -> String {
    dotenv().ok();
    env::var("GCS_JSON_CREDENTIALS").expect("unable to get gcp credentials")
}

// Function to parse Google Cloud service account key from JSON value
pub fn get_gcp_credentials(gcp_sa_key: Value) -> ServiceAccountKey {
    ServiceAccountKey {
        key_type: Some(gcp_sa_key["type"].as_str().unwrap().to_string()),
        project_id: Some(gcp_sa_key["project_id"].as_str().unwrap().to_string()),
        private_key_id: Some(gcp_sa_key["private_key_id"].as_str().unwrap().to_string()),
        private_key: gcp_sa_key["private_key"].as_str().unwrap().to_string(),
        client_email: gcp_sa_key["client_email"].as_str().unwrap().to_string(),
        client_id: Some(gcp_sa_key["client_id"].as_str().unwrap().to_string()),
        auth_uri: Some(gcp_sa_key["auth_uri"].as_str().unwrap().to_string()),
        token_uri: gcp_sa_key["token_uri"].as_str().unwrap().to_string(),
        auth_provider_x509_cert_url: Some(
            gcp_sa_key["auth_provider_x509_cert_url"]
                .as_str()
                .unwrap()
                .to_string(),
        ),
        client_x509_cert_url: Some(
            gcp_sa_key["client_x509_cert_url"]
                .as_str()
                .unwrap()
                .to_string(),
        ),
    }
}