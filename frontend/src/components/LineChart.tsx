import {Line as LineChartJS} from "react-chartjs-2";
import colors from "tailwindcss/colors"

import {
    Chart as ChartJS,
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend, type ChartData, type ChartOptions,
} from 'chart.js';
import type {LineChartData} from "../props/LineChartData.ts";

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
);

type LineChartProps = {
    inputData: LineChartData,
    max_y_scale: number | undefined
}

export function LineChart({inputData, max_y_scale}: LineChartProps) {
    const options: ChartOptions<'line'> = {
        responsive: true,
        maintainAspectRatio: false,
        scale: {
            y: {
                min: 0,
                max: max_y_scale
            }
        },
        plugins: {
            legend: {
                position: 'bottom',
            },
            title: {
                display: true,
                text: inputData.title,
            },
        },
        animation: {
            duration: 0
        }
    };

    const labels= inputData.dataset.data.map((val) => (`${val.x.getHours()}:${val.x.getMinutes()}:${val.x.getSeconds()}`))
    const values = inputData.dataset.data.map((val) => (val.y))

    const data: ChartData<'line'> = {
        labels,
        datasets: [
            {
                label: inputData.dataset.name,
                data: values,
                borderColor: colors.blue["400"],
                backgroundColor: colors.blue["200"],
                cubicInterpolationMode: 'monotone',
            }

        ],
    };

    return (
        <div style={{position: "relative", width: "100%", aspectRatio: "1.618"}}>
            <LineChartJS data={data} options={options} />
        </div>
    )
}
