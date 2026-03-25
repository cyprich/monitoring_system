import {getMetricsLimit, setMetricsLimit} from "../../helpFunctions.ts";
import {type Key, ToggleButton, ToggleButtonGroup} from "@heroui/react";
import SettingsGeneralSection from "./SettingsGeneralSection.tsx";
import {TriangleExclamationFill} from "@gravity-ui/icons";

interface Props {
    showWarning?: boolean
}

export function SettingsMetricsCountSection(props: Props) {
    const values = [50, 100, 250, 500, 1000];

    function keysToNumber(keys: Set<Key>): number {
        const val = keys.values().next().value;
        return Number(val)
    }

    return (
        <>
            <SettingsGeneralSection
                title={"Metrics Count"}
                description={"How many datapoints to show in metrics charts"}
            >
                <ToggleButtonGroup
                    defaultSelectedKeys={[String(getMetricsLimit())]}
                    selectionMode={"single"}
                    onSelectionChange={(keys) => {setMetricsLimit(keysToNumber(keys))}}
                >
                    {
                        values.map((value) => {
                            const val = String(value);
                            return ( <ToggleButton id={val} key={val}>{val}</ToggleButton> )
                        })
                    }
                </ToggleButtonGroup>
            </SettingsGeneralSection>
                {
                    props.showWarning && <div className={"flex items-center gap-1"}>
                        <TriangleExclamationFill className={"size-5 text-yellow-400"}/>
                        <p className={"custom-description"}>This changes the global setting</p>
                    </div>
                }
        </>
    )
}
