/// Settings related to Financial Addresses features on a Financial Account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<stripe_treasury::TreasuryFinancialAccountsResourceAbaToggleSettings>,
}
