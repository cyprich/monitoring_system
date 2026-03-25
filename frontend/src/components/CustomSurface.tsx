import type {ReactNode} from "react";
import {Surface} from "@heroui/react";

interface CustomSurfaceProps {
    title?: string,
    children: ReactNode,
    className?: string,
    variant?: "default" | "secondary" | "tertiary"
}

export default function CustomSurface(props: CustomSurfaceProps) {
    return (
        <Surface className={`p-8 rounded-3xl`} variant={props.variant}>
            {
                props.title && <h3 className={"mb-4"}>{props.title}</h3>
            }
            <div className={props.className}>
                { props.children }
            </div>
        </Surface>
    )
}