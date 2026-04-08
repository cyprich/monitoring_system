use dotenvy::dotenv;

pub fn get(variable_name: &str) -> Result<String, crate::Error> {
    let _ = dotenv();

    Ok(dotenvy::var(variable_name)?)
}

pub fn base_url() -> Result<String, crate::Error> {
    let addr = get("API_ADDRESS")?;
    let port = get("API_PORT")?;
    Ok(format!("{addr}:{port}/api/v1"))
}
