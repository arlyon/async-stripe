#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AccountSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<stripe_shared::AccountBacsDebitPaymentsSettings>,
    pub branding: stripe_shared::AccountBrandingSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<stripe_shared::AccountCardIssuingSettings>,
    pub card_payments: stripe_shared::AccountCardPaymentsSettings,
    pub dashboard: stripe_shared::AccountDashboardSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<stripe_shared::AccountInvoicesSettings>,
    pub payments: stripe_shared::AccountPaymentsSettings,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<stripe_shared::AccountPayoutSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<stripe_shared::AccountSepaDebitPaymentsSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<stripe_shared::AccountTreasurySettings>,
}
