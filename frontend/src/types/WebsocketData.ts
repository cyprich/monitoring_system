export interface WebsocketData {
    hostname: string,
    timestamp: Date,
    used_mem: number,
    cpu_usage: number,
    disks: Disk[],
    networks: NetworkInterface[],
}

export interface Disk {
    mountpoint: string,
    available_space: number,
}

export interface NetworkInterface {
    name: string,
    upload: number,
    download: number,
}
