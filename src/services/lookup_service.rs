// lookup_service.rs
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::location::{GeoInfo, AsnInfo};
use crate::models::threat_score::ThreatScore;
use crate::services::vpn_detection::VpnDetector;
use crate::services::proxy_detection::ProxyDetector;
use crate::services::tor_detection::TorDetector;
use crate::handlers::LookupResponse;
use crate::errors::AppError;
use std::collections::HashMap;

pub struct LookupService {
    maxmind_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
    asn_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
    lookup_cache: Arc<RwLock<HashMap<IpAddr, LookupResponse>>>,
}

impl LookupService {
    pub fn new(
        maxmind_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
        asn_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
        lookup_cache: Arc<RwLock<HashMap<IpAddr, LookupResponse>>>,
    ) -> Self {
        Self {
            maxmind_reader,
            asn_reader,
            lookup_cache,
        }
    }

    pub async fn lookup_ip(&self, ip_addr: IpAddr) -> Result<LookupResponse, AppError> {
        if let Some(cached) = self.lookup_cache.read().await.get(&ip_addr) {
            return Ok(cached.clone());
        }

        let reader = self.maxmind_reader.read().await;
        let asn_reader = self.asn_reader.read().await;
        let vpn_detector = VpnDetector::get();
        let proxy_detector = ProxyDetector::get();
        let tor_detector = TorDetector::get();

        let (geo_result, asn_result, vpn_result, proxy_result, tor_result) = tokio::join!(
            async { reader.lookup(ip_addr) },
            async { asn_reader.lookup(ip_addr) },
            async { vpn_detector.is_vpn_or_datacenter(ip_addr) },
            async { proxy_detector.check_proxy(ip_addr) },
            async { tor_detector.is_tor_exit_node(ip_addr) },
        );

        let city: Option<maxminddb::geoip2::City<'_>> = geo_result?;
        let asn: Option<maxminddb::geoip2::Asn<'_>> = asn_result?;
        let geo_info = GeoInfo::from(city.ok_or_else(|| 
            std::io::Error::new(
                std::io::ErrorKind::NotFound, 
                "No geo data found for this IP address"
            )
        )?);
        
        let asn_info = asn.as_ref().map(AsnInfo::from);
        let is_vpn = vpn_result;
        let proxy_type = proxy_result;
        let is_proxy = proxy_type.is_some();
        let is_tor = tor_result;

        let threat_score = ThreatScore::from_ip_info(
            ip_addr,
            is_vpn,
            is_proxy,
            proxy_type,
            is_tor,
        );

        let threat_details = threat_score.findings
            .iter()
            .map(|f| f.description.clone())
            .collect();

        let response = LookupResponse {
            ip: ip_addr.to_string(),
            geo_info,
            asn_info,
            is_vpn_or_datacenter: is_vpn,
            is_proxy,
            proxy_type,
            is_tor_exit_node: is_tor,
            threat_score: threat_score.score,
            threat_details,
        };

        self.lookup_cache.write().await.insert(ip_addr, response.clone());
        Ok(response)
    }
}