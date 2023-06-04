/// An error response from the Stripe API.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct StripeErrorResponse {
    pub error: Box<crate::api_errors::ApiErrors>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for StripeErrorResponse {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
