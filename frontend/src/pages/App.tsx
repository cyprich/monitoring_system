import {Button} from "@heroui/react";
import {useEffect, useState} from "react";
import type {WebsocketData} from "../props/WebsocketData.ts";
import {LineChart} from "../components/LineChart.tsx";

function App() {
    const [data, setData] = useState<WebsocketData[]>([])

    useEffect(() => {
        const socket = new WebSocket("ws://localhost:5000/ws");

        socket.addEventListener("open", () => {
            console.log("Websocket opened")
        })

        socket.addEventListener("message", (event) => {
            const newData: WebsocketData = JSON.parse(event.data)
            setData(oldData => [...oldData, newData].slice(-20))
        })

        return () => socket.close()


    }, []);

    return (
        <main className={"flex flex-col"}>
            <Button>Hello World</Button>
            <LineChart inputData={{
                title: "CPU Usage",
                dataset: {
                    name: "CPU 0",
                    data: data.map((val, index) => ({
                        x: index,
                        y: val.cpu_usage[0]
                    }))
                }
            }} />
        </main>
    )
}

export default App
