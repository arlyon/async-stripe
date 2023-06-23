#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SettingsTreasury {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance:
        Option<stripe_core::account::settings::settings_treasury::tos_acceptance::TosAcceptance>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SettingsTreasury {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod tos_acceptance;
pub use tos_acceptance::TosAcceptance;
