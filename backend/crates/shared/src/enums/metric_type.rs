use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MetricType {
    CpuUsage,
    UsedMemoryMb,
    UsedSwapMb,
    DriveUsedSpace,
    NetworkDownload,
    NetworkUpload,
}

impl MetricType {
    pub fn to_string_pretty(&self) -> Option<String> {
        let val = match self {
            Self::CpuUsage => "CPU Usage (%)",
            Self::UsedMemoryMb => "Used Memory (MB)",
            Self::UsedSwapMb => "Used Swap (MB)",
            Self::DriveUsedSpace => "Used Drive Space (GB)",
            Self::NetworkDownload => "Network Download (KB)",
            Self::NetworkUpload => "Network Upload (KB)",
        };

        String::from_str(val).ok()
    }

    pub fn unit(&self) -> Option<String> {
        let val = match &self {
            MetricType::CpuUsage => "%",
            MetricType::UsedMemoryMb | MetricType::UsedSwapMb => "MB",
            MetricType::DriveUsedSpace => "GB",
            MetricType::NetworkDownload | MetricType::NetworkUpload => "kB",
        };

        String::from_str(val).ok()
    }
}

impl FromStr for MetricType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cpu_usage" => Ok(Self::CpuUsage),
            "used_memory_mb" => Ok(Self::UsedMemoryMb),
            "used_swap_mb" => Ok(Self::UsedSwapMb),
            "drive_used_space" => Ok(Self::DriveUsedSpace),
            "network_upload" => Ok(Self::NetworkUpload),
            "network_download" => Ok(Self::NetworkDownload),
            _ => Err(s.to_string()),
        }
    }
}

impl Display for MetricType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            MetricType::CpuUsage => "cpu_usage",
            MetricType::UsedMemoryMb => "used_memory_mb",
            MetricType::UsedSwapMb => "used_swap_mb",
            MetricType::DriveUsedSpace => "drive_used_space",
            MetricType::NetworkDownload => "network_download",
            MetricType::NetworkUpload => "network_upload",
        };

        write!(f, "{}", value)
    }
}
