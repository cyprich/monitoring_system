use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "metric_type", rename_all = "snake_case")]
pub enum MetricType {
    CpuUsage,
    UsedMemoryMb,
}

impl Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricType::CpuUsage => write!(f, "cpu_usage"),
            MetricType::UsedMemoryMb => write!(f, "used_memory_mb"),
        }
    }
}
