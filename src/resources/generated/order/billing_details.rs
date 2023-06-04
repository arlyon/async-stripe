#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BillingDetails {
    /// Billing address for the order.
    pub address: Option<crate::address::Address>,
    /// Email address for the order.
    pub email: Option<String>,
    /// Full name for the order.
    pub name: Option<String>,
    /// Billing phone number for the order (including extension).
    pub phone: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BillingDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
