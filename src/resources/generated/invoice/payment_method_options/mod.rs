#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    pub acss_debit: Option<crate::invoice::payment_method_options::acss_debit::AcssDebit>,
    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    pub bancontact: Option<crate::invoice::payment_method_options::bancontact::Bancontact>,
    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    pub card: Option<crate::invoice::payment_method_options::card::Card>,
    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    pub customer_balance:
        Option<crate::invoice::payment_method_options::customer_balance::CustomerBalance>,
    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    pub konbini: Option<crate::invoice::payment_method_options::konbini::Konbini>,
    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    pub us_bank_account:
        Option<crate::invoice::payment_method_options::us_bank_account::UsBankAccount>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod acss_debit;
pub use acss_debit::AcssDebit;
pub mod bancontact;
pub use bancontact::Bancontact;
pub mod card;
pub use card::Card;
pub mod customer_balance;
pub use customer_balance::CustomerBalance;
pub mod konbini;
pub use konbini::Konbini;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
