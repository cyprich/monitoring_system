import {
    CartesianGrid,
    LineChart,
    XAxis,
    YAxis,
    Legend,
    Tooltip,
    Line,
    ReferenceLine,
    Label,
} from "recharts";
import {RechartsDevtools} from "@recharts/devtools";
import colors from "tailwindcss/colors"

interface LineChartProps {
    name: string,
    data: LineChartData[],
    keys: string[],
    unit: string,
    max_y?: number | undefined,
    threshold?: number | undefined
}

interface LineChartData {
    timestamp: string,
    [value: string]: number | string
}

const lineColors = [
    colors.blue[500],
    colors.orange[400]
]

function getColor(index: number): string {
    return lineColors[index % lineColors.length]
}


export default function CustomLineChart({name, data, keys, unit, max_y, threshold}: LineChartProps) {
    unit = unit || "";

    function tooltipFormatter(value: string): string {
        return `${Number(value).toFixed(2)}${unit}`;
    }


    return (
        <LineChart style={{width: "100%", aspectRatio: "1.618"}} responsive data={data}>
            {
                keys.map((k, i) => (
                    <Line name={k} dataKey={k.toLowerCase()} type={"monotone"} animationDuration={0} stroke={getColor(i)} strokeWidth={1.5} dot={false}/>
                ))
            }

            {
                threshold && <ReferenceLine y={threshold} stroke={"red"} strokeDasharray={"2 5"}>
                    <Label value={`Threshold: ${threshold}${unit}`} fill={colors.red[500]} position={"top"}/>
                </ReferenceLine>
            }

            <CartesianGrid strokeDasharray={"5 5"}/>
            <XAxis dataKey={"timestamp"} niceTicks={'adaptive'}/>
            <YAxis label={{value: `${name} [${unit}]`, dx: -24,  angle: -90}}
                   domain={max_y ? [0, max_y] : undefined} />
            <Legend/>
            <Tooltip formatter={tooltipFormatter}/>
            <RechartsDevtools/>
        </LineChart>
    )
}