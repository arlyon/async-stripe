/// Balance information for the FinancialAccount.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Balance {
    /// Funds the user can spend right now.
    pub cash: std::collections::HashMap<String, i64>,
    /// Funds not spendable yet, but will become available at a later time.
    pub inbound_pending: std::collections::HashMap<String, i64>,
    /// Funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: std::collections::HashMap<String, i64>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Balance {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
