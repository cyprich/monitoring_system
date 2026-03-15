import {useEffect, useState} from "react";
import type {WebsocketData} from "../props/WebsocketData.ts";
import {LineChart} from "../components/LineChart.tsx";

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
                <LineChart inputData={{
                    title: "CPU Usage",
                    dataset: {
                        name: "CPU",
                        data: data.map((val) => ({
                            x: val.timestamp,
                            y: val.cpu_usage
                        }))
                    }
                }} max_y_scale={100}/>

                <LineChart inputData={{
                    title: "RAM Usage",
                    dataset: {
                        name: "RAM",
                        data: data.map((val) => ({
                            x: val.timestamp,
                            y: val.used_mem
                        }))
                    }
                }} max_y_scale={undefined}/>

                <div/>

                <LineChart inputData={{
                    title: "Network",
                    dataset: {
                        name: "Network Upload",
                        data: data.map((val) => ({
                            x: val.timestamp,
                            y: val.networks[0].upload
                        }))
                    }
                }} max_y_scale={undefined}/>

                <LineChart inputData={{
                    title: "Network",
                    dataset: {
                        name: "Network Download",
                        data: data.map((val) => ({
                            x: val.timestamp,
                            y: val.networks[0].download
                        }))
                    }
                }} max_y_scale={undefined}/>
            </div>
        </main>
    )
}

export default Dashboard
