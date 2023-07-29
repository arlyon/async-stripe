#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NextActionKonbiniDisplayDetails {
    /// The timestamp at which the pending Konbini payment expires.
    pub expires_at: stripe_types::Timestamp,
    /// The URL for the Konbini payment instructions page, which allows customers to view and print a Konbini voucher.
    pub hosted_voucher_url: Option<String>,
    pub stores: stripe_types::payment_intent::next_action_konbini_display_details::stores::Stores,
}
pub mod stores;
pub use stores::Stores;
