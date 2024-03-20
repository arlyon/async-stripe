#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsCardNetworkToken {
    /// Indicates if Stripe used a network token, either user provided or Stripe managed when processing the transaction.
    pub used: bool,
}
