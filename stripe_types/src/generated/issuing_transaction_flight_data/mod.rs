#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct IssuingTransactionFlightData {
    /// The time that the flight departed.
    pub departure_at: Option<i64>,
    /// The name of the passenger.
    pub passenger_name: Option<String>,
    /// Whether the ticket is refundable.
    pub refundable: Option<bool>,
    /// The legs of the trip.
    pub segments: Option<Vec<stripe_types::IssuingTransactionFlightDataLeg>>,
    /// The travel agency that issued the ticket.
    pub travel_agency: Option<String>,
}
