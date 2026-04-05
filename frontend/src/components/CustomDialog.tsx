import type {ReactNode} from "react";
import {AlertDialog, Button} from "@heroui/react";
import {firstLetterUppercase} from "../helpFunctions.ts";

export interface CustomDialogProps {
    title: string
    body: ReactNode
    action: "add" | "edit" | "delete"
    onConfirm: () => void
    isOpen: boolean
    setIsOpen: (isOpen: boolean) => void
    showFooter: boolean
}

export function CustomDialog(props: CustomDialogProps) {
    return (
        <AlertDialog>
            <AlertDialog.Backdrop isOpen={props.isOpen} onOpenChange={props.setIsOpen}>
                <AlertDialog.Container size={"lg"}>
                    <AlertDialog.Dialog>
                        <AlertDialog.Header>
                            <h4>{props.title}</h4>
                            <AlertDialog.CloseTrigger/>
                        </AlertDialog.Header>
                            <AlertDialog.Body>
                                { props.body }
                                { props.action === "delete" && <p>This action is not reversible</p> }
                            </AlertDialog.Body>
                        {
                            props.showFooter &&
                                <AlertDialog.Footer>
                                    <Button slot={"close"} variant={"tertiary"}>Cancel</Button>
                                    <Button
                                        slot={"close"}
                                        variant={ props.action === "delete" ? "danger" : "primary" }
                                        onClick={props.onConfirm}
                                    >
                                        { firstLetterUppercase(props.action) }
                                    </Button>
                                </AlertDialog.Footer>
                        }
                    </AlertDialog.Dialog>
                </AlertDialog.Container>
            </AlertDialog.Backdrop>
        </AlertDialog>
    )
}