import {Tabs} from "@heroui/react";
import type {CollectorProps} from "./Collector.tsx";
import CustomChart from "../../components/CustomChart.tsx";

export default function Metrics({collector, data}: CollectorProps) {
    const className = "grid gap-y-24 mt-8"

    return (
        <>
            <Tabs>
                <Tabs.ListContainer>
                    <Tabs.List>
                        <Tabs.Tab id={"cpu"}>
                            CPU
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"mem"}>
                            Memory
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"drives"}>
                            Drives
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                        <Tabs.Tab id={"net"}>
                            Network
                            <Tabs.Indicator/>
                        </Tabs.Tab>
                    </Tabs.List>
                </Tabs.ListContainer>

                <Tabs.Panel id={"cpu"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(2, 1fr)"}}>
                        <CpuChartGlobal collector={collector} data={data}/>
                        <CpuChartCores collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"mem"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(2, 1fr)"}}>
                        <RamChart collector={collector} data={data}/>
                        <SwapChart collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"drives"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(3, 1fr)"}}>
                        <DriveCharts collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
                <Tabs.Panel id={"net"}>
                    <div className={className} style={{gridTemplateColumns: "repeat(3, 1fr)"}}>
                        <NetworkCharts collector={collector} data={data}/>
                    </div>
                </Tabs.Panel>
            </Tabs>
        </>
    )
}
function CpuChartGlobal(props: CollectorProps) {
    return (
        <CustomChart name={"CPU Total Usage"} keys={["CPU"]} data={
            props.data.map((i) => ({
                timestamp: i.timestamp.toLocaleTimeString(),
                cpu: i.cpu_usage_global
            }))
        } unit={"%"} max_y={100} />
    )

}

function CpuChartCores(props: CollectorProps) {
    const cpu_count = props.collector?.cpu_count || 0;
    const keys = Array.from({length: cpu_count}, (_, index) => index.toString())

    return (
        <CustomChart
            name={"CPU Usage Per Core"}
            keys={keys}
            data={
                props.data.map((i) => {
                    const result: {timestamp: string, [index: number]: number | string} = {
                        timestamp: i.timestamp.toLocaleTimeString()
                    }

                    i.cpu_usage_cores.forEach((value, index) => {
                        result[`${index}`] = value
                    })

                    return result
                })
            }
            unit={"%"}
            max_y={100}
            lighter={true}
        />
    )
}

function RamChart(props: CollectorProps) {
    return (
        <CustomChart
            name={"RAM"}
            keys={["RAM"]}
            data={
                props.data.map((i) => ({
                    timestamp: i.timestamp.toLocaleTimeString(),
                    ram: i.used_memory_mb
                }))
            }
            unit={"MB"}
            max_y={props.collector?.total_memory_mb || undefined}
            showTooltipPercent={true}
        />
    )
}

function SwapChart(props: CollectorProps) {
    return (
        <CustomChart
            name={"Swap"}
            keys={["Swap"]}
            data={
                props.data.map((i) => ({
                    timestamp: i.timestamp.toLocaleTimeString(),
                    swap: i.used_swap_mb
                }))
            }
            unit={"MB"}
            max_y={props.collector?.total_swap_mb || undefined}
            showTooltipPercent={true}
        />
    )
}

function DriveCharts(props: CollectorProps) {
    return (
        <>
            {
                props.collector?.drives?.map((drive, i) => (
                    <CustomChart
                        name={`Drive at '${drive.mountpoint}'`}
                        key={i}
                        keys={["Used"]}
                        data={
                            props.data.map((metric) => {
                                const selected = metric.drives?.find(
                                    (a) => a.mountpoint == drive.mountpoint
                                )

                                return {
                                    timestamp: metric.timestamp.toLocaleTimeString(),
                                    used: selected?.used_space_gb || 0
                                }
                            })
                        }
                        unit={"GB"}
                        max_y={drive.capacity_gb}
                        showTooltipPercent={true}
                    />
                ))
            }
        </>
    )
}

function NetworkCharts(props: CollectorProps) {
    return (
        <>
            {
                props.collector?.network_interfaces?.map((network, i) => (
                    <CustomChart
                        name={network.name}
                        key={i}
                        keys={["Download", "Upload"]}
                        data={
                            props.data.map((metric) => {
                                const selected = metric.network_interfaces?.find(
                                    (a) => a.name == network.name
                                )

                                return {
                                    timestamp: metric.timestamp.toLocaleTimeString(),
                                    download: (selected?.download_kb || 0) / 1000,
                                    upload: (selected?.upload_kb || 0) / 1000,
                                }
                            })
                        }
                        unit={"MB"}
                        max_y={10}
                        farColors={true}
                    />
                ))
            }
        </>
    )
}
