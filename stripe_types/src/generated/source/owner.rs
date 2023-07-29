#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Owner {
    /// Owner's address.
    pub address: Option<stripe_types::address::Address>,
    /// Owner's email address.
    pub email: Option<String>,
    /// Owner's full name.
    pub name: Option<String>,
    /// Owner's phone number (including extension).
    pub phone: Option<String>,
    /// Verified owner's address.
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_address: Option<stripe_types::address::Address>,
    /// Verified owner's email address.
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_email: Option<String>,
    /// Verified owner's full name.
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_name: Option<String>,
    /// Verified owner's phone number (including extension).
    ///
    /// Verified values are verified or provided by the payment method directly (and if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated.
    pub verified_phone: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Owner {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
