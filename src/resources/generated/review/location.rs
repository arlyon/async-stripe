#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Location {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
