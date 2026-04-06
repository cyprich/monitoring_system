import {Button, FieldError, Fieldset, Form, Input, Label, ListBox, Select, TextField} from "@heroui/react";
import * as React from "react";
import {firstLetterUppercase, getBaseUrl} from "../../../helpFunctions.ts";
import {useEffect, useState} from "react";
import {type MetricType, metricTypeUnit, prettyMetricType} from "../../../types/MetricType.ts";
import axios from "axios";
import type {Drive, NetworkInterface} from "../../../types/Collector.ts";
import {ValueAndCountTooltip} from "../../../components/ValueAndCountTooltip.tsx";
import type {MetricsThresholdsInterface} from "../../../types/MetricsThresholdsInterface.ts";

export interface MetricsThresholdsFormProps {
    action: "add" | "edit"
    collector_id: number
    thresholdId?: number
    setIsOpen: (isOpen: boolean) => void
    setThresholds: (f: (prev: MetricsThresholdsInterface[]) => MetricsThresholdsInterface[]) => void
}

export function MetricsThresholdsForm(props: MetricsThresholdsFormProps) {
    const [availableMetricTypes, setAvailableMetricTypes] = useState<MetricType[]>([])
    const [selectedMetricType, setSelectedMetricType] = useState<MetricType | null>(null)

    const [availableComponentNames, setAvailableComponentNames] = useState<string[]>([])
    const [availableDriveNames, setAvailableDriveNames] = useState<string[]>([])
    const [availableNetworksUploadNames, setAvailableNetworksUploadNames] = useState<string[]>([])
    const [availableNetworksDownloadNames, setAvailableNetworksDownloadNames] = useState<string[]>([])

    const [selectedComponentName, setSelectedComponentName] = useState<string>("")

    const [value, setValue] = useState<string | undefined>(undefined)
    const [count, setCount] = useState<string | undefined>(undefined)

    const url = getBaseUrl() + `/collector/${props.collector_id}/metrics_thresholds/`

    useEffect(() => {
        axios
            .get<MetricType[]>(`${url}/available_metric_types`)
            .then(resp => {
                setAvailableMetricTypes(resp.data)
            })
        axios
           .get<Drive[]>(`${url}/available_drives`)
           .then(resp => {
               setAvailableDriveNames(
                   resp.data.map(d => (d.mountpoint))
               )
           })
        axios
            .get<NetworkInterface[]>(`${url}/available_network_interfaces_upload`)
            .then(resp => (
                setAvailableNetworksUploadNames(
                    resp.data.map(n => n.name)
                )
            ))
        axios
            .get<NetworkInterface[]>(`${url}/available_network_interfaces_download`)
            .then(resp => (
                setAvailableNetworksDownloadNames(
                    resp.data.map(n => n.name)
                )
            ))
    }, [url]);

    function onSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault()
        const result: MetricsThresholdsInterface = {
            id: 0,
            collector_id: props.collector_id,
            component_name: selectedComponentName,
            metric_type: selectedMetricType || "",
            value: Number(value),
            count: Number(count)
        }

        axios
            .post<MetricsThresholdsInterface>(getBaseUrl() + "/metrics_thresholds", { ...result })
            .then(resp => {
                props.setThresholds(prev => [...prev, resp.data])
            })
            .catch(e => console.error(e));

        props.setIsOpen(false)
    }


    return (
        <Form className={"flex flex-col gap-2 mt-2"} onSubmit={onSubmit}>
            <Fieldset>
                <Select
                    placeholder={"Select Type"}
                    isRequired
                    selectionMode={"single"}
                    selectedKey={selectedMetricType}
                    onSelectionChange={key => {
                        if (typeof key === "string") {
                            const val: MetricType = key as MetricType;
                            setSelectedMetricType(val)
                            setSelectedComponentName("")
                            setAvailableComponentNames([])

                            if (val === "drive_used_space") {
                                setAvailableComponentNames(availableDriveNames)
                            } else if (val === "network_upload") {
                                setAvailableComponentNames(availableNetworksUploadNames)
                            } else if (val === "network_download") {
                                setAvailableComponentNames(availableNetworksDownloadNames)
                            }
                        } else {
                            setSelectedMetricType(null)
                        }
                    }}
                >
                    <Label>Type of Metric</Label>
                    <Select.Trigger>
                        <Select.Value/>
                        <Select.Indicator/>
                    </Select.Trigger>
                    <Select.Popover>
                        <ListBox>
                            {
                                availableMetricTypes.map(t => (
                                    <ListBox.Item id={t} key={t} textValue={prettyMetricType(t)}>
                                        {prettyMetricType(t)}
                                        <ListBox.ItemIndicator/>
                                    </ListBox.Item>
                                ))
                            }
                        </ListBox>
                    </Select.Popover>
                </Select>
                {
                    ( selectedMetricType === "drive_used_space" ||
                    selectedMetricType === "network_upload" ||
                    selectedMetricType === "network_download") &&

                    <Select
                        placeholder={"Select Component"}
                        isRequired
                        selectedKey={selectedComponentName}
                        onSelectionChange={key => {
                            if (typeof key === "string") {
                                setSelectedComponentName(key)
                            } else {
                                setSelectedComponentName("")
                            }
                        }}
                    >
                        <Label>Component</Label>
                        <Select.Trigger>
                            <Select.Value/>
                            <Select.Indicator/>
                        </Select.Trigger>
                        <Select.Popover>
                            <ListBox>
                                { availableComponentNames.map(n => (
                                    <ListBox.Item id={n} key={n} textValue={n}>
                                        {n}
                                        <ListBox.ItemIndicator/>
                                    </ListBox.Item>
                                )) }
                            </ListBox>
                        </Select.Popover>
                    </Select>
                }
                <TextField isRequired>
                    <Label>Value {
                        selectedMetricType &&
                        `(${metricTypeUnit(selectedMetricType)})`
                    }</Label>
                    <Input
                        placeholder={"Enter value"}
                        type={"number"}
                        value={value}
                        onChange={e => setValue(e.target.value)}
                    />
                    <FieldError/>
                </TextField>
                <TextField isRequired>
                    <Label>Count</Label>
                    <Input
                        placeholder={"Enter count"}
                        type={"number"}
                        value={count}
                        onChange={e => setCount(e.target.value)}
                    />
                    <FieldError/>
                </TextField>
                <ValueAndCountTooltip showStar={false}/>

                <div className={"flex gap-2"}>
                    <Button type={"submit"}>{firstLetterUppercase(props.action)}</Button>
                    <Button
                        type={"reset"}
                        variant={"secondary"}
                        onClick={() => {
                            // setEndpointUrl("")
                            // setResponseCodes([])
                        }}
                    >Reset</Button>
                </div>
            </Fieldset>
        </Form>
    )
}


