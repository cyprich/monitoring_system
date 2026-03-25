export interface Collector {
    id: number,
    name: string,
    system_name: string | null,
    host_name: string | null,
    kernel_version: string | null,
    total_memory_mb: number | null,
    total_swap_mb: number | null,
    cpu_count: number | null
}