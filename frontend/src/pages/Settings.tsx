import CustomSurface from "../components/CustomSurface.tsx";
import {SettingsMetricsCountSection} from "../components/settings/SettingsMetricsCountSection.tsx";

export default function Settings() {

    return (
        <main>
            <h1>Settings</h1>
            <div className={"grid grid-cols-2 gap-4"}>
                <CustomSurface title={"Metrics settings"} className={"flex flex-col gap-20"}>
                    <SettingsMetricsCountSection/>
                </CustomSurface>
                <CustomSurface title={"TODO"}>
                    <p className={"text-gray-500"}>//TODO</p>
                </CustomSurface>
            </div>
        </main>
    )
}

