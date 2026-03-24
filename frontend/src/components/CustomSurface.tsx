import type {ReactNode} from "react";
import {Surface} from "@heroui/react";

interface CustomSurfaceProps {
    title: string,
    children: ReactNode,
    className?: string
}

export default function CustomSurface(props: CustomSurfaceProps) {
    return (
        <Surface className={`p-8 rounded-3xl ${props.className}`}>
            <p className={"mb-4"}>{props.title}</p>
            { props.children }
        </Surface>
    )
}