# Monitoring System

Part of my bachelor's thesis

## Usage

### API server + Frontend + Database

These parts are designed to run together  
Environment variables are set in the `docker-compose.yaml` file, no need to change anything  
Simple run via `docker compose up -d`

### Collector

Collects data  
Run it on the system you want to monitor

First, you need to set these environment variables

| Variable      | Example value           | Description                     |
| ------------- | ----------------------- | ------------------------------- |
| `API_ADDRESS` | `http://localhost:5000` | IP address or URL of API server |
| `API_PORT`    | `80`                    | Port of API server              |

Running is done via `cargo run --release --bin collector` in the `backend` directory  
You can also use the precompiled binary - [https://github.com/cyprich/monitoring_system/releases/download/v0.1/collector-linux-x86_64](https://github.com/cyprich/monitoring_system/releases/download/v0.1/collector-linux-x86_64) - and run it as `./collector-linux-x86_64`

You can run multiple collectors which will report to one API server
