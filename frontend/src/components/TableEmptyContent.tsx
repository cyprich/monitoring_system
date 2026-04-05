import {EmptyState} from "@heroui/react";
import {CircleCheckFill, Tray} from "@gravity-ui/icons";

export interface TableEmptyContentProps {
    text: string[]
    icon: "tray" | "check"
}

export function TableEmptyContent(props: TableEmptyContentProps) {
    return (
        <EmptyState
            className={"flex flex-col gap-1 justify-center items-center bg-background rounded-2xl py-12"}>
            {
                props.icon === "tray"
                    ? <Tray className={"size-16 opacity-80 mb-2"}/>
                    : <CircleCheckFill className={"size-16 opacity-80 mb-2"}/>
            }
            {
                props.text.map(t => (
                    <span>{t}</span>
                ))
            }
        </EmptyState>
    )
}
