import {Input} from "@heroui/react";
import {Check, Xmark} from "@gravity-ui/icons";
import {useState} from "react";


interface Props {
    value: string,
    onConfirm: (value: string) => void,
    variant?: "primary" | "secondary"
}

export default function ConfirmableInput(props: Props) {
    const iconClassName = `size-6 drop-shadow-xs hover:scale-125 active:scale-110 transition-all`;

    const [value, setValue] = useState<string>(props.value)

    return (
        <div className={"flex items-center gap-4"}>
            <Input
                variant={props.variant}
                value={value}
                onChange={(e) => setValue(e.target.value)}
            />
            {
                value !== props.value && <>
                    <Check className={iconClassName} onClick={() => props.onConfirm(value)}/>
                    <Xmark className={iconClassName} onClick={() => setValue(props.value)}/>
                </>
            }
        </div>
    )
}