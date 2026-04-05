// import colors from "tailwindcss/colors";

export interface Endpoint {
    id: number,
    url: string,
    expected_codes: number[]
}

export interface EndpointResult {
    endpoint_id: number,
    time: string,
    result: boolean,
    latency_microseconds: number | null
}
