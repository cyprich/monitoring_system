use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectorTable {
    pub id: i32,
    pub name: String,
    pub system_name: String,
    pub host_name: String,
    pub kernel_version: String,
    pub total_memory_mb: i32,
    pub total_swap_mb: i32,
    pub cpu_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriveTable {
    pub mountpoint: String,
    pub collector_id: i32,
    pub capacity_gb: i32,
    pub file_system: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NetworkInterfaceTable {
    pub name: String,
    pub mac: String,
    pub collector_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetricTypeTable {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct MetricsTable {
    pub time: chrono::DateTime<chrono::Utc>,
    pub value: f64,
    pub metric_type: String,
    pub collector_id: i32,
    pub component_name: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EndpointsTable {
    pub id: i32,
    pub collector_id: i32,
    pub url: String,
    pub expected_codes: Vec<i32>,
}

// it has the same fields, only here for compatibility
pub type EndpointsResultsTable = crate::structs::endpoints::EndpointResult;
pub type NotificationsTable = crate::structs::notifications::Notification;
pub type MetricsThresholdsTable = crate::structs::thresholds::MetricsThreshold;
pub type EndpointThresholdsTable = crate::structs::thresholds::EndpointsThreshold;

// TODO do i really need these? cant i just ignore the ID on insert?
// used when inserting new values to database, when ID is not known yet
#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointInsert {
    pub url: String,
    pub expected_codes: HashSet<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationInsert {
    pub collector_id: i32,
    pub metric_type: String,
    pub component_name: String,
    pub threshold_value: f64,
    pub measured_values: Vec<f64>,
    pub time: chrono::DateTime<chrono::Utc>,
}

// joined tables
// tendpoints_thresholds joined with endpoints
#[derive(Debug, Serialize, Deserialize)]
pub struct EndpointsThresholdsJoin {
    pub threshold_id: i32,
    pub endpoint_id: i32,
    pub collector_id: i32,
    pub threshold_value: i32,
    pub url: String,
    pub expected_codes: Vec<i32>,
}
