export function getMetricsLimit(): number {
    const default_value = 100;
    const value = localStorage.getItem("metrics_limit")

    if (value !== null) {
        return Number(value);
    } else {
        return default_value;
    }
}

export function setMetricsLimit(value: number) {
    localStorage.setItem("metrics_limit", String(value))
}