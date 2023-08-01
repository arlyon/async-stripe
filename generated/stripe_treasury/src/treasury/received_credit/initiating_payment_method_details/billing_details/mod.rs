#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BillingDetails {
    pub address: stripe_types::address::Address,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
}
