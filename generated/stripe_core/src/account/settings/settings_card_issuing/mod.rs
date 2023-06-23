#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SettingsCardIssuing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<
        stripe_core::account::settings::settings_card_issuing::tos_acceptance::TosAcceptance,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SettingsCardIssuing {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod tos_acceptance;
pub use tos_acceptance::TosAcceptance;
