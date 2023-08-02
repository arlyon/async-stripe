#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct GbpCreditTransferData {
    /// Bank account fingerprint associated with the Stripe owned bank account receiving the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// The credit transfer rails the sender used to push this transfer.
    ///
    /// The possible rails are: Faster Payments, BACS, CHAPS, and wire transfers.
    /// Currently only Faster Payments is supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_method: Option<String>,
    /// Last 4 digits of sender account number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    /// Sender entered arbitrary information about the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Sender account number associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_account_number: Option<String>,
    /// Sender name associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
    /// Sender sort code associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_sort_code: Option<String>,
}
