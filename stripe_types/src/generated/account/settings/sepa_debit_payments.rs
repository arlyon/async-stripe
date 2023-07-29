#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SepaDebitPayments {
    /// SEPA creditor identifier that identifies the company making the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditor_id: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SepaDebitPayments {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
