export interface Metrics {
    collector_id: string,
    time: string,
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

export function average_metrics(values: Metrics[]): Metrics | null {
    if (values.length === 0) {
        return null
    }

    const len = values.length;

    let cpu_usage = 0;
    let used_memory_mb = 0;
    let used_swap_mb = 0;

    const drives_map = new Map<string, {used_space_gb: number}>()
    const network_map = new Map<string, {upload_kb: number, download_kb: number}>()

    for (const m of values) {
        cpu_usage += m.cpu_usage
        used_memory_mb += m.used_memory_mb
        used_swap_mb += m.used_swap_mb

        for (const d of m.drives) {
            const val = drives_map.get(d.mountpoint) || { used_space_gb: 0 }
            val.used_space_gb += d.used_space_gb
            drives_map.set(d.mountpoint, val)
        }

        for (const n of m.network_interfaces) {
            const val = network_map.get(n.name) || {upload_kb: 0, download_kb: 0}
            val.upload_kb += n.upload_kb
            val.download_kb += n.download_kb
            network_map.set(n.name, val)
        }
    }

    const drives: DriveMetrics[] = [];
    const network_interfaces: NetworkInterfaceMetrics[] = [];

    for (const [mountpoint, values] of drives_map.entries()) {
        const val: DriveMetrics = {
            mountpoint,
            used_space_gb: values.used_space_gb / len
        }

        drives.push(val)
    }

    for (const [name, values] of network_map.entries()) {
        const val: NetworkInterfaceMetrics = {
            name,
            download_kb: values.download_kb / len,
            upload_kb: values.upload_kb / len
        }

        network_interfaces.push(val)
    }


    return {
        collector_id: values[0].collector_id,
        time: values[0].time,
        cpu_usage: cpu_usage / len,
        used_memory_mb: used_memory_mb / len,
        used_swap_mb: used_swap_mb / len,
        drives,
        network_interfaces,
    }
}

