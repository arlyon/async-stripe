/// SWIFT Records contain U.S. bank account details per the SWIFT format.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct FundingInstructionsBankTransferSwiftRecord {
    /// The account number
    pub account_number: String,
    /// The bank name
    pub bank_name: String,
    /// The SWIFT code
    pub swift_code: String,
}
