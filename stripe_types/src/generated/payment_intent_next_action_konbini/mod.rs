#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentIntentNextActionKonbini {
    /// The timestamp at which the pending Konbini payment expires.
    pub expires_at: stripe_types::Timestamp,
    /// The URL for the Konbini payment instructions page, which allows customers to view and print a Konbini voucher.
    pub hosted_voucher_url: Option<String>,
    pub stores: stripe_types::PaymentIntentNextActionKonbiniStores,
}
