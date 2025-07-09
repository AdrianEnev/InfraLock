mod utils;
use maxminddb::Reader;
use serde::Deserialize;
use utils::read_line;

#[derive(Debug, Deserialize)]
struct GeoInfo {
    city: Option<Names>,
    country: Option<Names>,
    location: Option<Location>,
}

#[derive(Debug, Deserialize)]
struct Names {
    names: std::collections::HashMap<String, String>,
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

    // Add ip input manually
    println!("Enter example ip");
    let ip = read_line();

    // Parse IP address and lookup
    let ip_addr = ip.parse::<std::net::IpAddr>()
        .expect("Invalid IP address format");
    let geo = reader.lookup::<GeoInfo>(ip_addr)
        .expect("Failed to lookup IP");
    
    println!("{:?}", geo);
}
