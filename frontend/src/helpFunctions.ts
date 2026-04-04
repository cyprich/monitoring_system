import type {Key} from "@heroui/react";

// export function getMetricsLimit(): number {
//     const default_value = 100;
//     const value = localStorage.getItem("metrics_limit")
//
//     if (value !== null) {
//         return Number(value);
//     } else {
//         return default_value;
//     }
// }
//
// export function setMetricsLimit(value: number) {
//     localStorage.setItem("metrics_limit", String(value))
// }

export function getTimeLimit(): number {
    const value = localStorage.getItem("time_limit");

    if (value !== null) {
        return Number(value);
    } else {
        return 6; // default value
    }
}

export function getResolution(): number {
    const value = localStorage.getItem("resolution");

    if (value !== null) {
        return Number(value)
    } else {
        return 120; // default value
    }
}

export function setTimeLimit(value: number) {
    localStorage.setItem("time_limit", String(value))
}

export function keysToNumber(keys: Set<Key>): number {
    const val = keys.values().next().value;
    return Number(val)
}
