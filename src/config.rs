use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub maxmind: MaxmindSettings,
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

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let base_path = std::env::current_dir().expect("Failed to determine the current directory");
        let config_dir = base_path.join("config");

        // Initialize our configuration reader
        let settings = config::Config::builder()
            // Add default settings
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .set_default("maxmind.db_path", "data/GeoLite2-City.mmdb")?
            // Add configuration from environment variables with prefix "GEO_"
            .add_source(
                config::Environment::with_prefix("GEO")
                    .prefix_separator("__")
                    .separator("__"),
            )
            .build()?;

        // Convert the configuration values into our Settings type
        settings.try_deserialize()
    }

    pub fn server_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Failed to parse server address")
    }
}