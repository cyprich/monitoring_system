pub mod env;
mod error;
pub mod structs;

pub use error::ApiError;
pub use error::CollectorError;
pub use error::DatabaseError;

// TODO temporary
pub const BASE_URL: &str = "http://localhost:5000";
const UNKNOWN: &str = "<<unknown>>";
const CONFIG_FILENAME: &str = "./collector_config.toml";
