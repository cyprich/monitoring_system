import {Tabs} from "@heroui/react";
import type {CollectorProps} from "./Collector.tsx";
import CustomChart from "../../components/CustomChart.tsx";

export default function Metrics({collector, data}: CollectorProps) {
    const className = "grid gap-y-24 mt-8"

    const drives_cols = (collector?.drives?.length || 0) > 4 ? 3 : 2
    const nets_cols = (collector?.network_interfaces?.length || 0) > 4 ? 3 : 2

    return (
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
                <div className={"grid grid-cols-4 mt-8"}>
                    <div className={"col-span-2 col-start-2"}>
                        <CpuChart collector={collector} data={data}/>
                    </div>
                </div>
            </Tabs.Panel>
            <Tabs.Panel id={"mem"}>
                <div className={className} style={{gridTemplateColumns: `repeat(2, 1fr`}}>
                    <RamChart collector={collector} data={data}/>
                    <SwapChart collector={collector} data={data}/>
                </div>
            </Tabs.Panel>
            <Tabs.Panel id={"drives"}>
                <div className={className} style={{gridTemplateColumns: `repeat(${drives_cols}, 1fr)`}}>
                    <DriveCharts collector={collector} data={data}/>
                </div>
            </Tabs.Panel>
            <Tabs.Panel id={"net"}>
                <div className={className} style={{gridTemplateColumns: `repeat(${nets_cols}, 1fr)`}}>
                    <NetworkCharts collector={collector} data={data}/>
                </div>
            </Tabs.Panel>
        </Tabs>
    )
}

function CpuChart(props: CollectorProps) {
    return (
        <CustomChart name={"CPU Usage"} keys={["CPU"]} data={
            props.data.map(i => ({
                time: i.time,
                cpu: i.cpu_usage
            }))
        } unit={"%"} max_y={100} />
    )

}

function RamChart(props: CollectorProps) {
    return (
        <CustomChart
            name={"RAM"}
            keys={["RAM"]}
            data={
                props.data.map(i => ({
                    time: i.time,
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
                props.data.map(i => ({
                    time: i.time,
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
                            props.data.map(metric => {
                                const selected = metric.drives?.find(
                                    a => a.mountpoint == drive.mountpoint
                                )

                                return {
                                    time: metric.time,
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
                            props.data.map(metric => {
                                const selected = metric.network_interfaces?.find(
                                    a => a.name == network.name
                                )

                                return {
                                    time: metric.time,
                                    download: (selected?.download_kb || 0) / 1000,
                                    upload: (selected?.upload_kb || 0) / 1000,
                                }
                            })
                        }
                        unit={"MB"}
                        max_y={10}
                    />
                ))
            }
        </>
    )
}
