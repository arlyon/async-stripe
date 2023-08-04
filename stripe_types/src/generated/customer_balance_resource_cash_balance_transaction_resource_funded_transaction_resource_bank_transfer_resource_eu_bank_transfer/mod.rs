#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceFundedTransactionResourceBankTransferResourceEuBankTransfer
{
    /// The BIC of the bank of the sender of the funding.
    pub bic: Option<String>,
    /// The last 4 digits of the IBAN of the sender of the funding.
    pub iban_last4: Option<String>,
    /// The full name of the sender, as supplied by the sending bank.
    pub sender_name: Option<String>,
}
