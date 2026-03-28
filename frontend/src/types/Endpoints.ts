// import colors from "tailwindcss/colors";

export interface Endpoint {
    id: number,
    url: string,
    // TODO
    method: string,
    expected_codes: number[]
}