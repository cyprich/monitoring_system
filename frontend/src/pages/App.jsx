import {useEffect, useState} from "react";
import axios from "axios";
import Line from "../components/charts/Line.jsx";


function App() {
    const [data, setData] = useState([])

    useEffect(() => {
        axios.get("http://localhost:5000/sample_data").then((resp) => {
            console.log(resp)
            setData(resp.data)

        })
    }, []);

    return (
        <main className={"min-h-[90vh] flex-col"}>
            <p>Hello, World!</p>
            <p>Sample data from backend:</p>
            <div className={"pl-4"}>
                {data.map((i) => {
                    return <p>{i}</p>
                })}
            </div>
            <Line values={data}/>
        </main>
    )
}


export default App
