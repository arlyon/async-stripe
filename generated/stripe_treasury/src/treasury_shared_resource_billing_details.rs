#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasurySharedResourceBillingDetails {
    pub address: stripe_shared::Address,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
}
