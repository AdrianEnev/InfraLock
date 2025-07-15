use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub maxmind: MaxmindSettings,
    pub vpn_detector: VpnDetectorSettings,
    pub proxy_detector: ProxyDetectorSettings,
    pub tor_detector: TorDetectorSettings,
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

#[derive(Debug, Deserialize, Clone)]
pub struct ProxyDetectorSettings {
    pub http_db_path: PathBuf,
    pub socks4_db_path: PathBuf,
    pub socks5_db_path: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TorDetectorSettings {
    pub db_path: PathBuf,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            server: ServerSettings {
                host: "0.0.0.0".to_string(),
                port: 3000,
            },
            maxmind: MaxmindSettings {
                db_path: PathBuf::from("data/GeoLite2-City.mmdb"),
            },
            vpn_detector: VpnDetectorSettings {
                db_path: PathBuf::from("data/vpns/ipv4.txt"),
            },
            proxy_detector: ProxyDetectorSettings {
                http_db_path: PathBuf::from("data/proxies/http.txt"),
                socks4_db_path: PathBuf::from("data/proxies/socks4.txt"),
                socks5_db_path: PathBuf::from("data/proxies/socks5.txt"),
            },
            tor_detector: TorDetectorSettings {
                db_path: PathBuf::from("data/tor/exit-addresses.txt"),
            },
        }
    }
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 3000)?
            .set_default("maxmind.db_path", "data/GeoLite2-City.mmdb")?
            .set_default("vpn_detector.db_path", "data/vpns/ipv4.txt")?
            .set_default("proxy_detector.http_db_path", "data/proxies/http.txt")?
            .set_default("proxy_detector.socks4_db_path", "data/proxies/socks4.txt")?
            .set_default("proxy_detector.socks5_db_path", "data/proxies/socks5.txt")?
            .set_default("tor_detector.db_path", "data/tor/exit-addresses.txt")?
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

    pub fn resolve_tor_detector_db_path(&self) -> std::io::Result<PathBuf> {
        if self.tor_detector.db_path.is_absolute() {
            Ok(self.tor_detector.db_path.clone())
        } else {
            let base_path = std::env::current_dir()?;
            Ok(base_path.join(&self.tor_detector.db_path))
        }
    }

    pub fn resolve_proxy_detector_db_paths(&self) -> std::io::Result<(PathBuf, PathBuf, PathBuf)> {
        let http_db_path = if self.proxy_detector.http_db_path.is_absolute() {
            self.proxy_detector.http_db_path.clone()
        } else {
            let base_path = std::env::current_dir()?;
            base_path.join(&self.proxy_detector.http_db_path)
        };

        let socks4_db_path = if self.proxy_detector.socks4_db_path.is_absolute() {
            self.proxy_detector.socks4_db_path.clone()
        } else {
            let base_path = std::env::current_dir()?;
            base_path.join(&self.proxy_detector.socks4_db_path)
        };

        let socks5_db_path = if self.proxy_detector.socks5_db_path.is_absolute() {
            self.proxy_detector.socks5_db_path.clone()
        } else {
            let base_path = std::env::current_dir()?;
            base_path.join(&self.proxy_detector.socks5_db_path)
        };

        Ok((http_db_path, socks4_db_path, socks5_db_path))
    }
}