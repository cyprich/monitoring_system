import {useEffect, useState} from "react";
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

    return (
        <main className={"flex flex-col"}>
            <h1>Collector</h1>
            <h2>{collector?.host_name}</h2>
            <div className={"grid grid-flow-row grid-cols-2 gap-16"}>
                <CustomChart name={"CPU"} keys={["CPU"]} data={
                    data.map((i) => ({
                        timestamp: i.timestamp.toLocaleTimeString(),
                        cpu: i.cpu_usage
                    }))
                } unit={"%"} max_y={100} />

                <CustomChart name={"RAM"} keys={["RAM"]} data={
                    data.map((i) => ({
                        timestamp: i.timestamp.toLocaleTimeString(),
                        ram: i.used_mem / 1000000
                    }))
                } unit={"MB"} max_y={16000} />



                <CustomChart name={`Network (${data[0]?.networks[0]?.name})`} keys={["Upload", "Download"]} data={
                    data.map((i) => {
                        const net = i.networks.find((n) => n.name === "wlan0");

                        return {
                            timestamp: i.timestamp.toLocaleTimeString(),
                            upload: (net?.upload || 0) / 1_000_000,
                            download: (net?.download || 0) / 1_000_000,
                        }
                    })
                } unit={"Mb"} max_y={100} />
            </div>
        </main>
    )
}

