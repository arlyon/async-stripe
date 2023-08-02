/// Settings related to Outbound Payments features on a Financial Account.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct OutboundPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<stripe_treasury::ach_toggle_settings::AchToggleSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<stripe_treasury::toggle_settings::ToggleSettings>,
}
