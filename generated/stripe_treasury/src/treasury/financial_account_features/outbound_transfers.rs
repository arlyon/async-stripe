/// OutboundTransfers contains outbound transfers features for a FinancialAccount.
#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct OutboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<
        stripe_treasury::treasury::financial_account::ach_toggle_settings::AchToggleSettings,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<stripe_treasury::treasury::financial_account::toggle_settings::ToggleSettings>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for OutboundTransfers {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
