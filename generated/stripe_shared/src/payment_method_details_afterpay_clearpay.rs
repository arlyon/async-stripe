#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsAfterpayClearpay {
    /// The Afterpay order ID associated with this payment intent.
    pub order_id: Option<String>,
    /// Order identifier shown to the merchant in Afterpay’s online portal.
    pub reference: Option<String>,
}
