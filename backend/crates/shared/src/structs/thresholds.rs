use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Threshold {
    pub id: i32,
    pub collector_id: i32,
    pub component_name: String,
    pub metric_type: String,
    pub value: f64,
}
