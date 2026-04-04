import {Button, Table, Tooltip} from "@heroui/react";
import {useEffect, useState} from "react";
import axios from "axios";
import type {MetricsThresholdsInterface} from "../../../types/MetricsThresholdsInterface.ts";
import {TableEmptyContent} from "../../../components/TableEmptyContent.tsx";
import {TableActions} from "../../../components/TableActions.tsx";
import {Dialog} from "./Dialog.tsx";
import {CircleInfo} from "@gravity-ui/icons";

export interface MetricsThresholdsProps {
    collector_id: number
}

export function MetricsThresholds(props: MetricsThresholdsProps) {
    const [thresholds, setThresholds] = useState<MetricsThresholdsInterface[]>([])

    const [isAddOpen, setIsAddOpen] = useState<boolean>(false)
    const [isEditOpen, setIsEditOpen] = useState<boolean>(false)
    const [isDeleteOpen, setIsDeleteOpen] = useState<boolean>(false)

    const [editingThreshold, setEditingThreshold] = useState<MetricsThresholdsInterface | null>(null)
    const [deletingThreshold, setDeletingThreshold] = useState<MetricsThresholdsInterface | null>(null)

    const url = `http://localhost:5000/collector/${props.collector_id}/metrics_thresholds`

    useEffect(() => {
        axios
            .get(url)
            .then(resp => {
                const val: MetricsThresholdsInterface[] =
                    resp.data.map((t: MetricsThresholdsInterface) => (t))
                setThresholds(val)
            })
    }, [url]);

    return (
        <div>
            <h4>Notification Thresholds for Metrics</h4>
            <div className={"flex flex-col gap-4"}>
                <Table>
                    <Table.Content aria-label={"Metrics Notification Thresholds"}>
                        <Table.Header>
                            <Table.Column isRowHeader>Type</Table.Column>
                            <Table.Column>Component Name</Table.Column>
                            <Table.Column>Value <span className={"text-danger"}>*</span></Table.Column>
                            <Table.Column>Count <span className={"text-danger"}>*</span></Table.Column>
                            <Table.Column>Actions</Table.Column>
                        </Table.Header>
                        <Table.Body renderEmptyState={() => (
                            <TableEmptyContent text={["No thresholds found", "Start by adding your first threshold"]} icon={"tray"}/>
                        )}>
                            {
                                thresholds.map((t, i) => (
                                    <Table.Row key={i}>
                                        <Table.Cell>{t.metric_type} //TODO</Table.Cell>
                                        <Table.Cell>{t.component_name || ""}</Table.Cell>
                                        <Table.Cell>{t.value}</Table.Cell>
                                        <Table.Cell>//TODO</Table.Cell>
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
                <div className={"flex gap-1 items-center *:text-sm -mt-2"}>
                    <p><span className={"text-danger"}>*</span>Value and Count</p>
                    <Tooltip delay={0}>
                        <Tooltip.Trigger>
                            <CircleInfo />
                        </Tooltip.Trigger>
                        <Tooltip.Content showArrow className={"*:whitespace-nowrap"}>
                            <Tooltip.Arrow/>
                            <p>If average goes above <span className={"text-variable-name"}>Value</span> </p>
                            <p>for <span className={"text-variable-name"}>Count</span> consecutive times, </p>
                            <p>you will be notified.</p>
                            <p>Metrics are collected every 5 seconds.</p>
                        </Tooltip.Content>
                    </Tooltip>
                </div>
                <Button onClick={() => setIsAddOpen(true)}>Add new</Button>

                {/* dialogs */}
                <Dialog
                    collector_id={props.collector_id}
                    action={"add"}
                    isOpen={isAddOpen}
                    setIsOpen={setIsAddOpen}
                />
                <Dialog
                    collector_id={props.collector_id}
                    action={"edit"}
                    isOpen={isEditOpen}
                    setIsOpen={setIsEditOpen}
                />
                <Dialog
                    collector_id={props.collector_id}
                    action={"delete"}
                    isOpen={isDeleteOpen}
                    setIsOpen={setIsDeleteOpen}
                    threshold={deletingThreshold!}
                />
            </div>
        </div>
    )
}