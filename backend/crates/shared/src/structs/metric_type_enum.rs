use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum MetricTypeEnum {
    CpuUsage,
    UsedMemoryMb,
    UsedSwapMb,
    DriveUsedSpace,
    NetworkDownload,
    NetworkUpload,
}

impl MetricTypeEnum {
    pub fn to_string_pretty(self) -> Option<String> {
        let val = match self {
            Self::CpuUsage => "CPU Usage",
            Self::UsedMemoryMb => "Used Memory",
            Self::UsedSwapMb => "Used Swap",
            Self::DriveUsedSpace => "Used Drive Space",
            Self::NetworkDownload => "Network Download",
            Self::NetworkUpload => "Network Upload",
        };

        String::from_str(val).ok()
    }
}

impl FromStr for MetricTypeEnum {
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

impl Display for MetricTypeEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            MetricTypeEnum::CpuUsage => "cpu_usage",
            MetricTypeEnum::UsedMemoryMb => "used_memory_mb",
            MetricTypeEnum::UsedSwapMb => "used_swap_mb",
            MetricTypeEnum::DriveUsedSpace => "drive_used_space",
            MetricTypeEnum::NetworkDownload => "network_download",
            MetricTypeEnum::NetworkUpload => "network_upload",
        };

        write!(f, "{}", value)
    }
}
