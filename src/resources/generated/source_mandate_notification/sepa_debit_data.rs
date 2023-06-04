#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SepaDebitData {
    /// SEPA creditor ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditor_identifier: Option<String>,
    /// Last 4 digits of the account number associated with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// Mandate reference associated with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_reference: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SepaDebitData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
