#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MandatePaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types::MandateAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types::MandateAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types::MandateBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types::MandateBlik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::CardMandatePaymentMethodDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types::MandateCashapp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::MandateLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types::MandatePaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types::MandateSepaDebit>,
    /// The type of the payment method associated with this mandate.
    ///
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains mandate information specific to the payment method.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types::MandateUsBankAccount>,
}
