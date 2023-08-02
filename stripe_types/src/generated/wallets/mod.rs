#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Wallets {
    pub apple_pay: stripe_types::apple_pay::ApplePay,
    pub google_pay: stripe_types::google_pay::GooglePay,
    /// Unique identifier for a card used with digital wallets.
    pub primary_account_identifier: Option<String>,
}
