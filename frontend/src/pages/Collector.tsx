import type {Metrics} from "../types/Metrics.ts";
import CustomChart from "../components/CustomChart.tsx";
import type {Collector, Drive, NetworkInterface} from "../types/Collector.ts";
import axios from "axios";
import {useParams} from "react-router";
import CustomSurface from "../components/CustomSurface.tsx";
import {Tabs} from "@heroui/react";
import {useEffect, useState} from "react";
import {getMetricsLimit} from "../helpFunctions.ts";
import {SettingsMetricsCountSection} from "../components/settings/SettingsMetricsCountSection.tsx";
import SettingsGeneralSection from "../components/settings/SettingsGeneralSection.tsx";
import ConfirmableInput from "../components/ConfirmableInput.tsx";

export default function Collector() {
    const params = useParams();
    const id = params.id || "0";

    const [collector, setCollector] = useState<Collector | null>(null)
    const [data, setData] = useState<Metrics[]>([])

    // TODO link
    const url = `http://localhost:5000/collector/${id}`;
    const LIMIT = getMetricsLimit();

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
                    limit: LIMIT
                }
            })
            .then((resp) => {
                const data: Metrics[] = resp.data.map((i: Metrics) => (
                    {
                        ...i,
                        timestamp: new Date(i.timestamp)
                    }
                ))
                setData(data);
            })

        const socket = new WebSocket(`ws://localhost:5000/ws/metrics/${id}`);

        socket.addEventListener("open", () => {
            console.log("Websocket opened")
        })

        socket.addEventListener("message", (event) => {
            const newData: Metrics = JSON.parse(event.data)
            console.log(event.data, newData)
            // https://howtodoinjava.com/typescript/typescript-date-object/
            newData.timestamp = new Date(newData.timestamp)
            setData(oldData => [...oldData, newData].slice(-LIMIT))
        })

        return () => socket.close()

    }, [LIMIT, id, url]);

    return (
        <main className={"flex flex-col gap-4"}>
            <h1>{collector?.name}</h1>
            <CustomSurface title={"Metrics"}>
                <MetricsTabs collector={collector} data={data}/>
            </CustomSurface>

            <CustomSurface title={"API Endpoints"}>
                <p className={"custom-description"}>//TODO</p>
            </CustomSurface>

            <CustomSurface title={"Security stuff?"}>
                <p className={"custom-description"}>//TODO</p>
            </CustomSurface>

            <CustomSurface title={"Settings"} className={"flex flex-col gap-6"}>
                <div>
                    <SettingsGeneralSection title={"Collector name"}>
                        {
                            collector !== null &&
                            <ConfirmableInput
                                value={collector.name}
                                variant={"secondary"}
                                onConfirm={(newName) => {
                                    if (collector === null) { return }
                                    axios
                                        .patch(`${url}/rename`, {"name": newName})
                                        .then(() => {
                                            setCollector((old) => old ? {...old, name: newName} : old)
                                        }).catch((e) => { console.log(e) /* TODO */ })
                                }}
                            />
                        }
                    </SettingsGeneralSection>
                </div>
                <div>
                    <SettingsMetricsCountSection showWarning={true}/>
                </div>
            </CustomSurface>
        </main>
    )
}

interface CollectorProps {
    collector: Collector | null,
    data: Metrics[]
}

function MetricsTabs({collector, data}: CollectorProps) {
    const className = "grid gap-y-24 mt-8"

    return (
        <>
            <Tabs>
                <Tabs.ListContainer>
                    <Tabs.List>
                        <Tabs.Tab id={"cpu"}>
                            CPU
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"mem"}>
                            Memory
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"drives"}>
                            Drives
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"net"}>
                            Network
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                    </Tabs.List>
                </Tabs.ListContainer>

                <Tabs.Panel id={"cpu"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(2, 1fr)"}}>
                        <CpuChart collector={collector} data={data}/>
                        <CpuChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"mem"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(2, 1fr)"}}>
                        <RamChart collector={collector} data={data}/>
                        <SwapChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"drives"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(3, 1fr)"}}>
                        <DriveChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"net"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(3, 1fr)"}}>
                        <NetworkChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
            </Tabs>
        </>
    )
}

function CpuChart(props: CollectorProps) {
    return (
        <CustomChart name={"CPU"} keys={["CPU"]} data={
            props.data.map((i) => ({
                timestamp: i.timestamp.toLocaleTimeString(),
                cpu: i.cpu_usage
            }))
        } unit={"%"} max_y={100} />
    )

}

function RamChart(props: CollectorProps) {
    return (
        <CustomChart name={"RAM"} keys={["RAM"]} data={
            props.data.map((i) => ({
                timestamp: i.timestamp.toLocaleTimeString(),
                ram: i.used_memory_mb
            }))
        } unit={"MB"} max_y={props.collector?.total_memory_mb || undefined} />
    )
}

function SwapChart(props: CollectorProps) {
    return (
        <CustomChart name={"Swap"} keys={["Swap"]} data={
            props.data.map((i) => ({
                timestamp: i.timestamp.toLocaleTimeString(),
                swap: i.used_swap_mb
            }))
        } unit={"MB"} max_y={props.collector?.total_swap_mb || undefined} />
    )
}

function DriveChart(props: CollectorProps) {
    return (
        <>
            {
                props.collector?.drives?.map((drive, i) => (
                    <CustomChart
                        name={`Drive at '${drive.mountpoint}'`}
                        key={i}
                        keys={["available_space_gb"]}
                        data={
                            props.data.map((metric) => {
                                const selected = metric.drives?.find(
                                    (a) => a.mountpoint == drive.mountpoint
                                )

                                return {
                                    timestamp: metric.timestamp.toLocaleTimeString(),
                                    available_space_gb: selected?.available_space_gb || 0
                                }
                            })
                        }
                        unit={"GB"}
                        max_y={drive.capacity_gb}/>
                ))
            }
        </>
    )
}

function NetworkChart(props: CollectorProps) {
    // console.log(props)
    return (
        <>
            {
                props.collector?.network_interfaces?.map((network, i) => (
                    <CustomChart
                        name={network.name}
                        key={i}
                        keys={["Download", "Upload"]}
                        data={
                            props.data.map((metric) => {
                                const selected = metric.network_interfaces?.find(
                                    (a) => a.name == network.name
                                )

                                return {
                                    timestamp: metric.timestamp.toLocaleTimeString(),
                                    download: (selected?.download_kb || 0) / 1000,
                                    upload: (selected?.upload_kb || 0) / 1000,
                                }
                            })
                        }
                        unit={"MB"}
                        max_y={10}/>
                ))
            }
        </>
    )
}
