use std::{collections::HashSet, time::Instant};

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::structs::db::EndpointTable;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum RequestMethod {
    #[default]
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Endpoint {
    pub id: i32,
    pub url: String,
    pub method: RequestMethod,
    pub expected_codes: HashSet<u16>,
}

// TODO every request has it's own client - not good
impl Endpoint {
    pub fn new(id: i32, url: &str, method: &RequestMethod, expected_codes: HashSet<u16>) -> Self {
        Self {
            id,
            url: url.to_string(),
            method: method.clone(),
            expected_codes,
        }
    }

    pub async fn send(&self, client: &reqwest::Client) -> Result<EndpointResult, crate::Error> {
        let url = &self.url;

        let latency = Instant::now();
        let req = match self.method {
            RequestMethod::Get => client.get(url),
            RequestMethod::Post => client.post(url),
            RequestMethod::Put => client.put(url),
            RequestMethod::Patch => client.patch(url),
            RequestMethod::Delete => client.delete(url),
        };

        let resp = req.send().await;
        let latency = latency.elapsed().as_micros();

        let is_success = match resp {
            Ok(val) => self.expected_codes.contains(&val.status().as_u16()),
            Err(val) => {
                eprintln!("Error reaching endpoint: {}", val);
                false
            }
        };

        let result = EndpointResult {
            endpoint_id: self.id,
            timestamp: chrono::Local::now().naive_local(),
            result: is_success,
            latency_microseconds: Some(latency as i64),
        };

        Ok(result)
    }
}

impl From<EndpointTable> for Endpoint {
    fn from(value: EndpointTable) -> Self {
        let codes = value.expected_codes.iter().map(|c| *c as u16);

        Self::new(
            value.id,
            &value.url,
            &RequestMethod::Get,
            HashSet::from_iter(codes),
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointResult {
    pub endpoint_id: i32,
    pub timestamp: NaiveDateTime,
    pub result: bool,
    pub latency_microseconds: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
// used when inserting new endpoint to database
pub struct EndpointInsert {
    pub url: String,
    pub method: RequestMethod,
    pub expected_codes: HashSet<u16>,
}
