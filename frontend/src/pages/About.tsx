export default function About() {
    return (
        <main>
            <h1>About</h1>
            <p>This Monitoring system allows you to monitor cloud services in real time</p>
            <p className={"mt-2"}>You can monitor and set up notifications for:</p>
            <ul>
                <li>CPU Usage</li>
                <li>RAM & Swap Usage</li>
                <li>Drive Usage</li>
                <li>Network Interface Usage</li>
                <li>Availability and latency of defined endpoints</li>
                <li>Listening network ports</li>
            </ul>
            <p className={"mt-2"}>Communication between components is done via REST API and WebSockets</p>

            <h4 className={"mt-4"}>Tech stack</h4>
            <p>Backend - Rust, Actix Web, sqlx, reqwest, sysinfo, netstat2</p>
            <p>Frontend - React, TypeScript, TailwindCSS, HeroUI</p>
            <p>Database - PostgreSQL</p>

            <h4 className={"mt-4"}>Author</h4>
            <p>Peter Cyprich, 2026</p>
            <p className={"mt-1"}>This project was made as a part of my bachelor's thesis at</p>
            <p>University of Žilina, Faculty of Management Science and Informatics</p>

            <p className={"mt-4"}>GitHub links</p>
            <ul>
                <li><a
                    href="https://github.com/cyprich"
                    target={"_blank"}
                    className={"aboutpage-link"}
                >Author</a></li>
                <li><a
                    href="https://github.com/cyprich/monitoring_system"
                    target={"_blank"}
                    className={"aboutpage-link"}
                >This project</a></li>
            </ul>
        </main>
    )
}