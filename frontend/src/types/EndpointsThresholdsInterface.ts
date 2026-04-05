import type {Endpoint} from "./Endpoints.ts";

export interface EndpointsThresholdsInterface {
    id: number
    endpoint: Endpoint
    value: number
}