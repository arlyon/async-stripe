#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BacsDebitData {
    /// Last 4 digits of the account number associated with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BacsDebitData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
