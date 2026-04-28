import {useEffect, useState} from "react";
import axios from "axios";
import type {Collector} from "../types/Collector.ts";
import {Alert, Card} from "@heroui/react";
import {Link} from "react-router";
import {getBaseUrl} from "../helpFunctions.ts";
// import {CircleCheckFill} from "@gravity-ui/icons";

export default function Home() {
    const [isBackendReachable, setIsBackendReachable] = useState<boolean>(true)
    const [collectors, setCollectors] = useState<Collector[]>([])

    const url = getBaseUrl();

    useEffect(() => {
        // check if backend is reachable
        axios
            .get(url)
            .then(resp => {
                if (resp.data !== "Hello World from monitoring system backend API!") {
                    setIsBackendReachable(false)
                }
            })
            .catch(() => {
                setIsBackendReachable(false)
            })

        // get collectors
        axios
            .get<Collector[]>(url + "/collectors")
            .then(resp => {
                setCollectors(resp.data)
            })
    }, [url]);

    return (
        <main>
            <h1>Welcome!</h1>
            {
                !isBackendReachable
                    ?
                    <div className={"w-125"}>
                        <Alert status={"danger"}>
                            <Alert.Indicator/>
                            <Alert.Content>
                                <Alert.Title>Backend not Reachable</Alert.Title>
                                <Alert.Description>
                                    Unable to connect to Backend API Server.
                                    Please make sure it's running.
                                </Alert.Description>
                            </Alert.Content>
                        </Alert>
                    </div>
                    : <div>
                        <h2>Collectors</h2>
                        <div className={"flex flex-wrap gap-4"}>
                            {
                                collectors.length === 0
                                    ? <div className={"flex flex-col gap-1"}>
                                        <p>0 collectors found!</p>
                                        <p>Make sure that your collector is running and it can reach backend.</p>
                                    </div>
                                    : collectors.map((c, i) => (
                                        <Link to={`/collector/${c.id}`} key={i}>
                                            <Card className={"clickable-small min-w-80"}>
                                                <Card.Header>
                                                    <Card.Title>{c.name}</Card.Title>
                                                    <Card.Description className={"flex flex-col"}>
                                                        {
                                                            c.host_name != c.name &&
                                                            <span className={"-mt-2! font-bold"}>{c.host_name}</span>
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
                                                {/*<Card.Footer>*/}
                                                {/*    <div className={"flex items-center gap-1"}>*/}
                                                {/*        /!* TODO *!/*/}
                                                {/*        <CircleCheckFill className={"text-success"}/>*/}
                                                {/*        <p className={"text-success"}>Online</p>*/}
                                                {/*    </div>*/}
                                                {/*</Card.Footer>*/}
                                            </Card>
                                        </Link>
                                    ))
                            }
                        </div>
                    </div>
            }
        </main>
    )
}