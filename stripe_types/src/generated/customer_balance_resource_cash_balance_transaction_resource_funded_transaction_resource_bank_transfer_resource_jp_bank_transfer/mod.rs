#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceJpBankTransfer {
    /// The name of the bank of the sender of the funding.
    pub sender_bank: Option<String>,
    /// The name of the bank branch of the sender of the funding.
    pub sender_branch: Option<String>,
    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
}
