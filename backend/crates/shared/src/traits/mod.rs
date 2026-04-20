use std::{collections::HashSet, thread::sleep, time::Duration};

use netstat2::{AddressFamilyFlags, ProtocolFlags, TcpState, get_sockets_info};
use reqwest::StatusCode;

use crate::structs::{
    collector_config::CollectorConfig, collector_info::CollectorInfo, endpoints::Endpoint,
    metrics::Metrics, ports::Port,
};

#[async_trait::async_trait]
pub trait Collector: Send + Sync {
    fn get_info(&self) -> CollectorInfo;
    fn get_metrics(&mut self) -> Metrics;
    async fn get_endpoints(&self) -> Result<Vec<Endpoint>, crate::Error>;

    fn get_id(&self) -> Option<i32>;
    fn set_id(&mut self, id: i32);

    // needed for sending metrics
    fn get_client(&self) -> &reqwest::Client;
    fn get_base_url(&self) -> &str;

    async fn send_metrics(&self, metrics: &Metrics) -> Result<(), crate::Error> {
        let resp = self
            .get_client()
            .post(format!("{}/metrics", self.get_base_url()))
            .json(&metrics)
            .send()
            .await;

        match resp {
            Ok(val) => {
                // TODO change here if making settings feature later
                if val.status() == reqwest::StatusCode::UNAUTHORIZED {
                    return Err(crate::Error::CollectorRequiresID);
                }
            }
            Err(val) => {
                eprintln!("Error: {}", val);
            }
        }

        Ok(())
    }

    async fn try_get_id(&mut self, load_from_file: bool) -> Result<(), crate::Error> {
        if load_from_file {
            let result = self.try_get_id_from_file();
            if let Ok(id) = result {
                self.set_id(id);
                return Ok(());
            }
        }

        let id = self.try_get_new_id_from_api().await?;
        self.set_id(id);

        let mut config = CollectorConfig::load().unwrap_or_default();
        config.id = Some(id);

        let result = config.save();
        if let Err(val) = result {
            eprintln!(
                "Succesfully got new ID, but failed to save Collector config: {}",
                val
            );
        }

        Ok(())
    }

    fn try_get_id_from_file(&self) -> Result<i32, crate::Error> {
        let config = CollectorConfig::load()?;
        match config.id {
            Some(val) => Ok(val),
            None => Err(crate::Error::General(
                "ID not found in local config".to_string(),
            )),
        }
    }

    async fn try_get_new_id_from_api(&self) -> Result<i32, crate::Error> {
        let url = format!("{}/collector/register", self.get_base_url());
        let info = self.get_info();

        let mut tries = 10; // try 10 times

        while tries > 0 {
            let resp = self.get_client().post(&url).json(&info).send().await;
            let result = Self::handle_register_response(resp).await;

            match result {
                Ok(val) => return Ok(val),
                Err(val) => {
                    eprintln!("Error registering: {}, try {}/10", val, 10 - tries + 1)
                }
            }
            tries -= 1;
            sleep(Duration::from_secs(3));
        }

        Err(crate::Error::General(
            "Couldn't register to API server".to_string(),
        ))
    }

    async fn handle_register_response(
        resp: Result<reqwest::Response, reqwest::Error>,
    ) -> Result<i32, crate::Error> {
        match resp {
            Ok(val) => match val.status() {
                StatusCode::CREATED => {
                    let text = val.text().await;
                    match text {
                        Ok(val) => Ok(val.parse::<i32>()?),
                        Err(val) => Err(crate::Error::ReqwestGeneral(val)),
                    }
                }
                // TODO other responses?
                _ => Err(crate::Error::ReqwestFromString(format!(
                    "Invalid HTTP response from server: {}",
                    val.status()
                ))),
            },
            Err(val) => Err(crate::Error::ReqwestGeneral(val)),
        }
    }

    fn get_ports() -> Result<HashSet<Port>, crate::Error> {
        let last_update = chrono::Utc::now();
        let sockets_info = get_sockets_info(
            AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6,
            ProtocolFlags::TCP | ProtocolFlags::UDP,
        )?;

        let mut ports = HashSet::new();

        for s in sockets_info {
            match s.protocol_socket_info {
                netstat2::ProtocolSocketInfo::Tcp(tcp) => {
                    if tcp.state == TcpState::Listen {
                        ports.insert(Port {
                            address: tcp.local_addr.to_string(),
                            port: tcp.local_port,
                            protocol: "TCP".into(),
                            last_update,
                        });
                    }
                }
                netstat2::ProtocolSocketInfo::Udp(udp) => {
                    ports.insert(Port {
                        address: udp.local_addr.to_string(),
                        port: udp.local_port,
                        protocol: "UDP".into(),
                        last_update,
                    });
                }
            }
        }

        Ok(ports)
    }
}
