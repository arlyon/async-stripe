#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BillingDetails {
    /// Billing address.
    pub address: Option<stripe_shared::Address>,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
    /// Billing phone number (including extension).
    pub phone: Option<String>,
}
