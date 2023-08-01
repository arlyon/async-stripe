/// An error response from the Stripe API.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct StripeErrorResponse {
    pub error: Box<stripe_types::api_errors::ApiErrors>,
}
