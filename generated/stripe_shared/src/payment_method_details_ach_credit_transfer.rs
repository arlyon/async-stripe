#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetailsAchCreditTransfer {
    /// Account number to transfer funds to.
    pub account_number: Option<String>,
    /// Name of the bank associated with the routing number.
    pub bank_name: Option<String>,
    /// Routing transit number for the bank account to transfer funds to.
    pub routing_number: Option<String>,
    /// SWIFT code of the bank associated with the routing number.
    pub swift_code: Option<String>,
}
