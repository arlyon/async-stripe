/// InboundTransfers contains inbound transfers features for a FinancialAccount.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<
        stripe_treasury::treasury::financial_account::ach_toggle_settings::AchToggleSettings,
    >,
}
