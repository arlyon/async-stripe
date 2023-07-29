/// Settings related to Financial Addresses features on a Financial Account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct FinancialAddresses {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<stripe_treasury::treasury::financial_account::toggle_settings::ToggleSettings>,
}
