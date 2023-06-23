#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments:
        Option<stripe_core::account::settings::bacs_debit_payments::BacsDebitPayments>,
    pub branding: stripe_core::account::settings_branding::SettingsBranding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing:
        Option<stripe_core::account::settings::settings_card_issuing::SettingsCardIssuing>,
    pub card_payments: stripe_core::account::settings_card_payments::SettingsCardPayments,
    pub dashboard: stripe_core::account::settings_dashboard::SettingsDashboard,
    pub payments: stripe_core::account::settings_payments::SettingsPayments,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_core::account::settings_payouts::SettingsPayouts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments:
        Option<stripe_core::account::settings::sepa_debit_payments::SepaDebitPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_core::account::settings::settings_treasury::SettingsTreasury>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Settings {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod bacs_debit_payments;
pub use bacs_debit_payments::BacsDebitPayments;
pub mod settings_card_issuing;
pub use settings_card_issuing::SettingsCardIssuing;
pub mod sepa_debit_payments;
pub use sepa_debit_payments::SepaDebitPayments;
pub mod settings_treasury;
pub use settings_treasury::SettingsTreasury;
