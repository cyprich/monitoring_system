import {Button, Table} from "@heroui/react";
import {useEffect, useState} from "react";
import axios from "axios";
import {TableEmptyContent} from "../../../components/TableEmptyContent.tsx";
import {TableActions} from "../../../components/TableActions.tsx";
import type {EndpointsThresholdsInterface} from "../../../types/EndpointsThresholdsInterface.ts";
import {CustomDialog} from "../../../components/CustomDialog.tsx";
import {EndpointsThresholdsForm} from "./EndpointsThresholdsForm.tsx";
import {getBaseUrl} from "../../../helpFunctions.ts";

export interface EndpointsThresholdsProps {
    collector_id: number
}

export function EndpointsThresholds(props: EndpointsThresholdsProps) {
    const [thresholds, setThresholds] = useState<EndpointsThresholdsInterface[]>([])

    const [isAddOpen, setIsAddOpen] = useState<boolean>(false)
    const [isEditOpen, setIsEditOpen] = useState<boolean>(false)
    const [isDeleteOpen, setIsDeleteOpen] = useState<boolean>(false)

    // const [editingThreshold, setEditingThreshold] = useState<EndpointsThresholdsInterface | null>(null)
    const [deletingThreshold, setDeletingThreshold] = useState<EndpointsThresholdsInterface | null>(null)

    const url = getBaseUrl() + `/collector/${props.collector_id}/endpoints_thresholds_join`

    useEffect(() => {
        axios
            .get<EndpointsThresholdsInterface[]>(url)
            .then(resp => {
                setThresholds(resp.data)
            })
            .catch(e => console.error(e))
    }, [url]);

    function deleteThreshold(threshold_id: number) {
        axios
            .delete(getBaseUrl() + `/endpoints_thresholds/${threshold_id}`)
            .then(() => {
                setThresholds(prev => prev.filter(t => (t.threshold_id !== threshold_id)))
            })
            .catch(e => console.error(e))
    }

    return (
        <div>
            <h4>Notification Thresholds for API Endpoints</h4>
            <div className={"flex flex-col gap-4"}>
                <Table>
                    <Table.Content aria-label={"Endpoints Notification Thresholds"}>
                        <Table.Header>
                            <Table.Column isRowHeader>Endpoint</Table.Column>
                            <Table.Column>Unsuccessful Request Count</Table.Column>
                            <Table.Column>Actions</Table.Column>
                        </Table.Header>
                        <Table.Body renderEmptyState={() => (
                            <TableEmptyContent text={["No thresholds found", "Start by adding your first threshold"]}
                                               icon={"tray"}/>
                        )}>
                            {
                                thresholds.map(t => (
                                    <Table.Row key={t.threshold_id}>
                                        <Table.Cell>{t.url}</Table.Cell>
                                        <Table.Cell>{t.threshold_value}</Table.Cell>
                                        <TableActions
                                            deleteOnClick={() => {
                                                setDeletingThreshold(t)
                                                setIsDeleteOpen(true)
                                            }}
                                            // TODO
                                            // showEdit={true}
                                            // editOnClick={() => {
                                            //     setEditingThreshold(t)
                                            //     setIsEditOpen(true)
                                            // }}
                                        />
                                    </Table.Row>
                                ))
                            }
                        </Table.Body>
                    </Table.Content>
                    <Table.Footer>
                        <p className={"text-sm font-light"}>{thresholds.length} result{thresholds.length !== 1 && "s"}</p>
                    </Table.Footer>
                </Table>
                <Button onClick={() => setIsAddOpen(true)}>Add new</Button>

                <CustomDialog
                    title={"Add Threshold for Endpoint"}
                    body={ <EndpointsThresholdsForm
                        action={"add"}
                        collector_id={props.collector_id}
                        setIsOpen={setIsAddOpen}
                        setThresholds={setThresholds}
                    /> }
                    action={"add"}
                    onConfirm={() => {}}
                    isOpen={isAddOpen}
                    setIsOpen={setIsAddOpen}
                    showFooter={false}
                />
                <CustomDialog
                    title={"Edit Threshold for Endpoint"}
                    body={<>TODO</>}
                    action={"edit"}
                    onConfirm={() => {}}
                    isOpen={isEditOpen}
                    setIsOpen={setIsEditOpen}
                    showFooter={false}
                />
                <CustomDialog
                    title={"Delete Threshold for Endpoint?"}
                    body={ <p>You will no longer be receiving notifications after exceeding these limits</p> }
                    action={"delete"}
                    onConfirm={() => { deleteThreshold(deletingThreshold?.threshold_id || 0) }}
                    isOpen={isDeleteOpen}
                    setIsOpen={setIsDeleteOpen}
                    showFooter={true}
                />
            </div>
        </div>
    )
}