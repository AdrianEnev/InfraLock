use maxminddb::Reader;
use crate::types::location_types::{GeoInfo, City, Country, Location};

pub fn retreive_ip (
    ip_addr: std::net::IpAddr
) -> GeoInfo {

    let reader = Reader::open_readfile("data/GeoLite2-City.mmdb")
        .expect("Failed to open GeoLite2-City.mmdb");

    let geo: GeoInfo = reader.lookup(ip_addr).expect("Failed to lookup IP")
        .expect("Failed to parse GeoInfo");

    // Get city and country
    let city = geo.city
        .as_ref()
        .and_then(|c| c.names.as_ref())
        .and_then(|names| names.get("en"))
        .map_or_else(
            || None,
            |s| Some(City { names: Some(std::iter::once(("en".to_string(), s.clone())).collect()) })
        );
    
    let country = geo.country
        .as_ref()
        .and_then(|c| c.names.as_ref())
        .and_then(|names| names.get("en"))
        .map_or_else(
            || None,
            |s| Some(Country { names: Some(std::iter::once(("en".to_string(), s.clone())).collect()) })
        );

    let location = geo.location.as_ref().map(|l| Location {
        latitude: l.latitude,
        longitude: l.longitude,
    });

    GeoInfo {
        city,
        country,
        location,
    }    
}
