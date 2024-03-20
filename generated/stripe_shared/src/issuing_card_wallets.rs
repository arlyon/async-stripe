#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IssuingCardWallets {
    pub apple_pay: stripe_shared::IssuingCardApplePay,
    pub google_pay: stripe_shared::IssuingCardGooglePay,
    /// Unique identifier for a card used with digital wallets
    pub primary_account_identifier: Option<String>,
}
