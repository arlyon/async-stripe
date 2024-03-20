#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardholderAddress {
    pub address: stripe_shared::Address,
}
