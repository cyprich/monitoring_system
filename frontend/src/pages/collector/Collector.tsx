import {average_metrics, type Metrics as MetricsInterface} from "../../types/Metrics.ts";
import type {Collector, Drive, NetworkInterface} from "../../types/Collector.ts";
import axios from "axios";
import {Link, useParams} from "react-router";
import CustomSurface from "../../components/CustomSurface.tsx";
import {useEffect, useState} from "react";
import {
    ArrowShapeUpFromLine,
    Bell,
    ChartLineArrowUp,
    ChevronLeft,
    Gear,
    ShieldKeyhole
} from "@gravity-ui/icons";
import Notifications from "./Notifications.tsx";
import Endpoints from "./endpoints/Endpoints.tsx";
import Metrics from "./Metrics.tsx"
import type {EndpointResult} from "../../types/Endpoints.ts";
import type {Notification} from "../../types/Notifications.ts";
import {Settings} from "./Settings.tsx";
import {getBaseUrl, getResolution, getTimeLimit, getWebsocketBaseUrl} from "../../helpFunctions.ts";
import {SettingsTimeLimit} from "../../components/settings/SettingsTimeLimit.tsx";
import {SettingsResolution} from "../../components/settings/SettingsResolution.tsx";
import {Separator} from "@heroui/react";
import Ports from "./Ports.tsx";
import type {PortsInterface} from "../../types/PortsInterface.ts";

export interface CollectorProps {
    collector: Collector | null,
    data: MetricsInterface[]
}

export default function Collector() {
    const params = useParams();
    const id = Number(params.id || "0");

    const [collector, setCollector] = useState<Collector | null>(null)
    const [metrics, setMetrics] = useState<MetricsInterface[]>([])
    const [, setLastMetrics] = useState<MetricsInterface[]>([]);
    const [lastEndpointsResults, setLastEndpointsResults] = useState<EndpointResult[]>([])
    const [notifications, setNotifications] = useState<Notification[]>([])
    const [ports, setPorts] = useState<PortsInterface[]>([])
    // const [metricsThresholds, setMetricsThresholds] = useState<MetricsThresholdsInterface[]>([])

    const url = getBaseUrl() + `/collector/${id}`

    const TIME_LIMIT_HOURS = getTimeLimit();
    const RESOLUTION = getResolution();

    const METRICS_INTERVAL = 10; // new metrics every 10 seconds
    const TOTAL_METRICS_COUNT = (TIME_LIMIT_HOURS * 3600) / METRICS_INTERVAL; // how many metrics in total
    const VALUES_IN_WINDOW = Math.floor(TOTAL_METRICS_COUNT / RESOLUTION); // number of values for each window

    useEffect(() => {
        // collector
        axios
            .get<Collector>(url)
            .then((resp) => {
                setCollector(resp.data)
            })

        // drives
        axios.get<Drive[]>(`${url}/drives`).then((resp) => {
            setCollector(
                prev => (
                    prev ? {...prev, drives: resp.data} : prev
                )
            )
        });

        // network interfaces
        axios.get<NetworkInterface[]>(`${url}/network_interfaces`).then((resp) => {
            setCollector(
                prev => (
                    prev ? {...prev, network_interfaces: resp.data} : prev
                )
            )
        });

        // historic metrics
        axios
            .get<MetricsInterface[]>(`${url}/metrics`, {
                params: {
                    time_limit_hours: TIME_LIMIT_HOURS,
                    resolution: RESOLUTION
                }
            })
            .then((resp) => {
                setMetrics(
                    resp.data.map(m => ({
                        ...m,
                        time : new Date(m.time).toLocaleTimeString()
                    })) || []
                );
            })

        // historic notifications
        axios
            .get<Notification[]>(`${url}/notifications`)
            .then((resp) => {
                setNotifications(
                    resp.data.map(n => ({
                        ...n,
                        time: new Date(n.time).toLocaleDateString() + " " + new Date(n.time).toLocaleTimeString()
                    })) || []
                )
            })

        // last endpoints results
        axios
            .get<EndpointResult[]>(`${url}/endpoints_results/last`)
            .then((resp) => {
                setLastEndpointsResults(
                    resp.data.map(r => ({
                        ...r,
                        time: new Date(r.time).toLocaleTimeString()
                    })) || []
                )
        })

        // ports
        axios
            .get<PortsInterface[]>(`${url}/ports`)
            .then((resp) => {
                setPorts(
                    resp.data.map(p => ({
                        ...p,
                        last_update: new Date(p.last_update).toLocaleTimeString()
                    })) || []
                )
            })
    }, [TIME_LIMIT_HOURS, RESOLUTION, id, url]);

    useEffect(() => {
        const socket = new WebSocket(getWebsocketBaseUrl() + `/ws/collector/${id}`);

        socket.addEventListener("open", () => {
            console.log("Websocket opened")
        })

        socket.addEventListener("message", event => {
            const recv = JSON.parse(event.data);
            switch (recv.type) {
                case "metrics": {
                    // https://howtodoinjava.com/typescript/typescript-date-object/
                    const newData: MetricsInterface = recv.data
                    const newTime = new Date(newData.time).toLocaleTimeString()
                    setLastMetrics(prev => {
                        const newLastMetrics = [...prev, {...newData, time: newTime}]

                        if (newLastMetrics.length >= VALUES_IN_WINDOW) {
                            setMetrics(prev => [...prev, average_metrics(newLastMetrics)!].slice(-RESOLUTION))
                            return []
                        } else {
                            return newLastMetrics
                        }
                    });
                    break
                }
                case "endpoints_results": {
                    let newData: EndpointResult[] = recv.data;
                    newData = newData.map(r => ({
                        ...r,
                        time: new Date(r.time).toLocaleTimeString()
                    }))
                    setLastEndpointsResults(newData)
                    break
                }
                case "notifications": {
                    let newData: Notification[] = recv.data;
                    newData = newData.map(n => ({
                        ...n,
                        time: new Date(n.time).toLocaleDateString() + " " + new Date(n.time).toLocaleTimeString()
                    }))
                    setNotifications(prev => [...prev, ...newData])
                    break
                }
                case "ports_opened": {
                    let newData: PortsInterface[] = recv.data;
                    newData = newData.map(i => ({
                        ...i,
                        last_update: new Date(i.last_update).toLocaleTimeString()
                    }))
                    setPorts((prev) => [ ...prev, ...newData ])
                    break
                }
                case "ports_closed": {
                    const newData: PortsInterface[] = recv.data;
                    const ids = new Set(newData.map(p => p.id))
                    setPorts((prev) => prev.filter(p => !ids.has(p.id)))
                    break
                }
            }
        })

        return () => socket.close()

    }, [RESOLUTION, TIME_LIMIT_HOURS, VALUES_IN_WINDOW, id]);

    return (
        <main className={"flex flex-col gap-8"}>
            <Link to={"/"} className={"flex items-center custom-description hover:underline w-max"}>
                <ChevronLeft/>Home
            </Link>
            {
                collector && <CollectorHeader {...collector} />
            }
            <CustomSurface title={"Metrics"} id={"metrics"} icon={ <ChartLineArrowUp/> }>
                <Metrics collector={collector} data={metrics}/>
                <Separator variant={"tertiary"} className={"my-8"}/>
                <h4 className={"mt-4 pb-2"}>Settings</h4>
                <div className={"flex flex-col gap-4"}>
                    <SettingsTimeLimit/>
                    <SettingsResolution/>
                </div>
            </CustomSurface>

            <CustomSurface title={"API Endpoints"} id={"endpoints"} icon={ <ArrowShapeUpFromLine/> } >
                <Endpoints collector_id={collector?.id || 0} lastEndpointsResults={lastEndpointsResults}/>
            </CustomSurface>

            <CustomSurface title={"Listening Ports"} id={"ports"} icon={ <ShieldKeyhole/> } >
                <Ports collector_id={collector?.id || 0} ports={ports}/>
            </CustomSurface>

            <CustomSurface title={"Notifications"} id={"notifications"} icon={ <Bell/> } >
                <Notifications notifications={notifications} collector_id={id} setNotifications={setNotifications}/>
            </CustomSurface>

            {
                collector !== null &&
                <CustomSurface title={"Settings"} id={"settings"} className={"flex flex-col gap-6"} icon={ <Gear/> } >
                    <Settings collector={collector} setCollector={setCollector} />
                </CustomSurface>
            }
        </main>
    )
}

function CollectorHeader(collector: Collector) {
    const total_capacity = collector.drives?.reduce((acc, d) => acc + d.capacity_gb, 0) || 0;

    function scroll_to(value: string) {
        document.getElementById(value)?.scrollIntoView({
            behavior: "smooth",
            block: "start"
        })
    }

    return (
        <div className={"flex flex-col gap-3"}>
            <div className={"flex flex-col gap-2"}>
                <h1 className={"mb-1!"}>{collector.name}</h1>
                {
                    collector.name !== collector.host_name && <p className={"-mt-1"}>{collector.host_name}</p>
                }
                <p className={"flex items-center"}>
                    {collector.system_name}
                    <span className={"w-0.5 h-5 mx-2 bg-black/40"}/>
                    {collector.kernel_version}
                </p>
                <p>{collector.cpu_count} CPU Cores</p>
                <p>{(collector.total_memory_mb || 0) / 1000} GB RAM</p>
                <p>{(collector.total_swap_mb || 0) / 1000} GB Swap</p>
                <p>
                    {collector.drives?.length || 0} drive{collector.drives?.length !== 1 && "s"}
                    <span className={"font-extralight"}> with total capacity of </span>
                    {total_capacity}GB
                </p>
                <p>{collector.network_interfaces?.length || 0} network interface{collector.network_interfaces?.length !== 1 && "s"}</p>
            </div>
            <div className={"flex gap-2 *:size-11 *:p-2 *:rounded-lg *:border-2 *:hover:bg-zinc-200 transition-all "}>
                <ChartLineArrowUp onClick={() => scroll_to("metrics")}/>
                <ArrowShapeUpFromLine onClick={() => scroll_to("endpoints")}/>
                <ShieldKeyhole onClick={() => scroll_to("ports")}/>
                <Bell onClick={() => scroll_to("notifications")}/>
                <Gear onClick={() => scroll_to("settings")}/>
            </div>
        </div>
    )
}