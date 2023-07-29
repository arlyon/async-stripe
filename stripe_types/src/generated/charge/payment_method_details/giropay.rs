#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Giropay {
    /// Bank code of bank associated with the bank account.
    pub bank_code: Option<String>,
    /// Name of the bank associated with the bank account.
    pub bank_name: Option<String>,
    /// Bank Identifier Code of the bank associated with the bank account.
    pub bic: Option<String>,
    /// Owner's verified full name.
    ///
    /// Values are verified or provided by Giropay directly (if supported) at the time of authorization or settlement.
    /// They cannot be set or mutated. Giropay rarely provides this information so the attribute is usually empty.
    pub verified_name: Option<String>,
}
