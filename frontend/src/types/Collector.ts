export interface Collector {
    id: number,
    name: string,
    system_name: string | null,
    host_name: string | null,
    kernel_version: string | null,
    total_memory_mb: number | null,
    total_swap_mb: number | null,
    cpu_count: number | null,
    drives: Drive[] | null,
    network_interfaces: NetworkInterface[] | null
}

export interface Drive {
    mountpoint: string,
    capacity_gb: number,
    file_system: string
}

export interface NetworkInterface {
    name: string,
    mac: string
}