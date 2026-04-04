import type {MetricsThresholdsInterface} from "../../../types/MetricsThresholdsInterface.ts";
import {firstLetterUppercase} from "../../../helpFunctions.ts";
import {AlertDialog, Button} from "@heroui/react";
import axios from "axios";
import {EndpointsForm} from "../endpoints/Form.tsx";

interface DialogProps {
    action: "add" | "edit" | "delete",
    isOpen: boolean,
    setIsOpen: (isOpen: boolean) => void,
    collector_id: number,
    threshold?: MetricsThresholdsInterface,
}

export function Dialog(props: DialogProps) {
    const title = firstLetterUppercase(props.action)

    const url = `http://localhost:5000/metrics_thresholds/${props.threshold?.id}`
    function deleteThreshold() {
        axios.delete(url).then().catch(e => console.error(e))
    }

    return (
        props.isOpen && <AlertDialog>
            <AlertDialog.Backdrop isOpen={props.isOpen} onOpenChange={props.setIsOpen}>
                <AlertDialog.Container size={"lg"}>
                    <AlertDialog.Dialog>
                        <AlertDialog.Header>
                            <h4>{title} Threshold{props.action === "delete" && "?"}</h4>
                            <AlertDialog.CloseTrigger/>
                        </AlertDialog.Header>
                        {
                            props.action !== "delete"
                                ? <AlertDialog.Body>
                                    {/*<EndpointsForm */}
                                    {/*    action={props.action} */}
                                    {/*    collectorId={ props.action === "edit" ? props.threshold : undefined} */}
                                    {/*    setIsOpen={} */}
                                    {/*    refresh={}*/}
                                    {/*/>*/}
                                </AlertDialog.Body>
                                : <>
                                    <AlertDialog.Body>
                                        <p>You will no longer receiving notifications after exceeding these limits</p>
                                        <p>This action is not reversible</p>
                                    </AlertDialog.Body>
                                    <AlertDialog.Footer>
                                        <Button slot={"close"} variant={"tertiary"}>Cancel</Button>
                                        <Button
                                            slot={"close"}
                                            variant={"danger"}
                                            onClick={() => deleteThreshold()}
                                        >Delete</Button>
                                    </AlertDialog.Footer>
                                </>
                        }
                    </AlertDialog.Dialog>
                </AlertDialog.Container>
            </AlertDialog.Backdrop>
        </AlertDialog>
    )

}