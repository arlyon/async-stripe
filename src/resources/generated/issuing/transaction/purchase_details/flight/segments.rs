#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Segments {
    /// The three-letter IATA airport code of the flight's destination.
    pub arrival_airport_code: Option<String>,
    /// The airline carrier code.
    pub carrier: Option<String>,
    /// The three-letter IATA airport code that the flight departed from.
    pub departure_airport_code: Option<String>,
    /// The flight number.
    pub flight_number: Option<String>,
    /// The flight's service class.
    pub service_class: Option<String>,
    /// Whether a stopover is allowed on this flight.
    pub stopover_allowed: Option<bool>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Segments {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
