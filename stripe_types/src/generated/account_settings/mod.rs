#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<stripe_types::AccountBacsDebitPaymentsSettings>,
    pub branding: stripe_types::AccountBrandingSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<stripe_types::AccountCardIssuingSettings>,
    pub card_payments: stripe_types::AccountCardPaymentsSettings,
    pub dashboard: stripe_types::AccountDashboardSettings,
    pub payments: stripe_types::AccountPaymentsSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_types::AccountPayoutSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<stripe_types::AccountSepaDebitPaymentsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_types::AccountTreasurySettings>,
}
