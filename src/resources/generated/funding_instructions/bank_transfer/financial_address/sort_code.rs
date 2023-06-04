/// Sort Code Records contain U.K.
///
/// bank account details per the sort code format.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SortCode {
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    /// The account number.
    pub account_number: String,
    /// The six-digit sort code.
    pub sort_code: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SortCode {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
