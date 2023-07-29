#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct BillingDetails {
    /// Billing address for the order.
    pub address: Option<stripe_types::address::Address>,
    /// Email address for the order.
    pub email: Option<String>,
    /// Full name for the order.
    pub name: Option<String>,
    /// Billing phone number for the order (including extension).
    pub phone: Option<String>,
}
