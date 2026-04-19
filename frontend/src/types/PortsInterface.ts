export interface PortsInterface {
    id: number 
    collector_id: number 
    address: string 
    port: number 
    protocol: string
    last_update: string
}

export interface PortsNotificationSettings {
    id: number
    show_for_opened: boolean
    show_for_closed: boolean
}