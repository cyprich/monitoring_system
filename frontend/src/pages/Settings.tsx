import CustomSurface from "../components/CustomSurface.tsx";
import {SettingsTimeLimit} from "../components/settings/SettingsTimeLimit.tsx";
import {SettingsBackendUrl} from "../components/settings/SettingsBackendUrl.tsx";
import {SettingsResolution} from "../components/settings/SettingsResolution.tsx";

export default function Settings() {

    return (
        <main>
            <h1>Settings</h1>
            <div className={"grid grid-cols-2 gap-4"}>
                <CustomSurface title={"Metrics settings"} className={"flex flex-col gap-4"}>
                    <SettingsTimeLimit/>
                    <SettingsResolution/>
                </CustomSurface>
                <CustomSurface title={"Backend URL"}>
                    <SettingsBackendUrl/>
                </CustomSurface>
            </div>
        </main>
    )
}

