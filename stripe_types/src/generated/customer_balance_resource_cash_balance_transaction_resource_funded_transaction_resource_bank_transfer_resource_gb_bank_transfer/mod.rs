#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceGbBankTransfer
{
    /// The last 4 digits of the account number of the sender of the funding.
    pub account_number_last4: Option<String>,
    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
    /// The sort code of the bank of the sender of the funding.
    pub sort_code: Option<String>,
}
