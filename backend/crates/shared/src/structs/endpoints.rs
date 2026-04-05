use std::{collections::HashSet, time::Instant};

use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::structs::db::EndpointsTable;

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Endpoint {
    pub id: i32,
    pub url: String,
    pub expected_codes: HashSet<u16>,
}

// TODO every request has it's own client - not good
impl Endpoint {
    pub fn new(id: i32, url: &str, expected_codes: HashSet<u16>) -> Self {
        Self {
            id,
            url: url.to_string(),
            expected_codes,
        }
    }

    pub async fn send(&self, client: &reqwest::Client) -> Result<EndpointResult, crate::Error> {
        let url = &self.url;

        let latency = Instant::now();
        let resp = client.get(url).send().await;
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
            time: Utc::now(),
            result: is_success,
            latency_microseconds: Some(latency as i64),
        };

        Ok(result)
    }
}

impl From<EndpointsTable> for Endpoint {
    fn from(value: EndpointsTable) -> Self {
        let codes = value.expected_codes.iter().map(|c| *c as u16);
        Self::new(value.id, &value.url, HashSet::from_iter(codes))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow, utoipa::ToSchema)]
pub struct EndpointResult {
    pub endpoint_id: i32,
    pub time: chrono::DateTime<chrono::Utc>,
    pub result: bool,
    pub latency_microseconds: Option<i64>,
}
