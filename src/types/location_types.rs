use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeoInfo {
    pub city: Option<City>,
    pub country: Option<Country>,
    pub location: Option<Location>,
}

#[derive(Debug, Deserialize)]
pub struct City {
    pub names: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    pub names: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct Location {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}