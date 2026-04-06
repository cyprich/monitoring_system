import type {ReactNode} from "react";

interface SectionProps {
    title?: string,
    children: ReactNode,
    description?: string,
}

export default function SettingsGeneralSection(props: SectionProps) {
    return (
        <div className={"flex flex-col gap-2"}>
            { props.title && <p className={"font-semibold"}>{props.title}</p> }
            <p className={"custom-description"}>{props.description}</p>
            { props.children }
        </div>
    )
}