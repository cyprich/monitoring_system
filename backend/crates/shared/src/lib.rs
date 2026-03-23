use dotenvy::dotenv;

pub mod structs;

// TODO temporary
pub const BASE_URL: &str = "http://localhost:5000";
const UNKNOWN: &str = "<<unknown>>";
const CONFIG_FILENAME: &str = "./collector_config.toml";

pub fn get_env(variable_name: &str) -> Option<String> {
    dotenv().ok()?;
    dotenvy::var(variable_name).ok()
}
