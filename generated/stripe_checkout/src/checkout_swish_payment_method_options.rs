#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CheckoutSwishPaymentMethodOptions {
    /// The order reference that will be displayed to customers in the Swish application.
    /// Defaults to the `id` of the Payment Intent.
    pub reference: Option<String>,
}
