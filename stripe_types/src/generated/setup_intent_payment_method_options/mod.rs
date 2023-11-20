#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SetupIntentPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types::SetupIntentPaymentMethodOptionsAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::SetupIntentPaymentMethodOptionsCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::SetupIntentPaymentMethodOptionsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types::SetupIntentPaymentMethodOptionsPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types::SetupIntentPaymentMethodOptionsSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types::SetupIntentPaymentMethodOptionsUsBankAccount>,
}
