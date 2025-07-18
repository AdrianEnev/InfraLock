// lookup_service.rs
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::location::{GeoInfo, AsnInfo};
use crate::models::threat_score::ThreatScore;
use crate::handlers::LookupResponse;
use crate::errors::AppError;
use crate::services::response_action::ResponseActionService;
use crate::ip_lookup::{IpLookupService, IpCategory};
use maxminddb;
use moka::sync::Cache;

pub struct LookupService {
    maxmind_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
    asn_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
    lookup_cache: Arc<Cache<IpAddr, LookupResponse>>,
    ip_lookup_service: Arc<IpLookupService>,
}

impl LookupService {
    pub fn new(
        maxmind_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
        asn_reader: Arc<RwLock<maxminddb::Reader<Vec<u8>>>>,
        lookup_cache: Arc<Cache<IpAddr, LookupResponse>>,
        ip_lookup_service: Arc<IpLookupService>,
    ) -> Self {
        Self {
            maxmind_reader,
            asn_reader,
            lookup_cache,
            ip_lookup_service,
        }
    }

    pub async fn lookup_ip(&self, ip_addr: IpAddr) -> Result<LookupResponse, AppError> {
        // Check cache first
        if let Some(cached) = self.lookup_cache.get(&ip_addr) {
            return Ok(cached.clone());
        }

        // Get IP category using the new ip_lookup_service
        let ip_category = self.ip_lookup_service.tree().lookup(ip_addr);
        
        // Get geo and ASN information
        let reader = self.maxmind_reader.read().await;
        let asn_reader = self.asn_reader.read().await;
        
        let (geo_result, asn_result) = tokio::join!(
            async { reader.lookup(ip_addr) },
            async { asn_reader.lookup(ip_addr) },
        );

        // Convert the raw results to our models
        let city: Option<maxminddb::geoip2::City<'_>> = geo_result?;
        let asn: Option<maxminddb::geoip2::Asn<'_>> = asn_result?;
        
        let geo_info = city.map(GeoInfo::from);
        let asn_info = asn.as_ref().map(AsnInfo::from);

        // Determine threat type based on IP category
        let (is_vpn, is_proxy, is_tor, proxy_type) = match ip_category {
            Some(IpCategory::Vpn) => (true, false, false, None),
            Some(IpCategory::ProxyHttp) => (false, true, false, Some("http")),
            Some(IpCategory::ProxySocks4) => (false, true, false, Some("socks4")),
            Some(IpCategory::ProxySocks5) => (false, true, false, Some("socks5")),
            Some(IpCategory::TorExitNode) => (false, false, true, None),
            None => (false, false, false, None),
        };

        // Calculate threat score
        let threat_score = ThreatScore::from_ip_info(
            ip_addr,
            is_vpn,
            is_proxy,
            proxy_type,
            is_tor,
        );

        // Determine recommended response action
        let response_action_service = ResponseActionService::new();
        let recommended_action = response_action_service.determine_action(&threat_score);

        // Build the response
        let response = LookupResponse {
            ip: ip_addr.to_string(),
            geo_info,
            asn_info,
            is_vpn_or_datacenter: is_vpn,
            is_proxy,
            proxy_type,
            is_tor_exit_node: is_tor,
            threat_score: threat_score.score,
            threat_details: threat_score.findings
                .iter()
                .map(|f| f.description.clone())
                .collect(),
                recommended_action: format!("{:?}", recommended_action).to_lowercase()
        };

        // Cache the response
        self.lookup_cache.insert(ip_addr, response.clone());

        Ok(response)
    }
}