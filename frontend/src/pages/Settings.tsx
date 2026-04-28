import CustomSurface from "../components/CustomSurface.tsx";
import {SettingsTimeLimit} from "../components/settings/SettingsTimeLimit.tsx";
import {SettingsResolution} from "../components/settings/SettingsResolution.tsx";

export default function Settings() {

    return (
        <main>
            <h1>Settings</h1>
            {/*<div className={"grid grid-cols-1 gap-4"}>*/}
                <CustomSurface title={"Metrics settings"} className={"flex flex-col gap-4"}>
                    <SettingsTimeLimit/>
                    <SettingsResolution/>
                </CustomSurface>
            {/*</div>*/}
        </main>
    )
}

