// Import necessary crates
use hocon::HoconLoader;
use lazy_static::lazy_static;
use serde::Deserialize;

// Define a structure to hold configuration data
#[derive(Deserialize, Debug)]
pub struct Config {
    pub project_id: String,
    pub data_set_id: String,
    pub table_id: String,
}

// Define a lazy static variable to hold the configuration data
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

// Function to load configuration data
fn get_config() -> Config {
    // Load configuration data from a HOCON file
    let config: Config = HoconLoader::new()
        .load_file("src/resources/application.config") // Load the configuration file
        .expect("unable to load config file") // Handle errors if unable to load the config file
        .resolve() // Resolve any substitutions or references in the configuration
        .expect("config deserialize error"); // Handle errors if unable to deserialize the config
    config // Return the loaded configuration
}
