#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: Option<String>,
}
