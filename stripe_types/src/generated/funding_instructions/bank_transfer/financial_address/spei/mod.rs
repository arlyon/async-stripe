/// SPEI Records contain Mexico bank account details per the SPEI format.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Spei {
    /// The three-digit bank code.
    pub bank_code: String,
    /// The short banking institution name.
    pub bank_name: String,
    /// The CLABE number.
    pub clabe: String,
}
