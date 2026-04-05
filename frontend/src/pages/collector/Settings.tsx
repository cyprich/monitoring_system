import type {Collector} from "../../types/Collector.ts";
import SettingsGeneralSection from "../../components/settings/SettingsGeneralSection.tsx";
import ConfirmableInput from "../../components/ConfirmableInput.tsx";
import axios from "axios";
import {SettingsTimeLimit} from "../../components/settings/SettingsTimeLimit.tsx";
import {SettingsResolution} from "../../components/settings/SettingsResolution.tsx";
import {MetricsThresholds} from "./metricsThresholds/MetricsThresholds.tsx";
import {EndpointsThresholds} from "./endpointsThresholds/EndpointsThresholds.tsx";
import {Separator} from "@heroui/react";

export interface SettingsProps {
    collector: Collector,
    setCollector: (collector: Collector) => void
}

export function Settings(props: SettingsProps) {
    const url = `http://localhost:5000/collector/${props.collector.id}/rename`

    return (
        <div className={"grid grid-cols-[1fr_auto_1fr] gap-16 *:flex *:flex-col *:gap-8 "}>
            <div>
                <SettingsGeneralSection title={"Collector name"}>
                    <ConfirmableInput
                        value={props.collector.name}
                        variant={"secondary"}
                        onConfirm={(newName) => {
                            axios
                                .patch(url, {"name": newName})
                                .then(() => {
                                    props.setCollector({ ...props.collector, name: newName }
                                )})
                                .catch(e => { console.error(e) /* TODO */ })
                        }}
                    />
                </SettingsGeneralSection>
                <Separator/>
                <SettingsTimeLimit/>
                <SettingsResolution/>
            </div>
            <Separator orientation={"vertical"} />
            <div>
                <MetricsThresholds collector_id={props.collector.id}/>
                <Separator/>
                <EndpointsThresholds collector_id={props.collector.id}/>
            </div>
        </div>
    )
}
