#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BillingDetails {
    pub address: stripe_types::address::Address,
    /// Email address.
    pub email: Option<String>,
    /// Full name.
    pub name: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BillingDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
