export interface LineChartData {
    title: string,
    // TODO: multiple datasets
    dataset: Dataset
}

export interface Dataset {
    name: string,
    data: Data[]
}

export interface Data {
    x: number | string
    y: number
}
