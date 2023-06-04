#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EuBankTransfer {
    /// The BIC of the bank of the sender of the funding.
    pub bic: Option<String>,
    /// The last 4 digits of the IBAN of the sender of the funding.
    pub iban_last4: Option<String>,
    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EuBankTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
