export interface WebsocketData {
    hostname: string,
    timestamp: Date,
    used_memory_mb: number,
    cpu_usage: number,
    disks: Disk[],
    networks: NetworkInterface[],
}

export interface Disk {
    mountpoint: string,
    available_space_mb: number,
}

export interface NetworkInterface {
    name: string,
    upload_mb: number,
    download_mb: number,
}
