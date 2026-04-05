export interface EndpointsThresholdsInterface {
    threshold_id: number
    endpoint_id: number
    collector_id: number
    threshold_value: number
    url: string
    expected_codes: number[]
    count: number
}