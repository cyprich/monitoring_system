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
    Legend,
} from 'chart.js';

ChartJS.register(
    CategoryScale,
    LinearScale,
    PointElement,
    LineElement,
    Title,
    Tooltip,
    Legend
);

function Line({values}) {
    const options = {
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
                text: 'CPU Usage',
            },
        },
        animation: {
            duration: 0
        }
    };

    const labels = values.map(() => { return '' });

    const data = {
        labels,
        datasets: [
            {
                label: 'CPU 1',
                data: values.map((i) => Number(i)),
                borderColor: colors.blue["400"],
                backgroundColor: colors.blue["200"],
                cubicInterpolationMode: 'monotone',
            },
        ],
    };

    return (
        <div style={{position: "relative", width: "600px", height: "400px"}}>
            <LineChartJS data={data} options={options} />
        </div>
    )
}

export default Line;