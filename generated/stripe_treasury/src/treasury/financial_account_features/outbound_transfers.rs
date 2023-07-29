/// OutboundTransfers contains outbound transfers features for a FinancialAccount.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct OutboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<
        stripe_treasury::treasury::financial_account::ach_toggle_settings::AchToggleSettings,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<stripe_treasury::treasury::financial_account::toggle_settings::ToggleSettings>,
}
