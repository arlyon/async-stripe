#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<stripe_types::bacs_debit_payments::BacsDebitPayments>,
    pub branding: stripe_types::settings_branding::SettingsBranding,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<stripe_types::settings_card_issuing::SettingsCardIssuing>,
    pub card_payments: stripe_types::settings_card_payments::SettingsCardPayments,
    pub dashboard: stripe_types::settings_dashboard::SettingsDashboard,
    pub payments: stripe_types::settings_payments::SettingsPayments,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_types::settings_payouts::SettingsPayouts>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<stripe_types::sepa_debit_payments::SepaDebitPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types::settings_treasury::SettingsTreasury>,
}
