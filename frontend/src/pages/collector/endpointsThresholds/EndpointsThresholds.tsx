import {Button, Table} from "@heroui/react";
import {useEffect, useState} from "react";
import axios from "axios";
import type {MetricsThresholdsInterface} from "../../../types/MetricsThresholdsInterface.ts";
import {TableEmptyContent} from "../../../components/TableEmptyContent.tsx";
import {TableActions} from "../../../components/TableActions.tsx";
import type {EndpointsThresholdsInterface} from "../../../types/EndpointsThresholdsInterface.ts";
import {CustomDialog} from "../../../components/CustomDialog.tsx";

export interface EndpointsThresholdsProps {
    collector_id: number
}

export function EndpointsThresholds(props: EndpointsThresholdsProps) {
    const [thresholds, setThresholds] = useState<EndpointsThresholdsInterface[]>([])

    const [isAddOpen, setIsAddOpen] = useState<boolean>(false)
    const [isEditOpen, setIsEditOpen] = useState<boolean>(false)
    const [isDeleteOpen, setIsDeleteOpen] = useState<boolean>(false)

    // TODO
    const [editingThreshold, setEditingThreshold] = useState<MetricsThresholdsInterface | null>(null)
    const [deletingThreshold, setDeletingThreshold] = useState<MetricsThresholdsInterface | null>(null)

    const url = `http://localhost:5000/collector/${props.collector_id}/endpoints_thresholds`

    useEffect(() => {
        axios
            .get(url)
            .then(resp => {
                const val: EndpointsThresholdsInterface[] =
                    resp.data.map((t: EndpointsThresholdsInterface) => (t))
                setThresholds(val)
            })
    }, [url]);

    return (
        <div>
            <h4>Notification Thresholds for API Endpoints</h4>
            <div className={"flex flex-col gap-4"}>
                <Table>
                    <Table.Content aria-label={"Endpoints Notification Thresholds"}>
                        <Table.Header>
                            <Table.Column isRowHeader>Type</Table.Column>
                            <Table.Column>Component Name</Table.Column>
                            <Table.Column>Value</Table.Column>
                            <Table.Column>Actions</Table.Column>
                        </Table.Header>
                        <Table.Body renderEmptyState={() => (
                            <TableEmptyContent text={["No thresholds found", "Start by adding your first threshold"]} icon={"tray"}/>
                        )}>
                            {
                                thresholds.map((t, i) => (
                                    <Table.Row key={i}>
                                        <Table.Cell>{t.metric_type}</Table.Cell>
                                        <Table.Cell>{t.component_name || "-"}</Table.Cell>
                                        <Table.Cell>{t.value}</Table.Cell>
                                        <TableActions
                                            deleteOnClick={() => {
                                                setDeletingThreshold(t)
                                                setIsDeleteOpen(true)
                                            }}
                                            showEdit={true}
                                            editOnClick={() => {
                                                setEditingThreshold(t)
                                                setIsEditOpen(true)
                                            }}
                                        />
                                    </Table.Row>
                                ))
                            }
                        </Table.Body>
                    </Table.Content>
                    <Table.Footer>
                        <p className={"text-sm font-light"}>{thresholds.length} results</p>
                    </Table.Footer>
                </Table>
                <Button onClick={() => setIsAddOpen(true)}>Add new</Button>

                <CustomDialog
                    title={"Add Threshold"}
                    body={<>TODO</>}
                    action={"add"}
                    onConfirm={() => {}}
                    isOpen={isAddOpen}
                    setIsOpen={setIsAddOpen}
                    showFooter={false}
                />
                <CustomDialog
                    title={"Edit Threshold"}
                    body={<>TODO</>}
                    action={"edit"}
                    onConfirm={() => {}}
                    isOpen={isEditOpen}
                    setIsOpen={setIsEditOpen}
                    showFooter={false}
                />
                <CustomDialog
                    title={"Delete Threshold"}
                    body={ <p>You will no longer be receiving notifications after exceeding these limits</p> }
                    action={"add"}
                    onConfirm={() => {}}
                    isOpen={isAddOpen}
                    setIsOpen={setIsAddOpen}
                    showFooter={false}
                />
            </div>
        </div>
    )
}
