import type {ReactNode} from "react";
import {Surface} from "@heroui/react";

interface CustomSurfaceProps {
    title?: string,
    icon?: ReactNode,
    children: ReactNode,
    className?: string,
    variant?: "default" | "secondary" | "tertiary"
    id?: string
}

export default function CustomSurface(props: CustomSurfaceProps) {
    return (
        <Surface
            className={`p-8 rounded-3xl drop-shadow-black-50 drop-shadow-lg`}
            variant={props.variant}
            id={props.id}
        >
            <div className={"flex gap-3"}>
                {
                    props.icon && <div className={"*:size-7 mt-0.5"}>
                        {props.icon}
                    </div>
                }
                {
                    props.title && <h3>{props.title}</h3>
                }
            </div>
            <div className={props.className}>
                { props.children }
            </div>
        </Surface>
    )
}