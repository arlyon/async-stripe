#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Wallets {
    pub apple_pay: stripe_types::issuing::card::wallets::apple_pay::ApplePay,
    pub google_pay: stripe_types::issuing::card::wallets::google_pay::GooglePay,
    /// Unique identifier for a card used with digital wallets.
    pub primary_account_identifier: Option<String>,
}
pub mod apple_pay;
pub use apple_pay::ApplePay;
pub mod google_pay;
pub use google_pay::GooglePay;
