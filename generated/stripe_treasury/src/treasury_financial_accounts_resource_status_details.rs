#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceStatusDetails {
    /// Details related to the closure of this FinancialAccount
    pub closed: Option<stripe_treasury::TreasuryFinancialAccountsResourceClosedStatusDetails>,
}
