import CustomSurface from "../components/CustomSurface.tsx";
import {SettingsTimeLimit} from "../components/settings/SettingsTimeLimit.tsx";

export default function Settings() {

    return (
        <main>
            <h1>Settings</h1>
            <div className={"grid grid-cols-2 gap-4"}>
                <CustomSurface title={"Metrics settings"} className={"flex flex-col gap-20"}>
                    <SettingsTimeLimit/>
                </CustomSurface>
                <CustomSurface title={"TODO"}>
                    <p className={"text-gray-500"}>//TODO</p>
                </CustomSurface>
            </div>
        </main>
    )
}

