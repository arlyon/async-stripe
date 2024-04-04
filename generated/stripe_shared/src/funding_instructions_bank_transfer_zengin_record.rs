/// Zengin Records contain Japan bank account details per the Zengin format.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FundingInstructionsBankTransferZenginRecord {
    /// The account holder name
    pub account_holder_name: Option<String>,
    /// The account number
    pub account_number: Option<String>,
    /// The bank account type. In Japan, this can only be `futsu` or `toza`.
    pub account_type: Option<String>,
    /// The bank code of the account
    pub bank_code: Option<String>,
    /// The bank name of the account
    pub bank_name: Option<String>,
    /// The branch code of the account
    pub branch_code: Option<String>,
    /// The branch name of the account
    pub branch_name: Option<String>,
}
