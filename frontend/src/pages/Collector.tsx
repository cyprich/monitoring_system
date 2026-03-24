import {type ReactNode, useEffect, useState} from "react";
import type {WebsocketData} from "../types/WebsocketData.ts";
import CustomChart from "../components/CustomChart.tsx";
import type {Collector} from "../types/Collector.ts";
import axios from "axios";
import {useParams} from "react-router";
import CustomSurface from "../components/CustomSurface.tsx";
import {Tabs} from "@heroui/react";

export default function Collector() {
    const params = useParams();
    const id = params.id || "0";

    const [collector, setCollector] = useState<Collector | null>(null)
    const [data, setData] = useState<WebsocketData[]>([])

    // TODO
    const url = `http://localhost:5000/collector/${id}`;
    const LIMIT = 200;

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
            .get(`${url}/metrics`)
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

    }, [id, url]);

    // TODO make it into tabs instead of the `CollectorSection`?

    return (
        <main className={"flex flex-col gap-4"}>
            <h1>{collector?.name}</h1>
            <CustomSurface title={"Metrics"}>
                <MetricsTabs collector={collector} data={data}/>
            </CustomSurface>

            <CustomSurface title={"API Endpoints"}>
                <p className={"text-gray-500"}>//TODO</p>
            </CustomSurface>

            <CustomSurface title={"Security stuff"}>
                <p className={"text-gray-500"}>//TODO</p>
            </CustomSurface>

            <CustomSurface title={"Settings"}>
                <p className={"text-gray-500"}>//TODO</p>
            </CustomSurface>
        </main>
    )
}

interface CollectorProps {
    collector: Collector | null,
    data: WebsocketData[]
}

interface CollectionSectionProps {
    name: string,
    columns: number,
    children: ReactNode
}

function MetricsTabs({collector, data}: CollectorProps) {
    function className(gridCols: number): string {
        return `grid grid-cols-${gridCols} mt-8`
    }

    return (
        <>
            <Tabs>
                <Tabs.ListContainer>
                    <Tabs.List>
                        <Tabs.Tab id={"cpu"}>
                            CPU
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"ram"}>
                            RAM
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
                    <div className={className(2)}>
                        <CpuChart collector={collector} data={data}/>
                        <CpuChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"ram"}>
                    <div className={className(2)}>
                        <RamChart collector={collector} data={data}/>
                        <RamChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"drives"}>
                    <div className={className(3)}>
                        <DriveChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"net"}>
                    <div className={className(3)}>
                        <NetworkChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
            </Tabs>



            {/*<CollectorSection name={"CPU & RAM usage"} columns={2}>*/}
            {/*    <CpuChart collector={collector} data={data}/>*/}
            {/*    <RamChart collector={collector} data={data}/>*/}
            {/*</CollectorSection>*/}

            {/*<CollectorSection name={"Networks"} columns={4}>*/}
            {/*    <NetworkChart collector={collector} data={data}/>*/}
            {/*</CollectorSection>*/}

            {/*<CollectorSection name={"Drives"} columns={4}>*/}
            {/*    <DriveChart collector={collector} data={data}/>*/}
            {/*</CollectorSection>*/}
        </>

)

}


function CollectorSection(props: CollectionSectionProps) {
    return (
        <div>
            <h3>{props.name}</h3>
            <div
                className={ `grid grid-flow-row gap-16`}
                style={{gridTemplateColumns: `repeat(${props.columns}, 1fr)`}}
            >
                {
                    props.children
                }
            </div>
        </div>
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