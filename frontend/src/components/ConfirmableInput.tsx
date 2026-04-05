import {Input} from "@heroui/react";
import {Check, Xmark} from "@gravity-ui/icons";
import {useState} from "react";


interface Props {
    value: string,
    onConfirm: (value: string) => void,
    variant?: "primary" | "secondary",
    className?: string,
    activateOnHover?: boolean,
    width?: string
}

export default function ConfirmableInput(props: Props) {
    const iconClassName = `size-6 drop-shadow-xs clickable-big`;
    const [isActive, setIsActive] = useState<boolean>(false);

    const [value, setValue] = useState<string>(props.value)

    return (
        <div
            className={"flex items-center gap-4 relative"}
            style={{ width: props.width || "auto" }}
            onMouseEnter={() => setIsActive(true)}
            onMouseLeave={() => setIsActive(false)}
        >
            {
                !props.activateOnHover || (props.activateOnHover && isActive)
                    ?  <Input
                            variant={props.variant}
                            value={value}
                            onChange={(e) => setValue(e.target.value)}
                            className={props.className}
                        />
                    : <p className={`ml-3`}>{value}</p>
            }
            {
                value !== props.value && <div className={"flex gap-2 absolute left-72"}>
                    <Check className={`${iconClassName} hover:text-success`} onClick={() => props.onConfirm(value)}/>
                    <Xmark className={`${iconClassName} hover:text-red-500`} onClick={() => setValue(props.value)}/>
                </div>
            }
        </div>
    )
}