#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentLinksResourceCustomText {
    /// Custom text that should be displayed alongside shipping address collection.
    pub shipping_address: Option<stripe_types::PaymentLinksResourceCustomTextPosition>,
    /// Custom text that should be displayed alongside the payment confirmation button.
    pub submit: Option<stripe_types::PaymentLinksResourceCustomTextPosition>,
}
