// TODO shouldt this be named Metrics?
export interface Metrics {
    hostname: string,
    timestamp: Date,
    used_memory_mb: number,
    used_swap_mb: number,
    cpu_usage: number,
    drives: DriveMetrics[],
    network_interfaces: NetworkInterfaceMetrics[],
}

export interface DriveMetrics {
    mountpoint: string,
    used_space_gb: number,
}

export interface NetworkInterfaceMetrics {
    name: string,
    upload_kb: number,
    download_kb: number,
}
