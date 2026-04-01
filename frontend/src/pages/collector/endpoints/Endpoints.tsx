import {useEffect, useState} from "react";
import type {Endpoint, EndpointResult} from "../../../types/Endpoints.ts";
import axios from "axios";
import {
    Button,
    EmptyState,
    Table,
} from "@heroui/react";
import {CircleCheckFill, CircleXmarkFill, TriangleExclamationFill, Pencil, TrashBin, Tray} from "@gravity-ui/icons";
import {Dialog} from "./Dialog.tsx";

export interface EndpointsProps {
    collectorId: number,
    lastEndpointsResults: EndpointResult[]
}

export default function Endpoints(props: EndpointsProps) {
    const [endpoints, setEndpoints] = useState<Array<Endpoint> | null>(null)
    const [toggleRefresh, setToggleRefresh] = useState<boolean>(true)

    const [isAddOpen, setIsAddOpen] = useState<boolean>(false)

    const [isEditOpen, setIsEditOpen] = useState<boolean>(false)
    const [editingEndpoint, setEditingEndpoint] = useState<Endpoint | null>(null)

    const [isDeleteOpen, setIsDeleteOpen] = useState<boolean>(false)
    const [deletingEndpoint, setDeletingEndpoint] = useState<Endpoint | null>(null)

    // TODO url
    const url = `http://localhost:5000/collector/${props.collectorId}/endpoints`

    function refreshEndpoints() {
        setToggleRefresh(false)
        setToggleRefresh(true)
    }

    useEffect(() => {
        axios.get(url).then((resp) => {
            const endpoints: Array<Endpoint> = resp.data.map((e: Endpoint) => {
                return e
            })
            setEndpoints(endpoints)
        })
    }, [url, toggleRefresh]);

    function getEndpointResults(e: Endpoint): EndpointResult | null {
        return props.lastEndpointsResults.find((r) => r.endpoint_id === e.id) || null
    }

    return (
        <div className={"flex flex-col gap-4"}>
            <Table>
                {/*<Table.ScrollContainer>*/}
                <Table.ResizableContainer>
                    <Table.Content aria-label="Endpoints">
                        <Table.Header>
                            {/*<Table.Column isRowHeader>Method</Table.Column>*/}
                            <Table.Column isRowHeader>
                                URL
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Last Request
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Expected Response Codes
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Actions
                                <Table.ColumnResizer/>
                            </Table.Column>
                        </Table.Header>
                        <Table.Body renderEmptyState={() => (
                            <EmptyState
                                className={"flex flex-col justify-center items-center bg-background  rounded-2xl py-8"}>
                                <Tray className={"size-16 opacity-80 mb-2"}/>
                                <span>No Endpoints found</span>
                                <span>Start by adding your first endpoint</span>
                            </EmptyState>
                        )}>
                            {
                                endpoints?.map((e, i) => {
                                    const result = getEndpointResults(e);
                                    const latency: number | null = result && result.latency_microseconds && result.latency_microseconds / 1000

                                    return <Table.Row key={i}>
                                        <Table.Cell>
                                            <p>{e.url}</p>
                                        </Table.Cell>
                                        <Table.Cell>
                                            <div className={"flex flex-col gap-1"}>
                                                <p>Time: {result?.timestamp || "-"}</p>
                                                <div className={"flex gap-1 *:items-center *:flex *:gap-1"}>Status: {
                                                    getEndpointResults(e)?.result
                                                        ? <p className={"text-success"}><CircleCheckFill/> Success</p>
                                                        : <p className={"text-danger"}><CircleXmarkFill/> Fail</p>
                                                }</div>
                                                <p>Latency: {latency || "-"} ms</p>
                                            </div>
                                        </Table.Cell>
                                        <Table.Cell>
                                            {
                                                e.expected_codes.join(", ") ||
                                                <p className={"flex gap-2"}>
                                                    None
                                                    <TriangleExclamationFill className={"text-amber-400 size-5"}/>
                                                </p>
                                            }
                                        </Table.Cell>
                                        <Table.Cell
                                            className={"flex gap-4 items-center *:transition-all " +
                                                "*:w-max *:h-max *:p-2 *:rounded-lg *:cursor-pointer " +
                                                " *:active:scale:95"}
                                        >
                                            <div className={"bg-gray-200 hover:bg-gray-300"}
                                                 onClick={() => {
                                                     setEditingEndpoint(e)
                                                     setIsEditOpen(true)
                                                 }}
                                            >
                                                <Pencil className={"size-5"}/>
                                            </div>
                                            <div
                                                className={"bg-red-100 hover:bg-red-200 " +
                                                    "hover:*:text-red-600"}
                                                onClick={() => {
                                                    setDeletingEndpoint(e)
                                                    setIsDeleteOpen(true)
                                                }}
                                            >
                                                <TrashBin className={"size-5 text-red-500"}/>
                                            </div>
                                        </Table.Cell>
                                    </Table.Row>
                                })
                            }
                        </Table.Body>
                    </Table.Content>
                {/*</Table.ScrollContainer>*/}
                </Table.ResizableContainer>
                <Table.Footer>
                    <p className={"font-light text-sm"}>{endpoints?.length || 0} result{endpoints?.length !== 1 && "s"}</p>
                </Table.Footer>
            </Table>
            <Button onClick={() => setIsAddOpen(true)}>Add new</Button>

            { /* dialogs for add/edit/delete */}
            <Dialog
                collector_id={props.collectorId}
                action={"add"}
                isOpen={isAddOpen}
                setIsOpen={setIsAddOpen}
                refresh={refreshEndpoints}
            />
            <Dialog
                collector_id={props.collectorId}
                action={"edit"}
                endpoint={editingEndpoint!}
                isOpen={isEditOpen}
                setIsOpen={setIsEditOpen}
                refresh={refreshEndpoints}
            />
            <Dialog
                collector_id={props.collectorId}
                action={"delete"}
                endpoint={deletingEndpoint!}
                isOpen={isDeleteOpen}
                setIsOpen={setIsDeleteOpen}
                refresh={refreshEndpoints}
            />
        </div>
    )
}


