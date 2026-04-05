use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct MetricsThreshold {
    pub id: i32,
    pub collector_id: i32,
    pub metric_type: String,
    pub component_name: String,
    pub value: f64,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EndpointsThreshold {
    pub id: i32,
    pub endpoint_id: i32,
    pub count: i32,
}
