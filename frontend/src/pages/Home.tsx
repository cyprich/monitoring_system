import {useEffect, useState} from "react";
import axios from "axios";
import type {Collector} from "../types/Collector.ts";
import {Card, Separator} from "@heroui/react";
import {Link, useNavigate} from "react-router";
// import {CircleCheckFill} from "@gravity-ui/icons";

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
            <div className={"flex gap-4"}>
                {
                    collectors.map((c) => (
                        <Link to={`/collector/${c.id}`}>
                            <Card className={"clickable min-w-72"}>
                                <Card.Header>
                                    <Card.Title>{c.host_name}</Card.Title>
                                    <Card.Description className={"flex items-center"}>
                                        {c.system_name}
                                        <Separator className={"w-0.5 h-5 mx-2"} orientation={"vertical"} />
                                        {c.kernel_version}
                                    </Card.Description>
                                </Card.Header>
                                <Card.Footer>
                                    <div className={"flex items-center gap-1"}>
                                        <p>Last seen: </p>
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