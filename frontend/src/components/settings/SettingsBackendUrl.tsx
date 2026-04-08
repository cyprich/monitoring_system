import {Button, FieldError, Form, Input, Label, TextField, Toast, toast} from "@heroui/react";
import SettingsGeneralSection from "./SettingsGeneralSection.tsx";
import {useState} from "react";
import {
    getBaseUrl, setBaseUrl 
} from "../../helpFunctions.ts";
import axios from "axios";
import {TriangleExclamationFill} from "@gravity-ui/icons";

export function SettingsBackendUrl() {
    // const [apiValue, setApiValue] = useState<string>(getBaseUrlWithoutExtension())
    // const [wsValue, setWsValue] = useState<string>(getWebsocketBaseUrlWithoutExtension())
    const [apiValue, setApiValue] = useState<string>(getBaseUrl())
    const [wsValue, setWsValue] = useState<string>(getBaseUrl())

    function submit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault()
        setBaseUrl(apiValue)
        // setWebsocketBaseUrl(wsValue)

        axios
            .get(apiValue)
            .then(resp => {
                if (resp.data !== "Hello World from monitoring system backend API!") {
                    customToast("API", false)
                } else {
                    customToast("API", true)
                }
            })
            .catch((e) => {
                customToast("API", false)
                console.error(e)
            })

        const socket = new WebSocket(wsValue + "/api/v1/ws/hello")

        socket.addEventListener("open", () => {
            customToast("WebSocket", true)
            socket.close()
        })

        socket.addEventListener("error", (e) => {
            customToast("WebSocket", false)
            console.error(e)
        })

    }

    function reset() {
        // setApiValue(getBaseUrlWithoutExtension())
        // setWsValue(getWebsocketBaseUrlWithoutExtension())
    }

    function customToast(type: "API" | "WebSocket", success: boolean) {
        if (success) {
            toast.success("Success", {
                description: `${type} Sucessfully updated`,
                timeout: 3_000 //3 seconds
            })
        } else {
            toast.danger(`${type} Not Reachable!`, {
                description:
                    `Test connection to ${type} failed. 
                    You will not be able to ${type === "API" ? "get data" : "get live updates"}. 
                    See console for more details`,
                indicator: <TriangleExclamationFill/>,
                timeout: 15_000  // 15 seconds
            })
        }
    }


    return (
        <SettingsGeneralSection>
            <Toast.Provider/>
            <Form className={"flex flex-col gap-4"} onSubmit={submit}>
                <TextField isRequired>
                    <Label>URL for API</Label>
                    <Input
                        placeholder={"http://localhost:5000"}
                        value={apiValue}
                        onChange={e => (setApiValue(e.target.value))}
                    />
                    <FieldError/>
                </TextField>
                <TextField isRequired>
                    <Label>URL for WebSocket</Label>
                    <Input
                        placeholder={"ws://localhost:5000"}
                        value={wsValue}
                        onChange={e => (setWsValue(e.target.value))}
                    />
                    <FieldError/>
                </TextField>
                <div className={"flex gap-2"}>
                        <Button type={"submit"}>Update</Button>
                        <Button type={"reset"} variant={"secondary"} onClick={() => reset()}>Reset</Button>
                </div>
            </Form>
        </SettingsGeneralSection>
    )
}

