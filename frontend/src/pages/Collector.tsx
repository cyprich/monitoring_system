import {type ReactNode, useEffect, useState} from "react";
import type {WebsocketData} from "../types/WebsocketData.ts";
import CustomChart from "../components/CustomChart.tsx";
import type {Collector} from "../types/Collector.ts";
import axios from "axios";
import {useParams} from "react-router";

export default function Collector() {
    const params = useParams();
    const id = params.id || "0";

    const [collector, setCollector] = useState<Collector | null>(null)
    const [data, setData] = useState<WebsocketData[]>([])

    // TODO
    const url = `http://localhost:5000/collector/${id}`;

    useEffect(() => {
        // TODO historical metrics
        axios
            .get(url)
            .then((resp) => {
                // TODO check response code
                setCollector(resp.data)
            })

        const socket = new WebSocket(`ws://localhost:5000/ws/metrics/${id}`);

        socket.addEventListener("open", () => {
            console.log("Websocket opened")
        })

        socket.addEventListener("message", (event) => {
            const newData: WebsocketData = JSON.parse(event.data)
            // https://howtodoinjava.com/typescript/typescript-date-object/
            newData.timestamp = new Date(newData.timestamp)
            setData(oldData => [...oldData, newData].slice(-30))
        })

        return () => socket.close()

    }, [id, url]);

    // TODO make it into tabs instead of the `CollectorSection`?

    return (
        <main className={"flex flex-col"}>
            <h1>{collector?.name}</h1>
            <CollectorSection name={"CPU & RAM usage"} columns={2}>
                <CpuChart collector={collector} data={data}/>
                <RamChart collector={collector} data={data}/>
            </CollectorSection>

            <CollectorSection name={"Networks"} columns={4}>
                <NetworkChart collector={collector} data={data}/>
            </CollectorSection>

            <CollectorSection name={"Drives"} columns={4}>
                <DriveChart collector={collector} data={data}/>
            </CollectorSection>
        </main>
    )
}

interface CollectionSectionProps {
    name: string,
    columns: number,
    children: ReactNode
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

interface CollectorProps {
    collector: Collector | null,
    data: WebsocketData[]
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
                                    available_space: drive?.available_space_mb || 0
                                }
                            })
                        } unit={"MB"} max_y={1_000_000}/>
                    )
                })
            }
        </>
    )
}