#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionBoleto {
    /// The timestamp after which the boleto expires.
    pub expires_at: Option<stripe_types::Timestamp>,
    /// The URL to the hosted boleto voucher page, which allows customers to view the boleto voucher.
    pub hosted_voucher_url: Option<String>,
    /// The boleto number.
    pub number: Option<String>,
    /// The URL to the downloadable boleto voucher PDF.
    pub pdf: Option<String>,
}
