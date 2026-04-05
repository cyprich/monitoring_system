import {Button, Description, FieldError, Fieldset, Form, Input, Label, ListBox, Select, TextField} from "@heroui/react";
import * as React from "react";
import {firstLetterUppercase} from "../../../helpFunctions.ts";
import {useEffect, useState} from "react";
import axios from "axios";
import type {Endpoint} from "../../../types/Endpoints.ts";
import type {EndpointsThresholdsInterface} from "../../../types/EndpointsThresholdsInterface.ts";

export interface EndpointsThresholdsFormProps {
    action: "add" | "edit"
    collectorId: number
    thresholdId?: number
    setIsOpen: (isOpen: boolean) => void
    setThresholds: (f: (prev: EndpointsThresholdsInterface[]) => EndpointsThresholdsInterface[]) => void
}

export function EndpointsThresholdsForm(props: EndpointsThresholdsFormProps) {
    const [availableEndpoints, setAvailableEndpoints] = useState<Endpoint[]>([])
    const [endpointUrl, setEndpointUrl] = useState<string | null>(null)
    const [count, setCount] = useState<string | null>(null)

    // TODO URL
    const url = `http://localhost:5000/collector/${props.collectorId}/endpoints_thresholds`

    useEffect(() => {
        axios
            .get<Endpoint[]>(`${url}/available_endpoints`)
            .then(resp => {
                setAvailableEndpoints(resp.data)
            })
    }, [url]);

    function onSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault()
        if (endpointUrl === null) return
        if (count === null) return

        const endpoint = availableEndpoints.find(e => e.url === endpointUrl)
        if (endpoint === undefined) return

        const result = {
            id: 0,
            endpoint_id: endpoint.id,
            value: Number(count)
        }

        // TODO url
        axios
            .post("http://localhost:5000/endpoints_thresholds", { ...result })
            .then(resp => {
                // TODO this chould be done better
                const endpoint = availableEndpoints.find(e => e.id == resp.data.endpoint_id)!
                const newData: EndpointsThresholdsInterface = {
                    collector_id: props.collectorId,
                    endpoint_id: endpoint.id,
                    expected_codes: endpoint.expected_codes,
                    threshold_id: resp.data.id,
                    threshold_value: Number(count),
                    url: endpoint.url,
                }
                props.setThresholds(prev => [...prev, newData])
            })
            .catch(e => console.error(e));

        props.setIsOpen(false)
    }

    return (
        <Form className={"flex flex-col gap-2 mt-2"} onSubmit={onSubmit}>
            <Fieldset>
                <Select
                    placeholder={"Select Endpoint"}
                    isRequired
                    selectionMode={"single"}
                    selectedKey={endpointUrl}
                    onSelectionChange={key => {
                        if (typeof key === "string") {
                            setEndpointUrl(key)
                        } else {
                            setEndpointUrl(null)
                        }
                    }}
                >
                    <Label>Endpoint</Label>
                    <Select.Trigger>
                        <Select.Value/>
                        <Select.Indicator/>
                    </Select.Trigger>
                    <Select.Popover>
                        <ListBox>
                            {
                                availableEndpoints.map(e => (
                                    <ListBox.Item id={e.url} key={e.url} textValue={e.url}>
                                        {e.url}
                                        <ListBox.ItemIndicator/>
                                    </ListBox.Item>
                                ))
                            }
                        </ListBox>
                    </Select.Popover>
                </Select>
                <TextField isRequired>
                    <Label>Unsuccessful Request Count</Label>
                    <Description>If request fails this amount of times, you will be notified</Description>
                    <Description>Endpoints are tested every 10 seconds</Description>
                    <Input
                        placeholder={"Enter Count"}
                        type={"number"}
                        value={count || ""}
                        onChange={e => setCount(e.target.value)}
                    />
                    <FieldError/>
                </TextField>
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


