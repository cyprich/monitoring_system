import {
    CartesianGrid,
    XAxis,
    YAxis,
    Legend,
    Tooltip,
    ReferenceLine,
    Label, Area, AreaChart,
} from "recharts";
import {RechartsDevtools} from "@recharts/devtools";
import colors from "tailwindcss/colors"
import {Tray} from "@gravity-ui/icons";

interface LineChartProps {
    name: string,
    data: LineChartData[],
    keys: string[],
    unit: string,
    max_y?: number | undefined,
    threshold?: number | undefined,
    showTooltipPercent?: boolean,
}

interface LineChartData {
    time: string,
    [value: string]: number | string
}

// TODO colors, move to separate file
const lineColors = [
    colors.blue[500],
    colors.orange[500],
    colors.green[500],
    colors.yellow[500],
    colors.teal[500],
    colors.fuchsia[500],
    colors.violet[500],
    colors.rose[500],
]

function getColor(index: number): string {
    return lineColors[index % lineColors.length]
}

export default function CustomChart({name, data, keys, unit, max_y, threshold, showTooltipPercent}: LineChartProps) {
    if (data.length === 0) return Empty(name)
    unit = unit || "";

    return (
        <div>
            <h3 className={"text-center wrap-anywhere mx-4"}>{name}</h3>
            <AreaChart style={{width: "90%", aspectRatio: "1.618"}} responsive data={data}>
                <defs>
                    {
                        keys.map((k, i) => (
                            <linearGradient id={`grad-${k}`} x1={0} y1={0} x2={0} y2={1} key={i}>
                                <stop offset={"10%"} stopColor={getColor(i)} stopOpacity={0.30}/>
                                <stop offset={"90%"} stopColor={getColor(i)} stopOpacity={0.05}/>
                            </linearGradient>
                        ))
                    }
                </defs>

                {
                    keys.map((k, i) => (
                        <Area name={k} dataKey={k.toLowerCase()} type={"monotone"} animationDuration={0} dot={false}
                              fill={`url(#grad-${k})`} stroke={getColor(i)} stopOpacity={1} strokeWidth={1.5}/>
                    ))
                }

                {
                    threshold && <ReferenceLine y={threshold} stroke={"red"} strokeDasharray={"2 5"}>
                        <Label value={`Threshold: ${threshold}${unit}`} fill={colors.red[500]} position={"top"}/>
                    </ReferenceLine>
                }

                <CartesianGrid stroke={colors.gray[500]} opacity={0.25} vertical={false}/>
                <XAxis dataKey={"time"} niceTicks={'adaptive'} tickLine={false} minTickGap={60} />
                <YAxis domain={max_y ? [0, max_y] : undefined} tickLine={false} width={100} axisLine={false} unit={unit} />
                {
                    keys.length > 1 && <Legend/>
                }
                <Tooltip formatter={(val) => {
                    const formattedNumber = `${Number(val).toFixed(2)}`;

                    const percentText =
                        (showTooltipPercent && max_y && val !== undefined)
                            ? `(${((Number(val) / max_y)*100).toFixed(0)}%)`
                            : ""

                    return `${formattedNumber}${unit} ${percentText}`
                }}/>
                <RechartsDevtools/>
            </AreaChart>
        </div>
    )
}

function Empty(name: string) {
    return (
        <div className={"flex items-center justify-center p-8"}>

            <div className={"flex flex-col gap-2 items-center justify-center w-full aspect-[1.618] bg-zinc-100 p-8 rounded-xl"}>
                <p className={"wrap-anywhere text-lg text-center font-semibold"}>{name}</p>
                <Tray className={"size-16 text-zinc-500/90"}/>
                <p className={"font-light"}>No data to show</p>
            </div>
        </div>
    )
}