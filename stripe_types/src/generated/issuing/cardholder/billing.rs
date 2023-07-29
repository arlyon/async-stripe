#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Billing {
    pub address: stripe_types::address::Address,
}
