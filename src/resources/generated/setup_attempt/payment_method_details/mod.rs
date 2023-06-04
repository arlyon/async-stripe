#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::setup_attempt::payment_method_details::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit:
        Option<crate::setup_attempt::payment_method_details::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::setup_attempt::payment_method_details::bacs_debit::BacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::setup_attempt::payment_method_details::bancontact::Bancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<crate::setup_attempt::payment_method_details::blik::Blik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<crate::setup_attempt::payment_method_details::boleto::Boleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::setup_attempt::payment_method_details::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present:
        Option<crate::setup_attempt::payment_method_details::card_present::CardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::setup_attempt::payment_method_details::ideal::Ideal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::setup_attempt::payment_method_details::klarna::Klarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<crate::setup_attempt::payment_method_details::link::Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::setup_attempt::payment_method_details::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::setup_attempt::payment_method_details::sofort::Sofort>,
    /// The type of the payment method used in the SetupIntent (e.g., `card`).
    ///
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains confirmation-specific information for the payment method.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<crate::setup_attempt::payment_method_details::us_bank_account::UsBankAccount>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod acss_debit;
pub use acss_debit::AcssDebit;
pub mod au_becs_debit;
pub use au_becs_debit::AuBecsDebit;
pub mod bacs_debit;
pub use bacs_debit::BacsDebit;
pub mod bancontact;
pub use bancontact::Bancontact;
pub mod blik;
pub use blik::Blik;
pub mod boleto;
pub use boleto::Boleto;
pub mod card;
pub use card::Card;
pub mod card_present;
pub use card_present::CardPresent;
pub mod ideal;
pub use ideal::Ideal;
pub mod klarna;
pub use klarna::Klarna;
pub mod link;
pub use link::Link;
pub mod sepa_debit;
pub use sepa_debit::SepaDebit;
pub mod sofort;
pub use sofort::Sofort;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
