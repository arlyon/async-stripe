#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct AccountBacsDebitPaymentsSettings {
    /// The Bacs Direct Debit Display Name for this account.
    ///
    /// For payments made with Bacs Direct Debit, this will appear on the mandate, and as the statement descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
