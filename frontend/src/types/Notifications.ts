export interface Notification {
    id: number,
    collector_id: number,
    metric_type: string,
    component_name: string,
    threshold_value: number,
    measured_values: number[],
    time: string,
}