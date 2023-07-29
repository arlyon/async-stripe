#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
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
