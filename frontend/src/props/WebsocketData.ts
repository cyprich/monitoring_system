export interface WebsocketData {
    system_name: string,
    host_name: string,
    kernel_version: string,
    total_mem: number,
    used_mem: number,
    cpu_count: number,
    cpu_usage: number[],
    disks: Disk[],
    networks: NetworkInterface[],
}

export interface Disk {
    name: string,
    mountpoint: string,
    filesystem: string,
    total_space: number,
    available_space: number,
}

export interface NetworkInterface {
    name: string,
    mac: string,
    upload: number,
    download: number,
    total_upload: number,
    total_download: number,
}