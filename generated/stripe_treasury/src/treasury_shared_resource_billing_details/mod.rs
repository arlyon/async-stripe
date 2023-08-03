#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasurySharedResourceBillingDetails {
    pub address: stripe_types::Address,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
}
