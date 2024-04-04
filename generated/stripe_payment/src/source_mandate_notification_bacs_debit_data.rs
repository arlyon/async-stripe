#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SourceMandateNotificationBacsDebitData {
    /// Last 4 digits of the account number associated with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}
