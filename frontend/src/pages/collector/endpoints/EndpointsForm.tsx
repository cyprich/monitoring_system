import type {Endpoint} from "../../../types/Endpoints.ts";
import {useState} from "react";
import {Button, FieldError, Fieldset, Form, Input, TextField, Label} from "@heroui/react";
import {ExclamationShapeFill, Plus, TrashBin} from "@gravity-ui/icons";
import axios from "axios";
import * as React from "react";

export interface EndpointsFormProps {
    action: "add" | "edit"
    collectorId: number
    setIsOpen: (isOpen: boolean) => void
    endpoint?: Endpoint,
}

export function EndpointsForm(props: EndpointsFormProps) {
    const [endpointUrl, setEndpointUrl] = useState<string>(props.endpoint?.url || "")
    const [responseCodeInput, setResponseCodeInput] = useState<string>("")
    const [responseCodes, setResponseCodes] = useState<number[]>(props.endpoint?.expected_codes || [])

    const backendUrl = `http://localhost:5000/collector/${props.collectorId}/endpoints`

    // TODO historical data

    function onSubmit(event: React.FormEvent<HTMLFormElement>) {
        event.preventDefault();

        const endpoint = {
            url: endpointUrl,
            expected_codes: responseCodes,
            method: "Get"
        }

        if (props.action === "add") {
            axios
                .post(backendUrl, { ...endpoint })
                .then()
                .catch((err) => console.error("post error", err))
        }

        if (props.action === "edit") {
            if (props.endpoint?.id === undefined) return

            axios
                .put(backendUrl, { ...endpoint, id: props.endpoint.id })
                .then()
                .catch((err) => console.error("put error", err))
        }

        props.setIsOpen(false)
    }

    return (
        <Form className={"flex flex-col gap-2 mt-2"} onSubmit={onSubmit}>
            <Fieldset>
                <TextField isRequired name={"url"} type={"text"} validate={() => true}>
                    <Label>URL</Label>
                    <Input
                        placeholder={"http://192.168.10.10:80/api"}
                        variant={"secondary"}
                        value={endpointUrl}
                        onChange={e => setEndpointUrl(e.target.value)}
                    />
                    <FieldError/>
                </TextField>
                <TextField name={"responsecodes"} type={"text"} validate={() => true}>
                    <Label>Expected Response Codes</Label>
                    {
                        responseCodes.length === 0 && <span className={"flex gap-1 items-center my-1"}>
                            <ExclamationShapeFill className={"text-danger size-4"}/>
                            <p>Endpoint without Response Codes will always fail</p>
                        </span>
                    }
                    <div className={"flex flex-col gap-2"}>
                        <div className={"flex gap-2"}>
                            <Input
                                variant={"secondary"}
                                type={"number"}
                                className={"w-56"}
                                value={responseCodeInput}
                                onChange={e => setResponseCodeInput(e.target.value)}
                                placeholder={"Response Code Number"}
                            />
                            <Button
                                variant={"tertiary"}
                                className={"aspect-square"}
                                onClick={() => {
                                    const val = Number(responseCodeInput)
                                    if (val === 0 || responseCodes.find(
                                        (c) => (c === val)
                                    )) { return }

                                    setResponseCodes(prev => [ ...prev, Number(val) ])
                                }}
                            ><Plus/></Button>
                        </div>
                        <div className={"flex items-center gap-1"}>
                            <p className={"pr-2"}>Selected Codes:</p>
                            {
                                responseCodes.length === 0 ? "None" :
                                responseCodes.map((c, i) => (
                                    <div
                                        className={"bg-background p-2 w-max rounded-xl relative cursor-pointer group min-w-10"}
                                        key={i}
                                        onClick={() => {
                                            const newList = responseCodes.filter(x => x !== c)
                                            setResponseCodes(newList)
                                        }}
                                    >
                                        <p className={"group-hover:opacity-0 transition-opacity text-center font-semibold"}>{c}</p>
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
                    <Button type={"submit"}>{props.action === "edit" ? "Edit" : "Add"}</Button>
                    <Button
                        type={"reset"}
                        variant={"secondary"}
                        onClick={() => {
                            setEndpointUrl("")
                            setResponseCodes([])
                        }}
                    >Reset</Button>
                </div>
            </Fieldset>
        </Form>
    )
}
