import {ToggleButton, ToggleButtonGroup} from "@heroui/react";
import SettingsGeneralSection from "./SettingsGeneralSection.tsx";
import {getResolution, keysToNumber, setResolution} from "../../helpFunctions.ts";

export function SettingsResolution() {
    const values = [10, 20, 30, 60, 120, 240];

    return (
        <>
            <SettingsGeneralSection
                title={"Resolution"}
                description={"What number of points to display in the charts"}
            >
                <ToggleButtonGroup
                    defaultSelectedKeys={[String(getResolution())]}
                    selectionMode={"single"}
                    onSelectionChange={(keys) => {setResolution(keysToNumber(keys))}}
                >
                    {
                        values.map((value) => {
                            const val = String(value);
                            return ( <ToggleButton id={val} key={val}>{val}</ToggleButton> )
                        })
                    }
                </ToggleButtonGroup>
            </SettingsGeneralSection>
        </>
    )
}
