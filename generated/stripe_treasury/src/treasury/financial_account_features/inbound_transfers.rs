/// InboundTransfers contains inbound transfers features for a FinancialAccount.
#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<
        stripe_treasury::treasury::financial_account::ach_toggle_settings::AchToggleSettings,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InboundTransfers {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
