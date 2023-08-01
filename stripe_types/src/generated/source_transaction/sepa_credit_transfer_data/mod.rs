#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
