/// An error response from the Stripe API
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Error {
    pub error: Box<stripe_shared::ApiErrors>,
}
