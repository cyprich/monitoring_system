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

export interface CollectorProps {
    collector: Collector | null,
    data: MetricsInterface[]
}

export default function Collector() {
    const params = useParams();
    const id = Number(params.id || "0");

    const [collector, setCollector] = useState<Collector | null>(null)
    const [metrics, setMetrics] = useState<MetricsInterface[]>([])
    const [lastMetrics, setLastMetrics] = useState<MetricsInterface[]>([]);
    const [lastEndpointsResults, setLastEndpointsResults] = useState<EndpointResult[]>([])
    const [notifications, setNotifications] = useState<Notification[]>([])

    // TODO link
    const url = `http://localhost:5000/collector/${id}`;

    // TODO
    const TIME_LIMIT_HOURS = 2;
    const RESOLUTION = 20;

    const METRICS_INTERVAL = 5; // new metrics every 5 seconds; TODO
    const TOTAL_METRICS_COUNT = (TIME_LIMIT_HOURS * 3600) / METRICS_INTERVAL; // how many metrics in total
    const VALUES_IN_WINDOW = Math.floor(TOTAL_METRICS_COUNT / RESOLUTION); // number of values for each window

    // TODO move the corresponding useEffect get to it's component
    // TODO not sure about the websocket tho, it would be nice to have just one

    useEffect(() => {
        // collector
        axios
            .get(url)
            .then((resp) => {
                // TODO check response code
                setCollector(resp.data)
            })

        // drives
        axios.get(`${url}/drives`).then((resp) => {
            const drives: Drive[] = (resp.data as any[]).map(
                (d: Drive) => ({
                    mountpoint: d.mountpoint,
                    capacity_gb: d.capacity_gb,
                    file_system: d.file_system
                })
            )
            setCollector(
                (old) => (
                    old ? {...old, drives: drives} : old
                )
            )
        });

        // network interfaces
        axios.get(`${url}/network_interfaces`).then((resp) => {
            const network_interfaces: NetworkInterface[] = (
                resp.data as any[]).map((n: NetworkInterface) => ({
                    name: n.name, mac: n.mac
                })
            )
            setCollector(
                (old) => (
                    old ? {...old, network_interfaces: network_interfaces} : old
                )
            )
        });

        // historic metrics
        axios
            .get(`${url}/metrics`, {
                params: {
                    time_limit_hours: TIME_LIMIT_HOURS,
                    resolution: RESOLUTION
                }
            })
            .then((resp) => {
                const data: MetricsInterface[] = resp.data.map((i: MetricsInterface) => (
                    {
                        ...i,
                        time: new Date(i.time).toLocaleTimeString()
                    }
                ))
                console.log(data.length)
                setMetrics(data);
            })

        // historic notifications
        axios
            .get(`${url}/notifications`)
            .then((resp) => {
                let newData: Notification[] = resp.data;
                newData = newData.map((n) => ({
                    ...n,
                    time: new Date(n.time).toLocaleTimeString()
                }))
                setNotifications(newData)
            })

        // last endpoints results
        axios
            .get(`${url}/endpoints_results/last`)
            .then((resp) => {
                const data: EndpointResult[] = resp.data.map((r: EndpointResult) => {
                    return {
                        ...r,
                        time: new Date(r.time).toLocaleTimeString()
                    }
                })
                setLastEndpointsResults(data)
        })
    }, [TIME_LIMIT_HOURS, RESOLUTION, id, url]);

    useEffect(() => {
        const socket = new WebSocket(`ws://localhost:5000/ws/collector/${id}`);

        socket.addEventListener("open", () => {
            console.log("Websocket opened")
        })

        socket.addEventListener("message", (event) => {
            const recv = JSON.parse(event.data);
            if (recv.type === "metrics") {
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
            } else if (recv.type === "endpoints_results") {
                let newData: EndpointResult[] = recv.data;
                newData = newData.map((r) => ({
                    ...r,
                    time: new Date(r.time).toLocaleTimeString()
                }))
                setLastEndpointsResults(newData)
            } else if (recv.type === 'notifications') {
                let newData: Notification[] = recv.data;
                newData = newData.map((n) => ({
                    ...n,
                    time: new Date(n.time).toLocaleTimeString()
                }))
                setNotifications((prev) => [...prev, ...newData])
            }
        })

        return () => socket.close()

    }, [TIME_LIMIT_HOURS, id]);

    return (
        <main className={"flex flex-col gap-4"}>
            <Link to={"/"} className={"flex items-center custom-description hover:underline w-max"}>
                <ChevronLeft/>Home
            </Link>
            {
                collector && <CollectorHeader {...collector} />
            }
            <CustomSurface title={"Metrics"} icon={ <ChartLineArrowUp/> }>
                <Metrics collector={collector} data={metrics}/>
            </CustomSurface>

            <CustomSurface title={"API Endpoints"} icon={ <ArrowShapeUpFromLine/> } >
                <Endpoints collectorId={collector?.id || 0} lastEndpointsResults={lastEndpointsResults}/>
            </CustomSurface>

            <CustomSurface title={"Security stuff?"} icon={ <ShieldKeyhole/> } >
                <p className={"custom-description"}>//TODO</p>
            </CustomSurface>

            <CustomSurface title={"Notifications"} icon={ <Bell/> } >
                <Notifications notifications={notifications} collector_id={id} setNotifications={setNotifications}/>
            </CustomSurface>

            {
                collector !== null &&
                <CustomSurface title={"Settings"} className={"flex flex-col gap-6"} icon={ <Gear/> } >
                    <Settings collector={collector} setCollector={setCollector} />
                </CustomSurface>
            }
        </main>
    )
}

function CollectorHeader(collector: Collector) {
    const total_capacity = collector.drives?.reduce((acc, d) => acc + d.capacity_gb, 0) || 0;

    // TODO hidden drives/networks

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
                    {collector.drives?.length || 0} drives
                    <span className={"font-extralight"}> with total capacity of </span>
                    {total_capacity}GB
                </p>
                <p>{collector.network_interfaces?.length || 0} network interfaces</p>
            </div>
            {/* TODO */}
            <div className={"flex gap-2 *:size-11 *:p-2 *:rounded-lg *:border-2 *:hover:bg-zinc-200 transition-all "}>
                <ChartLineArrowUp/>
                <ArrowShapeUpFromLine/>
                <ShieldKeyhole/>
                <Bell/>
                <Gear/>
            </div>
        </div>
    )
}

