#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct BalanceDetail {
    /// Funds that are available for use.
    pub available: Vec<stripe_core::BalanceAmount>,
}
