/// ABA Records contain U.S. bank account details per the ABA format.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FundingInstructionsBankTransferAbaRecord {
    /// The ABA account number
    pub account_number: String,
    /// The bank name
    pub bank_name: String,
    /// The ABA routing number
    pub routing_number: String,
}
