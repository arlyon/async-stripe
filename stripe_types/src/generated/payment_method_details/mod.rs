#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types::bacs_debit::BacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types::bancontact::Bancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types::blik::Blik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types::boleto::Boleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types::card_present::CardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types::cashapp::Cashapp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types::ideal::Ideal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types::klarna::Klarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::link::Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types::paypal::Paypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types::sofort::Sofort>,
    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    ///
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types::us_bank_account::UsBankAccount>,
}
