#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit:
        Option<stripe_types::setup_intent::payment_method_options::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types::setup_intent::payment_method_options::blik::Blik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::setup_intent::payment_method_options::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::setup_intent::payment_method_options::link::Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types::setup_intent::payment_method_options::paypal::Paypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit:
        Option<stripe_types::setup_intent::payment_method_options::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<stripe_types::setup_intent::payment_method_options::us_bank_account::UsBankAccount>,
}
pub mod acss_debit;
pub use acss_debit::AcssDebit;
pub mod blik;
pub use blik::Blik;
pub mod card;
pub use card::Card;
pub mod link;
pub use link::Link;
pub mod blik_mandate_options;
pub use blik_mandate_options::BlikMandateOptions;
pub mod sepa_debit_mandate_options;
pub use sepa_debit_mandate_options::SepaDebitMandateOptions;
pub mod paypal;
pub use paypal::Paypal;
pub mod sepa_debit;
pub use sepa_debit::SepaDebit;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
