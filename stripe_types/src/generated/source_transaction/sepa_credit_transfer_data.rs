#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SepaCreditTransferData {
    /// Reference associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Sender's bank account IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_iban: Option<String>,
    /// Sender's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SepaCreditTransferData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
