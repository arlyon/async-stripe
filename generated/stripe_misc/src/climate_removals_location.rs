#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClimateRemovalsLocation {
    /// The city where the supplier is located.
    pub city: Option<String>,
    /// Two-letter ISO code representing the country where the supplier is located.
    pub country: String,
    /// The geographic latitude where the supplier is located.
    pub latitude: Option<f64>,
    /// The geographic longitude where the supplier is located.
    pub longitude: Option<f64>,
    /// The state/county/province/region where the supplier is located.
    pub region: Option<String>,
}
