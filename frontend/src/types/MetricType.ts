export type MetricType =
    "cpu_usage"
    | "used_memory_mb"
    | "used_swap_mb"
    | "drive_used_space"
    | "network_upload"
    | "network_download"


export function prettyMetricType(value: MetricType): string {
    switch (value) {
        case "cpu_usage":
            return "CPU Usage (%)"
        case "used_memory_mb":
            return "Used Memory (MB)"
        case "used_swap_mb":
            return "Used Swap (MB)"
        case "drive_used_space":
            return "Drive Used Space (GB)"
        case "network_upload":
            return "Network Upload (kB)"
        case "network_download":
            return "Network Download (kB)"
    }
}

export function metricTypeUnit(value: MetricType): string {
    switch (value) {
        case "cpu_usage":
            return "%"
        case "used_memory_mb":
        case "used_swap_mb":
            return "MB"
        case "drive_used_space":
            return "GB"
        case "network_upload":
        case "network_download":
            return "kB"
    }

}