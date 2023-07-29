#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments:
        Option<stripe_types::account::settings::bacs_debit_payments::BacsDebitPayments>,
    pub branding: stripe_types::account::settings_branding::SettingsBranding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing:
        Option<stripe_types::account::settings::settings_card_issuing::SettingsCardIssuing>,
    pub card_payments: stripe_types::account::settings_card_payments::SettingsCardPayments,
    pub dashboard: stripe_types::account::settings_dashboard::SettingsDashboard,
    pub payments: stripe_types::account::settings_payments::SettingsPayments,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_types::account::settings_payouts::SettingsPayouts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments:
        Option<stripe_types::account::settings::sepa_debit_payments::SepaDebitPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types::account::settings::settings_treasury::SettingsTreasury>,
}
pub mod bacs_debit_payments;
pub use bacs_debit_payments::BacsDebitPayments;
pub mod settings_card_issuing;
pub use settings_card_issuing::SettingsCardIssuing;
pub mod sepa_debit_payments;
pub use sepa_debit_payments::SepaDebitPayments;
pub mod settings_treasury;
pub use settings_treasury::SettingsTreasury;
