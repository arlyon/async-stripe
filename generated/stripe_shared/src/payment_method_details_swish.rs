#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsSwish {
    /// Uniquely identifies the payer's Swish account.
    /// You can use this attribute to check whether two Swish transactions were paid for by the same payer.
    pub fingerprint: Option<String>,
    /// Payer bank reference number for the payment
    pub payment_reference: Option<String>,
    /// The last four digits of the Swish account phone number
    pub verified_phone_last4: Option<String>,
}
