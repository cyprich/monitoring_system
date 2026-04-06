import type {Key} from "@heroui/react";

// general functions
export function reload() {
    window.location.reload()
}

// localStorage access
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

export function getBaseUrl(): string {
    return localStorage.getItem("base_url") || "http://localhost:5000/api/v1"
}

export function setBaseUrl(value: string) {
    localStorage.setItem("base_url", value + "/api/v1")
}

// formatting and more
export function keysToNumber(keys: Set<Key>): number {
    const val = keys.values().next().value;
    return Number(val)
}

export function firstLetterUppercase(value: string): string {
    return value.charAt(0).toUpperCase() + value.slice(1)
}