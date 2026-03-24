mod error;
pub use error::Error;

pub mod env;
pub mod structs;

// TODO temporary
const UNKNOWN: &str = "<<unknown>>";
const CONFIG_FILENAME: &str = "./collector_config.toml";
