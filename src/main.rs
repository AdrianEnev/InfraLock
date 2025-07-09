use crate::utils::retreive_ip;

mod utils;
mod types;

fn main() {
    let ip = "8.8.8.8";

    let ip_addr = ip.parse::<std::net::IpAddr>()
        .expect("Invalid IP address format");
    
    let geo_info = retreive_ip::retreive_ip(ip_addr);

    println!("City: {:?}", geo_info.city);
    println!("Country: {:?}", geo_info.country);
    println!("Location: {:?}", geo_info.location);
}
