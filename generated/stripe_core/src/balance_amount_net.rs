#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BalanceAmountNet {
    /// Balance amount.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_types: Option<stripe_core::BalanceAmountBySourceType>,
}
