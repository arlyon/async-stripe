#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetupAttemptPaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_shared::SetupAttemptPaymentMethodDetailsBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_shared::SetupAttemptPaymentMethodDetailsCashapp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_shared::SetupAttemptPaymentMethodDetailsKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_shared::SetupAttemptPaymentMethodDetailsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_shared::SetupAttemptPaymentMethodDetailsPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_shared::SetupAttemptPaymentMethodDetailsSofort>,
    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_shared::SetupAttemptPaymentMethodDetailsUsBankAccount>,
}
