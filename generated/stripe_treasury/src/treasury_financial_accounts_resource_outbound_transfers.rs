/// OutboundTransfers contains outbound transfers features for a FinancialAccount.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<stripe_treasury::TreasuryFinancialAccountsResourceAchToggleSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<stripe_treasury::TreasuryFinancialAccountsResourceToggleSettings>,
}
