/// ABA Records contain U.S.
///
/// bank account details per the ABA format.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Aba {
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    /// The account number.
    pub account_number: Option<String>,
    /// The last four characters of the account number.
    pub account_number_last4: String,
    /// Name of the bank.
    pub bank_name: String,
    /// Routing number for the account.
    pub routing_number: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Aba {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
