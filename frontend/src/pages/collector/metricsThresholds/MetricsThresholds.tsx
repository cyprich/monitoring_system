import {Button, Table } from "@heroui/react";
import {useEffect, useState} from "react";
import axios from "axios";
import type {MetricsThresholdsInterface} from "../../../types/MetricsThresholdsInterface.ts";
import {TableEmptyContent} from "../../../components/TableEmptyContent.tsx";
import {TableActions} from "../../../components/TableActions.tsx";
import {CustomDialog} from "../../../components/CustomDialog.tsx";
import {MetricsThresholdsForm} from "./MetricsThresholdsForm.tsx";
import {type MetricType, prettyMetricType} from "../../../types/MetricType.ts";
import {ValueAndCountTooltip} from "../../../components/ValueAndCountTooltip.tsx";
import {getBaseUrl} from "../../../helpFunctions.ts";

export interface MetricsThresholdsProps {
    collector_id: number
}

export function MetricsThresholds(props: MetricsThresholdsProps) {
    const [thresholds, setThresholds] = useState<MetricsThresholdsInterface[]>([])

    const [isAddOpen, setIsAddOpen] = useState<boolean>(false)
    const [isEditOpen, setIsEditOpen] = useState<boolean>(false)
    const [isDeleteOpen, setIsDeleteOpen] = useState<boolean>(false)

    // const [editingThreshold, setEditingThreshold] = useState<MetricsThresholdsInterface | null>(null)
    const [deletingThreshold, setDeletingThreshold] = useState<MetricsThresholdsInterface | null>(null)

    const url = getBaseUrl()

    useEffect(() => {
        axios
            .get<MetricsThresholdsInterface[]>(`${url}/collector/${props.collector_id}/metrics_thresholds`)
            .then(resp => {setThresholds(resp.data)})
            .catch(e => console.error(e))
    }, [url, props.collector_id]);

    function deleteThreshold(id: number) {
        axios
            .delete(`${url}/metrics_thresholds/${id}`)
            .then(() => {
                setThresholds(prev => prev.filter(t => (t.id !== id)))
            })
            .catch(e => console.error(e))
    }

    return (
        <div>
            <h4>Notification Thresholds for Metrics</h4>
            <div className={"flex flex-col gap-4"}>
                <Table>
                    <Table.Content aria-label={"Metrics Notification Thresholds"}>
                        <Table.Header>
                            <Table.Column isRowHeader>Type</Table.Column>
                            <Table.Column>Component Name</Table.Column>
                            <Table.Column>Value <span className={"text-accent"}>*</span></Table.Column>
                            <Table.Column>Count <span className={"text-accent"}>*</span></Table.Column>
                            <Table.Column>Actions</Table.Column>
                        </Table.Header>
                        <Table.Body renderEmptyState={() => (
                            <TableEmptyContent text={["No thresholds found", "Start by adding your first threshold"]} icon={"tray"}/>
                        )}>
                            {
                                thresholds.map((t, i) => (
                                    <Table.Row key={i}>
                                        <Table.Cell>{prettyMetricType(t.metric_type as MetricType)}</Table.Cell>
                                        <Table.Cell className={"wrap-anywhere"}>{t.component_name || ""}</Table.Cell>
                                        <Table.Cell>{t.value}</Table.Cell>
                                        <Table.Cell>{t.count}</Table.Cell>
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
                        <p className={"text-sm font-light"}>{thresholds.length} results</p>
                    </Table.Footer>
                </Table>
                <ValueAndCountTooltip showStar={true} />
                <Button onClick={() => setIsAddOpen(true)}>Add new</Button>
            </div>

            {/* dialogs */}
            <CustomDialog
                title={"Add Threshold for Metrics"}
                body={
                    <MetricsThresholdsForm
                        action={"add"}
                        collector_id={props.collector_id}
                        setIsOpen={setIsAddOpen}
                        setThresholds={setThresholds}
                    />
                }
                action={"add"}
                onConfirm={() => {}}
                isOpen={isAddOpen}
                setIsOpen={setIsAddOpen}
                showFooter={false}
            />
            <CustomDialog
                title={"Edit Threshold for Metrics"}
                body={
                    <MetricsThresholdsForm
                        action={"edit"}
                        collector_id={props.collector_id}
                        // endpoint={ editingThreshold! }
                        setIsOpen={setIsAddOpen}
                        setThresholds={setThresholds}
                    />
                }
                action={"edit"}
                onConfirm={() => {}}
                isOpen={isEditOpen}
                setIsOpen={setIsEditOpen}
                showFooter={false}
            />
            <CustomDialog
                title={"Delete Threshold for Metrics?"}
                body={ <>
                    <p>You will no longer be receiving notifications after exceeding these limits</p>
                </> }
                action={"delete"}
                onConfirm={() => {
                    deleteThreshold(deletingThreshold?.id || -1)
                }}
                isOpen={isDeleteOpen}
                setIsOpen={setIsDeleteOpen}
                showFooter={true}
            />
        </div>
    )
}

