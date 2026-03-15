import {useEffect, useState} from "react";
import type {WebsocketData} from "../types/WebsocketData.ts";
import CustomLineChart from "../components/CustomLineChart.tsx";

function Dashboard() {
    const [data, setData] = useState<WebsocketData[]>([])

    useEffect(() => {
        const socket = new WebSocket("ws://localhost:5000/ws");

        socket.addEventListener("open", () => {
            console.log("Websocket opened")
        })

        socket.addEventListener("message", (event) => {
            const newData: WebsocketData = JSON.parse(event.data)
            // https://howtodoinjava.com/typescript/typescript-date-object/
            newData.timestamp = new Date(newData.timestamp)
            setData(oldData => [...oldData, newData].slice(-20))
        })

        return () => socket.close()

    }, []);

    return (
        <main className={"flex flex-col"}>
            <h1>Dashboard</h1>
            <div className={"grid grid-flow-row grid-cols-3 gap-16"}>
                <CustomLineChart name={"CPU"} keys={["CPU"]} data={
                    data.map((i) => ({
                        timestamp: i.timestamp.toLocaleTimeString(),
                        cpu: i.cpu_usage
                    }))
                } unit={"%"} max_y={100} />

                <CustomLineChart name={"RAM"} keys={["RAM"]} data={
                    data.map((i) => ({
                        timestamp: i.timestamp.toLocaleTimeString(),
                        ram: i.used_mem / 1000000
                    }))
                } unit={"MB"} max_y={16000} />

                <CustomLineChart name={`Network (${data[0]?.networks[0]?.name})`} keys={["Upload", "Download"]} data={
                    data.map((i) => ({
                        timestamp: i.timestamp.toLocaleTimeString(),
                        upload: i.networks[0].upload / 1000000,
                        download: i.networks[0].download / 1000000,
                    }))
                } unit={"Mb"} max_y={1} />
            </div>
        </main>
    )
}

export default Dashboard
