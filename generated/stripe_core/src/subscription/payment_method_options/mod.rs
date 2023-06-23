#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices created by the subscription.
    pub acss_debit: Option<stripe_core::invoice::payment_method_options::acss_debit::AcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscription.
    pub bancontact: Option<stripe_core::invoice::payment_method_options::bancontact::Bancontact>,
    /// This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription.
    pub card: Option<stripe_core::subscription::payment_method_options::card::Card>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subscription.
    pub customer_balance:
        Option<stripe_core::invoice::payment_method_options::customer_balance::CustomerBalance>,
    /// This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription.
    pub konbini: Option<stripe_core::invoice::payment_method_options::konbini::Konbini>,
    /// This sub-hash contains details about the ACH direct debit payment method options to pass to invoices created by the subscription.
    pub us_bank_account:
        Option<stripe_core::invoice::payment_method_options::us_bank_account::UsBankAccount>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodOptions {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod card;
pub use card::Card;
