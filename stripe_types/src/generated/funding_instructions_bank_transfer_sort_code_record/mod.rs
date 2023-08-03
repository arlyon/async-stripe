/// Sort Code Records contain U.K.
///
/// bank account details per the sort code format.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FundingInstructionsBankTransferSortCodeRecord {
    /// The name of the person or business that owns the bank account.
    pub account_holder_name: String,
    /// The account number.
    pub account_number: String,
    /// The six-digit sort code.
    pub sort_code: String,
}
