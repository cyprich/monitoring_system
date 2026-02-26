import {useEffect, useState} from "react";
import Line from "../components/charts/Line.jsx";


function App() {
    const [data, setData] = useState([])

    useEffect(() => {
        const socket = new WebSocket("ws://127.0.0.1:8000");

        socket.addEventListener("open", () => {
            console.log("Opened Websocket connection")
        })

        socket.addEventListener("message", (event) => {
            // https://www.w3schools.com/js/js_array_methods.asp#mark_slice
            setData(data => [...data, event.data].slice(-20))
        })

        return () => socket.close();
    }, []);

    return (
        <main className={"min-h-[90vh] flex-col"}>
            <p>Hello, World!</p>
            <Line values={data}/>
        </main>
    )
}


export default App
