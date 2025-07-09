use maxminddb::Reader;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GeoInfo {
    city: Option<City>,
    country: Option<Country>,
    location: Option<Location>,
}

#[derive(Debug, Deserialize)]
struct City {
    names: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct Country {
    names: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: Option<f64>,
    longitude: Option<f64>,
}


fn main() {
    // Path to the mounted .mmdb file (adjust if needed)
    let reader = Reader::open_readfile("data/GeoLite2-City.mmdb")
        .expect("Failed to open GeoLite2-City.mmdb");

    let ip = "8.8.8.8";

    // Parse IP address and lookup
    let ip_addr = ip.parse::<std::net::IpAddr>()
        .expect("Invalid IP address format");
    let geo: GeoInfo = reader.lookup(ip_addr).expect("Failed to lookup IP")
        .expect("Failed to parse GeoInfo");

    // Get city and country
    let city = geo.city
        .as_ref()
        .and_then(|c| c.names.as_ref())
        .and_then(|names| names.get("en"))
        .map_or("Unknown City".to_string(), |s| s.clone());
    
    let country = geo.country
        .as_ref()
        .and_then(|c| c.names.as_ref())
        .and_then(|names| names.get("en"))
        .map_or("Unknown Country".to_string(), |s| s.clone());

    let location = geo.location
        .as_ref()
        .map(|l| format!("{}, {}", l.latitude.unwrap_or(0.0), l.longitude.unwrap_or(0.0)))
        .unwrap_or_else(|| "Unknown Location".to_string());
    
    println!("City: {}", city);
    println!("Country: {}", country);
    println!("Location: {}", location);
}
