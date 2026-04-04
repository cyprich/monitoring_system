import type {Key} from "@heroui/react";

export function reload() {
    window.location.reload()
}

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
    reload()
}

export function setResolution(value: number) {
    localStorage.setItem("resolution", String(value))
    reload()
}

export function keysToNumber(keys: Set<Key>): number {
    const val = keys.values().next().value;
    return Number(val)
}


export function firstLetterUppercase(value: string): string {
    return value.charAt(0).toUpperCase() + value.slice(1)
}