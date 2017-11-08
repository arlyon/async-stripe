use params::Timestamp;

/// The resource representing a Stripe review of a payment.
///
/// For more details see https://stripe.com/docs/api/dotnet#review_object.
#[derive(Debug, Deserialize)]
pub struct Review {
    pub id: String,
    pub object: String,
    pub charge: String,
    pub created: Timestamp,
    pub livemode: bool,
    pub open: bool,
    pub reason: String,
}
