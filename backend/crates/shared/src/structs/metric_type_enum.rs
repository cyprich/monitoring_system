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
