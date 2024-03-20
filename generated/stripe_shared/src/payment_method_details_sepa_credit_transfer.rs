#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsSepaCreditTransfer {
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// IBAN of the bank account to transfer funds to.
    pub iban: Option<String>,
}
