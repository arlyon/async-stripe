#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_core::mandate::payment_method_details::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_core::mandate::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_core::mandate::bacs_debit::BacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_core::mandate::payment_method_details::blik::Blik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_core::mandate::payment_method_details::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_core::mandate::payment_method_details::link::Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_core::mandate::payment_method_details::sepa_debit::SepaDebit>,
    /// The type of the payment method associated with this mandate.
    ///
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains mandate information specific to the payment method.
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<stripe_core::mandate::payment_method_details::us_bank_account::UsBankAccount>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod card;
pub use card::Card;
pub mod acss_debit;
pub use acss_debit::AcssDebit;
pub mod blik;
pub use blik::Blik;
pub mod link;
pub use link::Link;
pub mod sepa_debit;
pub use sepa_debit::SepaDebit;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
