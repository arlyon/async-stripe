/// Settings related to Financial Addresses features on a Financial Account.
#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct FinancialAddresses {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<stripe_treasury::treasury::financial_account::toggle_settings::ToggleSettings>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for FinancialAddresses {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
