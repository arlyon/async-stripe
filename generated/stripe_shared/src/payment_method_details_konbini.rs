#[derive(Copy, Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsKonbini {
    /// If the payment succeeded, this contains the details of the convenience store where the payment was completed.
    pub store: Option<stripe_shared::PaymentMethodDetailsKonbiniStore>,
}
