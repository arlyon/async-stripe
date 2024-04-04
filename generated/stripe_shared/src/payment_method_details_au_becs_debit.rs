#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsAuBecsDebit {
    /// Bank-State-Branch number of the bank account.
    pub bsb_number: Option<String>,
    /// Uniquely identifies this particular bank account.
    /// You can use this attribute to check whether two bank accounts are the same.
    pub fingerprint: Option<String>,
    /// Last four digits of the bank account number.
    pub last4: Option<String>,
    /// ID of the mandate used to make this payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<String>,
}
