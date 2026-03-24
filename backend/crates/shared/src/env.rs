use dotenvy::dotenv;

use crate::error::EnvError;

pub fn get(variable_name: &str) -> Result<String, EnvError> {
    dotenv()?;
    Ok(dotenvy::var(variable_name)?)
}
