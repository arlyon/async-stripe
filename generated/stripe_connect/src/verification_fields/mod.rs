#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct VerificationFields {
    pub company: stripe_connect::details::Details,
    pub individual: stripe_connect::details::Details,
}
