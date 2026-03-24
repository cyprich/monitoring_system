use dotenvy::dotenv;

pub fn get(variable_name: &str) -> Result<String, crate::Error> {
    dotenv()?;
    Ok(dotenvy::var(variable_name)?)
}

pub fn api_address() -> Result<String, crate::Error> {
    dotenv()?;
    let addr = get("API_ADDRESS")?;
    let port = get("API_PORT")?;
    Ok(format!("{addr}:{port}"))
}
