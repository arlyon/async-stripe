/// Change to a FinancialAccount's balance
#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryTransactionsResourceBalanceImpact {
    /// The change made to funds the user can spend right now.
    pub cash: i64,
    /// The change made to funds that are not spendable yet, but will become available at a later time.
    pub inbound_pending: i64,
    /// The change made to funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: i64,
}
