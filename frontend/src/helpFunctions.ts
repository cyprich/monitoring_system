import type {Key} from "@heroui/react";

// general functions
export function reload() {
    window.location.reload()
}

// localStorage access
// time limit
export function getTimeLimit(): number {
    const value = localStorage.getItem("time_limit");

    if (value !== null) {
        return Number(value);
    } else {
        return 6; // default value
    }
}

export function setTimeLimit(value: number) {
    localStorage.setItem("time_limit", String(value))
    reload()
}

// resolution
export function getResolution(): number {
    const value = localStorage.getItem("resolution");

    if (value !== null) {
        return Number(value)
    } else {
        return 120; // default value
    }
}

export function setResolution(value: number) {
    localStorage.setItem("resolution", String(value))
    reload()
}

// base url
export function getBaseUrl(): string {
    // return getBaseUrlWithoutExtension() + "/api/v1"
    // return "frontend/api/v1"

    // const x = localStorage.getItem("base_url")!;
    // console.log("URL", x); 
    // return x

    return "/api/v1"
}

export function setBaseUrl(value: string) {
    localStorage.setItem("base_url", value)
}

// websocket base url
export function getWebsocketBaseUrl(): string {
    // return localStorage.getItem("ws_base_url")!
    return "/api/v1"
}

export function setWebsocketBaseUrl(value: string) {
    localStorage.setItem("ws_base_url", value)
}

// export function getWebsocketBaseUrlWithoutExtension(): string {
//     return (localStorage.getItem("ws_base_url") || "ws://localhost:5000")
// }


// formatting and more
export function keysToNumber(keys: Set<Key>): number {
    const val = keys.values().next().value;
    return Number(val)
}

export function firstLetterUppercase(value: string): string {
    return value.charAt(0).toUpperCase() + value.slice(1)
}