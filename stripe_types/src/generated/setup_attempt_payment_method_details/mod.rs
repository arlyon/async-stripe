#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct SetupAttemptPaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types::SetupAttemptPaymentMethodDetailsAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types::SetupAttemptPaymentMethodDetailsAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types::SetupAttemptPaymentMethodDetailsBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types::SetupAttemptPaymentMethodDetailsBancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types::SetupAttemptPaymentMethodDetailsBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::SetupAttemptPaymentMethodDetailsCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types::SetupAttemptPaymentMethodDetailsCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types::SetupAttemptPaymentMethodDetailsCashapp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types::SetupAttemptPaymentMethodDetailsIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types::SetupAttemptPaymentMethodDetailsKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::SetupAttemptPaymentMethodDetailsLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types::SetupAttemptPaymentMethodDetailsPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types::SetupAttemptPaymentMethodDetailsSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types::SetupAttemptPaymentMethodDetailsSofort>,
    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    ///
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types::SetupAttemptPaymentMethodDetailsUsBankAccount>,
}
