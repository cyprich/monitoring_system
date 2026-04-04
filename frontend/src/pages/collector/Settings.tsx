import type {Collector} from "../../types/Collector.ts";
import SettingsGeneralSection from "../../components/settings/SettingsGeneralSection.tsx";
import ConfirmableInput from "../../components/ConfirmableInput.tsx";
import axios from "axios";
import {SettingsMetricsCountSection} from "../../components/settings/SettingsMetricsCountSection.tsx";

export interface SettingsProps {
    collector: Collector,
    setCollector: (collector: Collector) => void
}

export function Settings(props: SettingsProps) {
    const url = `http://localhost:5000/collector/${props.collector.id}/rename`
    return (
        <>
            <div>
                <SettingsGeneralSection title={"Collector name"}>
                    <ConfirmableInput
                        value={props.collector.name}
                        variant={"secondary"}
                        onConfirm={(newName) => {
                            axios
                                .patch(url, {"name": newName})
                                .then(() => {
                                    props.setCollector({...props.collector, name: newName})
                                }).catch((e) => { console.error(e) /* TODO */ })
                        }}
                    />
                </SettingsGeneralSection>
            </div>
            <div>
                <SettingsMetricsCountSection showWarning={true}/>
            </div>
        </>
    )
}
