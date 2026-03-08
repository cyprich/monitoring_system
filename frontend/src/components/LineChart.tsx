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
    inputData: LineChartData
}

export function LineChart({inputData}: LineChartProps) {
    console.log(inputData)

    const options: ChartOptions<'line'> = {
        responsive: true,
        maintainAspectRatio: false,
        scale: {
            y: {
                min: 0,
                max: 100
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

    const labels= inputData.dataset.data.map((val) => (val.x))
    const values = inputData.dataset.data.map((val) => (val.y))

    const data: ChartData<'line'> = {
        labels,
        datasets: [
            {
                label: `CPU`,
                data: values,
                borderColor: colors.blue["400"],
                backgroundColor: colors.blue["200"],
                cubicInterpolationMode: 'monotone',
            }

        ],
    };

    return (
        <div style={{position: "relative", width: "600px", height: "400px"}}>
            <LineChartJS data={data} options={options} />
        </div>
    )
}
