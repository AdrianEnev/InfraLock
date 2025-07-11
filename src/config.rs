use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub maxmind: MaxmindSettings,
    pub vpn_detector: VpnDetectorSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MaxmindSettings {
    pub db_path: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VpnDetectorSettings {
    pub db_path: PathBuf,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .set_default("maxmind.db_path", "data/GeoLite2-City.mmdb")?
            .set_default("vpn_detector.db_path", "data/vpns/ipv4.txt")?
            .add_source(
                config::Environment::with_prefix("GEO")
                    .prefix_separator("__")
                    .separator("__"),
            )
            .build()?;

        settings.try_deserialize()
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Failed to parse server address")
    }

    pub fn resolve_db_path(&self) -> std::io::Result<PathBuf> {
        if self.maxmind.db_path.is_absolute() {
            Ok(self.maxmind.db_path.clone())
        } else {
            let base_path = std::env::current_dir()?;
            Ok(base_path.join(&self.maxmind.db_path))
        }
    }

    pub fn resolve_vpn_detector_db_path(&self) -> std::io::Result<PathBuf> {
        if self.vpn_detector.db_path.is_absolute() {
            Ok(self.vpn_detector.db_path.clone())
        } else {
            let base_path = std::env::current_dir()?;
            Ok(base_path.join(&self.vpn_detector.db_path))
        }
    }
}