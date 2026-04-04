use serde::{Deserialize, Serialize};

use crate::structs::db::NotificationInsert;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: i32,
    pub collector_id: i32,
    pub metric_type: String,
    pub component_name: String,
    pub threshold_value: f64,
    pub measured_values: Vec<f64>,
    pub time: chrono::DateTime<chrono::Utc>,
}

impl Notification {
    pub fn from_notification_insert(value: NotificationInsert, id: i32) -> Self {
        Self {
            id,
            collector_id: value.collector_id,
            metric_type: value.metric_type,
            component_name: value.component_name,
            threshold_value: value.threshold_value,
            measured_values: value.measured_values,
            time: value.time,
        }
    }
}
