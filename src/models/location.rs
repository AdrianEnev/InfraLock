use serde::{Deserialize, Serialize};
use maxminddb::geoip2;

impl From<geoip2::City<'_>> for GeoInfo {
    fn from(city: geoip2::City<'_>) -> Self {
        GeoInfo {
            city: city.city.and_then(|c| {
                c.names.and_then(|names| {
                    names.get("en").map(|name| City {
                        names: Some([("en".to_string(), name.to_string())].into_iter().collect()),
                    })
                })
            }),
            country: city.country.and_then(|c| {
                c.names.and_then(|names| {
                    names.get("en").map(|name| Country {
                        names: Some([("en".to_string(), name.to_string())].into_iter().collect()),
                    })
                })
            }),
            location: city.location.map(|loc| Location {
                latitude: loc.latitude,
                longitude: loc.longitude,
            }),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeoInfo {
    pub city: Option<City>,
    pub country: Option<Country>,
    pub location: Option<Location>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct City {
    pub names: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    pub names: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AsnInfo {
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<String>,
}

impl<'a> From<&geoip2::Asn<'a>> for AsnInfo {
    fn from(asn: &geoip2::Asn<'a>) -> Self {
        AsnInfo {
            autonomous_system_number: asn.autonomous_system_number,
            autonomous_system_organization: asn.autonomous_system_organization.as_ref().map(|s| s.to_string()),
        }
    }
}