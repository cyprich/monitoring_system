import {useEffect, useState} from "react";
import type {Endpoint, EndpointResult} from "../../../types/Endpoints.ts";
import axios from "axios";
import {Button, Table} from "@heroui/react";
import {CircleCheckFill, CircleXmarkFill, TriangleExclamationFill} from "@gravity-ui/icons";
import {TableEmptyContent} from "../../../components/TableEmptyContent.tsx";
import {TableActions} from "../../../components/TableActions.tsx";
import {CustomDialog} from "../../../components/CustomDialog.tsx";
import {EndpointsForm} from "./EndpointsForm.tsx";

export interface EndpointsProps {
    collectorId: number,
    lastEndpointsResults: EndpointResult[]
}

export default function Endpoints(props: EndpointsProps) {
    const [endpoints, setEndpoints] = useState<Endpoint[]>([])

    const [isAddOpen, setIsAddOpen] = useState<boolean>(false)
    // const [isEditOpen, setIsEditOpen] = useState<boolean>(false)
    const [isDeleteOpen, setIsDeleteOpen] = useState<boolean>(false)

    // const [editingEndpoint, setEditingEndpoint] = useState<Endpoint | null>(null)
    const [deletingEndpoint, setDeletingEndpoint] = useState<Endpoint | null>(null)

    // TODO url
    const url = `http://localhost:5000/collector/${props.collectorId}/endpoints`

    // TODO REFRESH
    useEffect(() => {
        axios
            .get<Endpoint[]>(url)
            .then(resp => {
                setEndpoints(resp.data)
            })
    }, [url]);

    function getEndpointResults(e: Endpoint): EndpointResult | null {
        return props.lastEndpointsResults.find(r => r.endpoint_id === e.id) || null
    }

    function deleteEndpoint(id: number) {
        axios
            .delete(`${url}/${id}`)
            .then(() => {
                setEndpoints(prev => prev.filter(e => (e.id !== id)))
            })
            .catch(e => console.error(e))
    }

    return (
        <div className={"flex flex-col gap-4"}>
            <Table>
                <Table.ResizableContainer>
                    <Table.Content aria-label="Endpoints">
                        <Table.Header>
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
                            <TableEmptyContent text={["No endpoints found", "Start by adding your first endpoint"]} icon={"tray"}/>
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
                                                <p>Time: {result?.time || "-"}</p>
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
                                                e.expected_codes?.join(", ") ||
                                                <p className={"flex gap-2"}>
                                                    None
                                                    <TriangleExclamationFill className={"text-amber-400 size-5"}/>
                                                </p>
                                            }
                                        </Table.Cell>
                                        <TableActions
                                            deleteOnClick={() => {
                                                setDeletingEndpoint(e)
                                                setIsDeleteOpen(true)
                                            }}
                                            // TODO
                                            // showEdit={true}
                                            // editOnClick={() => {
                                            //     setEditingEndpoint(e)
                                            //     setIsEditOpen(true)
                                            // }}
                                        />
                                    </Table.Row>
                                })
                            }
                        </Table.Body>
                    </Table.Content>
                </Table.ResizableContainer>
                <Table.Footer>
                    <p className={"font-light text-sm"}>{endpoints?.length || 0} result{endpoints?.length !== 1 && "s"}</p>
                </Table.Footer>
            </Table>
            <Button onClick={() => setIsAddOpen(true)}>Add new</Button>

            { /* dialogs */}
            <CustomDialog
                title={"Add Endpoint"}
                body={
                    <EndpointsForm
                        action={"add"}
                        collectorId={props.collectorId}
                        setIsOpen={setIsAddOpen}
                        setEndpoints={setEndpoints}
                    />
                }
                action={"add"}
                onConfirm={() => {}}
                isOpen={isAddOpen}
                setIsOpen={setIsAddOpen}
                showFooter={false}
            />
            {/*<CustomDialog*/}
            {/*    title={"Edit Endpoint"}*/}
            {/*    body={*/}
            {/*        <EndpointsForm*/}
            {/*            action={"edit"}*/}
            {/*            collectorId={props.collectorId}*/}
            {/*            endpoint={ editingEndpoint! }*/}
            {/*            setIsOpen={setIsAddOpen}*/}
            {/*            setEndpoints={setEndpoints}*/}
            {/*        />*/}
            {/*    }*/}
            {/*    action={"edit"}*/}
            {/*    onConfirm={() => {}}*/}
            {/*    isOpen={isEditOpen}*/}
            {/*    setIsOpen={setIsEditOpen}*/}
            {/*    showFooter={false}*/}
            {/*/>*/}
            <CustomDialog
                title={"Delete Endpoint?"}
                body={ <p>Collector will no longer send requests to this endpoint</p> }
                action={"delete"}
                onConfirm={() => {
                    deleteEndpoint(deletingEndpoint!.id)
                }}
                isOpen={isDeleteOpen}
                setIsOpen={setIsDeleteOpen}
                showFooter={true}
            />
        </div>
    )
}


