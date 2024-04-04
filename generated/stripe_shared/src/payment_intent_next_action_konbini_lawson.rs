#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionKonbiniLawson {
    /// The confirmation number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,
    /// The payment code.
    pub payment_code: String,
}
