#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct BacsDebitPayments {
    /// The Bacs Direct Debit Display Name for this account.
    ///
    /// For payments made with Bacs Direct Debit, this will appear on the mandate, and as the statement descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for BacsDebitPayments {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
