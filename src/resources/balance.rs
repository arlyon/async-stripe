use serde_json as json;

/// The resource representing a Stripe account balance.
///
/// For more details see https://stripe.com/docs/api#balance.
#[derive(Debug, Deserialize)]
pub struct Balance {
    pub object: String,
    pub available: Vec<json::Value>,
    pub connect_reserved: Vec<json::Value>,
    pub livemode: bool,
    pub pending: Vec<json::Value>,
}
