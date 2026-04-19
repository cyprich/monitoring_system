pub mod enums;
pub mod env;
pub mod structs;
pub mod traits;

pub use enums::error::Error;

// TODO temporary
const UNKNOWN: &str = "<<unknown>>";
const CONFIG_FILENAME: &str = "./collector_config.toml";
