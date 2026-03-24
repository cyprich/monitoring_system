import {useEffect, useState} from "react";
import axios from "axios";
import type {Collector} from "../types/Collector.ts";
import {Card} from "@heroui/react";
import {Link} from "react-router";

export default function Home() {
    return (
        <main>
            <h1>Welcome!</h1>
            <Collectors/>
        </main>
    )
}

function Collectors() {
    const [collectors, setCollectors] = useState<Collector[]>([])

    // TODO
    const URL = 'http://localhost:5000/collectors'

    useEffect(() => {
        axios
            .get(URL)
            .then((resp) => {
                setCollectors(resp.data)
            })
    }, []);

    return (
        <div>
            <h2>Collectors</h2>
            <div className={"flex flex-wrap gap-4"}>
                {
                    collectors.map((c, i) => (
                        <Link to={`/collector/${c.id}`} key={i}>
                            <Card className={"clickable min-w-80"}>
                                <Card.Header>
                                    <Card.Title>{c.name}</Card.Title>
                                    <Card.Description className={"flex flex-col"}>
                                        {
                                            c.host_name != c.name && <span>{c.host_name}</span>
                                        }
                                        <span className={"flex items-center"}>
                                            <span>{c.system_name}</span>
                                            <span className={"w-0.5 h-5 mx-2 bg-black/25"}/>
                                            <span>{c.kernel_version}</span>
                                        </span>
                                        <span>CPU: {c.cpu_count} cores</span>
                                        <span>RAM: {((c.total_memory_mb || 0) / 1000).toFixed(0)} GB</span>
                                    </Card.Description>
                                </Card.Header>
                                <Card.Footer>
                                    <div className={"flex items-center gap-1"}>
                                        <p>Last seen: <span className={"text-gray-500"}>TODO</span></p>
                                        {/*<CircleCheckFill className={"text-success"}/>*/}
                                        {/*<p className={"text-success"}>Online</p>*/}
                                    </div>
                                </Card.Footer>
                            </Card>
                        </Link>
                    ))
                }
            </div>
        </div>
    )
}