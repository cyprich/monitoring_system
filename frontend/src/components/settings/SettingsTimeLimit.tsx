import {ToggleButton, ToggleButtonGroup} from "@heroui/react";
import SettingsGeneralSection from "./SettingsGeneralSection.tsx";
import {getTimeLimit, keysToNumber, setTimeLimit} from "../../helpFunctions.ts";

export function SettingsTimeLimit() {
    const values = [1, 3, 6, 12, 24];

    return (
        <>
            <SettingsGeneralSection
                title={"Time Limit"}
                description={"What time period (hours) to display in the charts"}
            >
                <ToggleButtonGroup
                    defaultSelectedKeys={[String(getTimeLimit())]}
                    selectionMode={"single"}
                    onSelectionChange={(keys) => {setTimeLimit(keysToNumber(keys))}}
                >
                    {
                        values.map((value) => {
                            const val = String(value);
                            return ( <ToggleButton id={val} key={val}>{val}h</ToggleButton> )
                        })
                    }
                </ToggleButtonGroup>
            </SettingsGeneralSection>
        </>
    )
}
