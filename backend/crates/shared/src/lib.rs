use dotenvy::dotenv;

pub mod structs;

pub fn get_env(variable_name: &str) -> Option<String> {
    dotenv().ok()?;
    dotenvy::var(variable_name).ok()
}
