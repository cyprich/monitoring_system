mod error;
pub use error::Error;

pub mod enums;
pub mod env;
pub mod structs;
pub mod traits;

// TODO temporary
const UNKNOWN: &str = "<<unknown>>";
const CONFIG_FILENAME: &str = "./collector_config.toml";
