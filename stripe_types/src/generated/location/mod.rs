#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Location {
    /// The city where the payment originated.
    pub city: Option<String>,
    /// Two-letter ISO code representing the country where the payment originated.
    pub country: Option<String>,
    /// The geographic latitude where the payment originated.
    pub latitude: Option<f64>,
    /// The geographic longitude where the payment originated.
    pub longitude: Option<f64>,
    /// The state/county/province/region where the payment originated.
    pub region: Option<String>,
}
