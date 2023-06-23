#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Masterpass {
    /// Owner's verified billing address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub billing_address: Option<stripe_types::address::Address>,
    /// Owner's verified email.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub email: Option<String>,
    /// Owner's verified full name.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub name: Option<String>,
    /// Owner's verified shipping address.
    ///
    /// Values are verified or provided by the wallet directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub shipping_address: Option<stripe_types::address::Address>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Masterpass {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}