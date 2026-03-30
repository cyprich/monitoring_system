import {AlertDialog, Button} from "@heroui/react";
import {EndpointsForm} from "./Form.tsx";
import type {Endpoint} from "../../../types/Endpoints.ts";
import axios from "axios";

export interface DialogProps {
    action: "add" | "edit" | "delete",
    isOpen: boolean,
    setIsOpen: (isOpen: boolean) => void,
    collector_id: number,
    endpoint?: Endpoint,
    refresh: () => void,
}

export function Dialog(props: DialogProps) {
    // just formatting - first letter is uppercase
    const title: string = props.action.charAt(0).toUpperCase() + props.action.slice(1)

    function deleteEndpoint(id: number) {
        const url = `http://localhost:5000/collector/${props.collector_id}/endpoints/${id}`
        axios.delete(url).then()
    }

    return (
        props.isOpen && <AlertDialog>
            <AlertDialog.Backdrop isOpen={props.isOpen} onOpenChange={props.setIsOpen}>
                <AlertDialog.Dialog>
                    <AlertDialog.Header>
                        <h4>{title} Endpoint{props.action === "delete" && "?"}</h4>
                        <AlertDialog.CloseTrigger/>
                    </AlertDialog.Header>
                    {
                        props.action !== "delete"
                            ? <AlertDialog.Body>
                                <EndpointsForm
                                    action={props.action}
                                    endpoint={ props.action === "edit" ? props.endpoint : undefined}
                                    collectorId={props.collector_id}
                                    setIsOpen={props.setIsOpen}
                                    refresh={props.refresh}
                                />
                            </AlertDialog.Body>
                            : <>
                                <AlertDialog.Body>
                                    <p>Collector will no longer send requests to this endpoint</p>
                                    <p>This action is not reversible</p>
                                </AlertDialog.Body>
                                <AlertDialog.Footer>
                                    <Button slot={"close"} variant={"tertiary"}>Cancel</Button>
                                    <Button
                                        slot={"close"}
                                        variant={"danger"}
                                        onClick={() => {
                                            deleteEndpoint(props.endpoint?.id || 0)
                                            props.refresh()
                                        }}
                                    >Delete</Button>
                                </AlertDialog.Footer>
                            </>
                    }
                </AlertDialog.Dialog>
            </AlertDialog.Backdrop>
        </AlertDialog>
    )
}