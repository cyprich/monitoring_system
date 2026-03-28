import type {Metrics} from "../types/Metrics.ts";
import CustomChart from "../components/CustomChart.tsx";
import type {Collector, Drive, NetworkInterface} from "../types/Collector.ts";
import axios from "axios";
import {Link, useParams} from "react-router";
import CustomSurface from "../components/CustomSurface.tsx";
import {
    AlertDialog,
    Button,
    Description, EmptyState,
    FieldError,
    Fieldset,
    Form,
    Input,
    Label,
    Table,
    Tabs,
    TextField
} from "@heroui/react";
import {useEffect, useState} from "react";
import {getMetricsLimit} from "../helpFunctions.ts";
import {SettingsMetricsCountSection} from "../components/settings/SettingsMetricsCountSection.tsx";
import SettingsGeneralSection from "../components/settings/SettingsGeneralSection.tsx";
import ConfirmableInput from "../components/ConfirmableInput.tsx";
import type {Endpoint} from "../types/Endpoints.ts";
import {ChevronLeft, Plus, TrashBin, Tray} from "@gravity-ui/icons";

// TODO split into multiple files

export default function Collector() {
    const params = useParams();
    const id = params.id || "0";

    const [collector, setCollector] = useState<Collector | null>(null)
    const [data, setData] = useState<Metrics[]>([])

    // TODO link
    const url = `http://localhost:5000/collector/${id}`;
    const LIMIT = getMetricsLimit();

    useEffect(() => {
        // collector
        axios
            .get(url)
            .then((resp) => {
                // TODO check response code
                setCollector(resp.data)
            })

        // drives
        axios.get(`${url}/drives`).then((resp) => {
            const drives: Drive[] = (resp.data as any[]).map(
                (d: Drive) => ({
                    mountpoint: d.mountpoint,
                    capacity_gb: d.capacity_gb,
                    file_system: d.file_system
                })
            )

            setCollector(
                (old) => (
                    old ? {...old, drives: drives} : old
                )
            )
        });

        // network interfaces
        axios.get(`${url}/network_interfaces`).then((resp) => {
            const network_interfaces: NetworkInterface[] = (
                resp.data as any[]).map((n: NetworkInterface) => ({
                    name: n.name, mac: n.mac
                })
            )

            setCollector(
                (old) => (
                    old ? {...old, network_interfaces: network_interfaces} : old
                )
            )
        });

        // historic metrics
        axios
            .get(`${url}/metrics`, {
                params: {
                    limit: LIMIT
                }
            })
            .then((resp) => {
                const data: Metrics[] = resp.data.map((i: Metrics) => (
                    {
                        ...i,
                        timestamp: new Date(i.timestamp)
                    }
                ))
                setData(data);
            })


    }, [LIMIT, id, url]);

    useEffect(() => {
        const socket = new WebSocket(`ws://localhost:5000/ws/metrics/${id}`);

        socket.addEventListener("open", () => {
            console.log("Websocket opened")
        })

        socket.addEventListener("message", (event) => {
            const newData: Metrics = JSON.parse(event.data)
            // https://howtodoinjava.com/typescript/typescript-date-object/
            newData.timestamp = new Date(newData.timestamp)
            setData(oldData => [...oldData, newData].slice(-LIMIT))
        })

        return () => socket.close()

    }, [LIMIT, id]);

    return (
        <main className={"flex flex-col gap-4"}>
            <Link to={"/"} className={"flex items-center custom-description hover:underline w-max"}>
                <ChevronLeft/>Home
            </Link>
            {
                collector && <CollectorHeader {...collector} />
            }
            <CustomSurface title={"Metrics"}>
                <MetricsTabs collector={collector} data={data}/>
            </CustomSurface>

            <CustomSurface title={"Notifications"}>
                <NotificationsTab/>
            </CustomSurface>

            <CustomSurface title={"API Endpoints"}>
                <EndpointsTab collector_id={collector?.id || 0}/>
            </CustomSurface>

            <CustomSurface title={"Security stuff?"}>
                <p className={"custom-description"}>//TODO</p>
            </CustomSurface>

            <CustomSurface title={"Settings"} className={"flex flex-col gap-6"}>
                <div>
                    <SettingsGeneralSection title={"Collector name"}>
                        {
                            collector !== null &&
                            <ConfirmableInput
                                value={collector.name}
                                variant={"secondary"}
                                onConfirm={(newName) => {
                                    if (collector === null) { return }
                                    axios
                                        .patch(`${url}/rename`, {"name": newName})
                                        .then(() => {
                                            setCollector((old) => old ? {...old, name: newName} : old)
                                        }).catch((e) => { console.log(e) /* TODO */ })
                                }}
                            />
                        }
                    </SettingsGeneralSection>
                </div>
                <div>
                    <SettingsMetricsCountSection showWarning={true}/>
                </div>
            </CustomSurface>
        </main>
    )
}

function CollectorHeader(collector: Collector) {
    const total_capacity = collector.drives?.reduce((acc, d) => acc + d.capacity_gb, 0) || 0;

    // TODO hidden

    return (
        <div className={"flex flex-col gap-1"}>
            <h1>{collector.name}</h1>
            {
                collector.name !== collector.host_name && <h3 className={"mb-0! -mt-2!"}>{collector.host_name}</h3>
            }
            <p className={"flex items-center"}>
                {collector.system_name}
                <span className={"w-0.5 h-5 mx-2 bg-black/40"}/>
                {collector.kernel_version}
            </p>
            <p>{collector.total_memory_mb} MB RAM</p>
            <p>{collector.total_swap_mb} MB Swap</p>
            <p>{collector.cpu_count} CPU Cores</p>
            <p>
                {collector.drives?.length || 0} drives
                <span className={"font-extralight"}> with total capacity of </span>
                {total_capacity}GB
            </p>
            <p>{collector.network_interfaces?.length || 0} network interfaces</p>
        </div>
    )
}

interface CollectorProps {
    collector: Collector | null,
    data: Metrics[]
}

function MetricsTabs({collector, data}: CollectorProps) {
    const className = "grid gap-y-24 mt-8"

    return (
        <>
            <Tabs>
                <Tabs.ListContainer>
                    <Tabs.List>
                        <Tabs.Tab id={"cpu"}>
                            CPU
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"mem"}>
                            Memory
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"drives"}>
                            Drives
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"net"}>
                            Network
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                    </Tabs.List>
                </Tabs.ListContainer>

                <Tabs.Panel id={"cpu"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(2, 1fr)"}}>
                        <CpuChartGlobal collector={collector} data={data}/>
                        <CpuChartCores collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"mem"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(2, 1fr)"}}>
                        <RamChart collector={collector} data={data}/>
                        <SwapChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"drives"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(3, 1fr)"}}>
                        <DriveChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"net"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(3, 1fr)"}}>
                        <NetworkChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
            </Tabs>
        </>
    )
}

function CpuChartGlobal(props: CollectorProps) {
    return (
        <CustomChart name={"CPU Total Usage"} keys={["CPU"]} data={
            props.data.map((i) => ({
                timestamp: i.timestamp.toLocaleTimeString(),
                cpu: i.cpu_usage_global
            }))
        } unit={"%"} max_y={100} />
    )

}

function CpuChartCores(props: CollectorProps) {
    const cpu_count = props.collector?.cpu_count || 0;
    const keys = Array.from({length: cpu_count}, (_, index) => index.toString())

    return (
        <CustomChart
            name={"CPU Usage Per Core"}
            keys={keys}
            data={
                props.data.map((i) => {
                    const result: {timestamp: string, [index: number]: number | string} = {
                        timestamp: i.timestamp.toLocaleTimeString()
                    }

                    i.cpu_usage_cores.forEach((value, index) => {
                        result[`${index}`] = value
                    })

                    return result
                })
            }
            unit={"%"}
            max_y={100}
            lighter={true}
        />
    )
}

function RamChart(props: CollectorProps) {
    return (
        <CustomChart
            name={"RAM"}
            keys={["RAM"]}
            data={
                props.data.map((i) => ({
                    timestamp: i.timestamp.toLocaleTimeString(),
                    ram: i.used_memory_mb
                }))
            }
            unit={"MB"}
            max_y={props.collector?.total_memory_mb || undefined}
            showTooltipPercent={true}
        />
    )
}

function SwapChart(props: CollectorProps) {
    return (
        <CustomChart
            name={"Swap"}
            keys={["Swap"]}
            data={
                props.data.map((i) => ({
                    timestamp: i.timestamp.toLocaleTimeString(),
                    swap: i.used_swap_mb
                }))
            }
            unit={"MB"}
            max_y={props.collector?.total_swap_mb || undefined}
            showTooltipPercent={true}
        />
    )
}

function DriveChart(props: CollectorProps) {
    return (
        <>
            {
                props.collector?.drives?.map((drive, i) => (
                    <CustomChart
                        name={`Drive at '${drive.mountpoint}'`}
                        key={i}
                        keys={["Used"]}
                        data={
                            props.data.map((metric) => {
                                const selected = metric.drives?.find(
                                    (a) => a.mountpoint == drive.mountpoint
                                )

                                return {
                                    timestamp: metric.timestamp.toLocaleTimeString(),
                                    used: selected?.used_space_gb || 0
                                }
                            })
                        }
                        unit={"GB"}
                        max_y={drive.capacity_gb}
                        showTooltipPercent={true}
                    />
                ))
            }
        </>
    )
}

function NetworkChart(props: CollectorProps) {
    return (
        <>
            {
                props.collector?.network_interfaces?.map((network, i) => (
                    <CustomChart
                        name={network.name}
                        key={i}
                        keys={["Download", "Upload"]}
                        data={
                            props.data.map((metric) => {
                                const selected = metric.network_interfaces?.find(
                                    (a) => a.name == network.name
                                )

                                return {
                                    timestamp: metric.timestamp.toLocaleTimeString(),
                                    download: (selected?.download_kb || 0) / 1000,
                                    upload: (selected?.upload_kb || 0) / 1000,
                                }
                            })
                        }
                        unit={"MB"}
                        max_y={10}
                        farColors={true}
                    />
                ))
            }
        </>
    )
}

function NotificationsTab() {
    return (
        <p className={"custom-description"}>//TODO</p>
    )
}

interface EndpointsTabProps {
    collector_id: number,
}

function EndpointsTab(props: EndpointsTabProps) {
    const [endpoints, setEndpoints] = useState<Array<Endpoint> | null>(null)
    const [isDeleteOpen, setIsDeleteOpen] = useState<boolean>(false)
    const [deletingEndpointId, setDeletingEndpointId] = useState<number>(0)

    const [formResponseInputValue, setFormResponseInputValue] = useState<string>("")
    const [formResponseCodes, setFormResponseCodes] = useState<number[]>([])

    // TODO url
    const url = `http://localhost:5000/collector/${props.collector_id}/endpoints`

    useEffect(() => {
        axios.get(url).then((resp) => {
            console.log(resp.data)
            const endpoints: Array<Endpoint> = resp.data.map((e) => {
                const val: Endpoint = e
                return val
            })
            setEndpoints(endpoints)
        })
    }, [url]);

    function deleteEndpoint() {
        axios
            .delete(`${url}/${deletingEndpointId}`)
            .then(() => {})
            .catch((e) => console.log(e))
    }


    return (
        <div className={"flex flex-col gap-4"}>
            <Table>
                <Table.ScrollContainer>
                    <Table.Content aria-label="Team members" >
                        <Table.Header>
                            {/*<Table.Column isRowHeader>Method</Table.Column>*/}
                            <Table.Column isRowHeader>URL</Table.Column>
                            <Table.Column>Expected Response Codes</Table.Column>
                            <Table.Column>Actions</Table.Column>
                        </Table.Header>
                        <Table.Body renderEmptyState={() => (
                            <EmptyState className={"flex flex-col justify-center items-center bg-background  rounded-2xl py-8"}>
                                <Tray className={"size-16 opacity-80 mb-2"}/>
                                <span>No results found</span>
                                <span>Start by adding your first endpoint</span>
                            </EmptyState>
                        )}>
                            {
                                endpoints?.map((e, i) => (
                                    <Table.Row key={i}>
                                        {/*<Table.Cell>{e.method}</Table.Cell>*/}
                                        <Table.Cell>
                                            <ConfirmableInput
                                                value={e.url}
                                                onConfirm={() => {}}
                                                variant={"primary"}
                                                className={"w-64"}
                                            />
                                        </Table.Cell>
                                        <Table.Cell>{e.expected_codes.join(", ")}</Table.Cell>
                                        <Table.Cell>
                                            <div
                                                className={"bg-red-100 hover:bg-red-200 " +
                                                    "rounded-lg w-max h-max p-2 transition-all cursor-pointer " +
                                                    "active:scale-105 hover:*:text-red-600 hover:*:scale-105 "}
                                                onClick={() => {
                                                    setDeletingEndpointId(e.id)
                                                    setIsDeleteOpen(true)
                                                }}
                                            >
                                                <TrashBin className={"size-5 text-red-500"}/>
                                            </div>
                                        </Table.Cell>
                                    </Table.Row>
                                ))
                            }
                        </Table.Body>
                    </Table.Content>
                </Table.ScrollContainer>
                <Table.Footer>
                    <p className={"font-light text-sm"}>{endpoints?.length || 0} result{endpoints?.length !== 1 && "s"}</p>
                </Table.Footer>
            </Table>
            <Form className={"flex flex-col gap-2 mt-2"}>
                <Fieldset>
                    <Fieldset.Legend>Add new Endpoint</Fieldset.Legend>
                    <Description>Create new Endpoint that will be monitored by this Collector</Description>
                    <TextField isRequired name={"url"} type={"text"} validate={() => true}>
                        <Label>URL</Label>
                        <Input placeholder={"http://192.168.10.10:80/api"} variant={"secondary"}/>
                        <FieldError/>
                    </TextField>
                    <TextField name={"responsecodes"} type={"text"} validate={() => true}>
                        <Label>Expected Response Codes</Label>
                        <div className={"flex flex-col gap-2"}>
                            <div className={"flex gap-2"}>
                                <Input
                                    variant={"secondary"}
                                    type={"number"}
                                    className={"w-48"}
                                    value={formResponseInputValue}
                                    onChange={(e) => setFormResponseInputValue(e.target.value)}
                                    placeholder={"200"}
                                />
                                <Button
                                    variant={"tertiary"}
                                    className={"aspect-square"}
                                    onClick={() => {
                                        const val = Number(formResponseInputValue)
                                        if (val === 0 || formResponseCodes.find(
                                            (c) => (c === val)
                                        )) { return }

                                        setFormResponseCodes(
                                            (prev) => [ ...prev, Number(val) ]
                                        )
                                    }
                                }>
                                    <Plus/>
                                </Button>
                            </div>
                            <div className={"flex gap-1"}>
                                {
                                    formResponseCodes.map((c, i) => (
                                        <div
                                            className={"bg-background p-2 w-max rounded-xl relative cursor-pointer group min-w-10"}
                                            key={i}
                                            onClick={() => {
                                                const newList = formResponseCodes.filter((x) => x !== c)
                                                setFormResponseCodes(newList)
                                            }}
                                        >
                                            <p className={"group-hover:opacity-0 transition-opacity text-center"}>{c}</p>
                                            <TrashBin className={"absolute top-[50%] left-[50%] translate-[-50%] " +
                                                "size-6 bg-background text-red-600 " +
                                                "opacity-0 group-hover:opacity-100 transition-opacity "}/>
                                        </div>
                                    ))
                                }
                            </div>
                        </div>
                    </TextField>

                    <div className={"flex gap-2"}>
                        <Button type={"submit"}>Add</Button>
                        <Button type={"reset"} variant={"secondary"} onClick={() => {setFormResponseCodes([])}}>Reset</Button>
                    </div>
                </Fieldset>
            </Form>
            {
                isDeleteOpen && <AlertDialog>
                    <AlertDialog.Backdrop isOpen={isDeleteOpen} onOpenChange={setIsDeleteOpen}>
                        <AlertDialog.Dialog>
                            <AlertDialog.Header>
                                <p>Delete Endpoint?</p>
                            </AlertDialog.Header>
                            <AlertDialog.Body>
                                <p>Collector will no longer send requests to this endpoint</p>
                                <p>This action is not reversible</p>
                            </AlertDialog.Body>
                            <AlertDialog.Footer>
                                <Button slot={"close"} variant={"tertiary"}>Cancel</Button>
                                <Button slot={"close"} variant={"danger"} onClick={() => deleteEndpoint()}>Delete</Button>
                            </AlertDialog.Footer>
                        </AlertDialog.Dialog>
                    </AlertDialog.Backdrop>
                </AlertDialog>
            }
        </div>
    )
}