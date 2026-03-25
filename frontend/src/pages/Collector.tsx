import type {WebsocketData} from "../types/WebsocketData.ts";
import CustomChart from "../components/CustomChart.tsx";
import type {Collector} from "../types/Collector.ts";
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
    const [data, setData] = useState<WebsocketData[]>([])

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

        // historic metrics
        axios
            .get(`${url}/metrics`, {
                params: {
                    limit: LIMIT
                }
            })
            .then((resp) => {
                const data: WebsocketData[] = resp.data.map((i: WebsocketData) => (
                    {
                        ...i,
                        timestamp: new Date(i.timestamp)
                    }
                ))
                setData(data)
            })

        const socket = new WebSocket(`ws://localhost:5000/ws/metrics/${id}`);

        socket.addEventListener("open", () => {
            console.log("Websocket opened")
        })

        socket.addEventListener("message", (event) => {
            const newData: WebsocketData = JSON.parse(event.data)
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
                                    console.log(`${url}/rename`)
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
    data: WebsocketData[]
}

function MetricsTabs({collector, data}: CollectorProps) {
    const className = "grid mt-8"

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
                        <MemoryCharts collector={collector} data={data}/>
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

function MemoryCharts(props: CollectorProps) {
    const max_y = props.collector === null ? undefined : (
        Math.max(props.collector.total_memory_mb || 0, props.collector.total_swap_mb || 0)
    )

    return (
        <>
            <CustomChart name={"RAM"} keys={["RAM"]} data={
                props.data.map((i) => ({
                    timestamp: i.timestamp.toLocaleTimeString(),
                    ram: i.used_memory_mb
                }))
            } unit={"MB"} max_y={max_y} />

            <CustomChart name={"SWAP"} keys={["SWAP"]} data={
                props.data.map((i) => ({
                    timestamp: i.timestamp.toLocaleTimeString(),
                    swap: i.used_swap_mb
                }))
            } unit={"MB"} max_y={max_y} />

        </>
    )
}

function NetworkChart(props: CollectorProps) {
    // TODO
    const interfaceNames = ["wlan0", "tailscale0"];

    return (
        <>
            {
                // TODO idk if this is the best - same with drives
                interfaceNames.map((name) => {
                    return (
                        <CustomChart name={`Network (${name})`} keys={["Upload", "Download"]} data={
                            props.data.map((i) => {
                                const net = i.networks.find((n) => name === n.name);

                                return {
                                    timestamp: i.timestamp.toLocaleTimeString(),
                                    upload: net?.upload_mb || 0,
                                    download: net?.download_mb || 0,
                                }
                            })
                        } unit={"MB"} max_y={1000} />
                    )
                })
            }
        </>
    )
}

function DriveChart(props: CollectorProps) {
    // TODO
    const drives = props.data[0]?.disks.map((d) => (d.mountpoint));

    return (
        <>
            {
                drives?.map((name) => {
                    return (
                        <CustomChart name={name} keys={["available_space"]} data={
                            props.data.map((i) => {
                                const drive = i.disks.find((d) => d.mountpoint == name);

                                return {
                                    timestamp: i.timestamp.toLocaleTimeString(),
                                    available_space: (drive?.available_space_mb || 0) / 1000
                                }
                            })
                        } unit={"GB"} max_y={1_000}/>
                    )
                })
            }
        </>
    )
}