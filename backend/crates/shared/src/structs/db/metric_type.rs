use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(sqlx::Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "metric_type", rename_all = "snake_case")]
pub enum MetricType {
    CpuUsageGlobal,
    CpuUsageCores,
    UsedMemoryMb,
    UsedSwapMb,
    DriveUsedSpace,
    NetworkDownload,
    NetworkUpload,
}

impl Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            MetricType::CpuUsageGlobal => "cpu_usage_global",
            MetricType::CpuUsageCores => "cpu_usage_cores",
            MetricType::UsedMemoryMb => "used_memory_mb",
            MetricType::UsedSwapMb => "used_swap_mb",
            MetricType::DriveUsedSpace => "drive_used_space",
            MetricType::NetworkDownload => "network_download",
            MetricType::NetworkUpload => "network_upload",
        };

        write!(f, "{}", val)
    }
}
