#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceTransactionAchCreditTransferData {
    /// Customer data associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<String>,
    /// Bank account fingerprint associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Last 4 digits of the account number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// Routing number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
