use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricsThreshold {
    pub id: i32,
    pub collector_id: i32,
    pub metric_type: String,
    pub component_name: String,
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointsThresholds {
    pub id: i32,
    pub endpoint_id: i32,
    pub value: i32,
}
