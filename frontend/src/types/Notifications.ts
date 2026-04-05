export interface Notification {
    id: number,
    collector_id: number,
    cause: string,
    description: string | undefined,
    time: string,
}