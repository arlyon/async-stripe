/// ABA Records contain U.S. bank account details per the ABA format.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceAbaRecord {
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    /// The account number.
    pub account_number: Option<String>,
    /// The last four characters of the account number.
    pub account_number_last4: String,
    /// Name of the bank.
    pub bank_name: String,
    /// Routing number for the account.
    pub routing_number: String,
}
