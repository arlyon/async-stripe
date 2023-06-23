#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ChfCreditTransferData {
    /// Reference associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Sender's country address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_address_country: Option<String>,
    /// Sender's line 1 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_address_line1: Option<String>,
    /// Sender's bank account IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_iban: Option<String>,
    /// Sender's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ChfCreditTransferData {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
