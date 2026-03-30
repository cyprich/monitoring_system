// import colors from "tailwindcss/colors";

export interface Endpoint {
    id: number,
    url: string,
    // TODO
    method: string,
    expected_codes: number[]
}

export interface EndpointResult {
    endpoint_id: number,
    timestamp: string,
    result: boolean,
    latency_microseconds: number | null
}