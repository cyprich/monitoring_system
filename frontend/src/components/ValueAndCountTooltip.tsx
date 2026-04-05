import {Tooltip} from "@heroui/react";
import {CircleInfo} from "@gravity-ui/icons";

interface CountAndValueTooltipProps {
    showStar: boolean
}

export function ValueAndCountTooltip(props: CountAndValueTooltipProps) {
    return (
        <div className={"flex gap-1 items-center *:text-sm -mt-2"}>
            <p>
                { props.showStar && <span className={"text-accent"}>*</span> }
                Value and Count
            </p>
            <Tooltip delay={0}>
                <Tooltip.Trigger>
                    <CircleInfo />
                </Tooltip.Trigger>
                <Tooltip.Content showArrow className={"*:whitespace-nowrap"}>
                    <Tooltip.Arrow/>
                    <p>If average goes above <span className={"text-variable-name"}>Value</span> </p>
                    <p>for <span className={"text-variable-name"}>Count</span> consecutive times, </p>
                    <p>you will be notified.</p>
                    <p>Metrics are collected every 5 seconds.</p>
                </Tooltip.Content>
            </Tooltip>
        </div>
    )

}