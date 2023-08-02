#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices created by the subscription.
    pub acss_debit: Option<stripe_types::acss_debit::AcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscription.
    pub bancontact: Option<stripe_types::bancontact::Bancontact>,
    /// This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription.
    pub card: Option<stripe_types::card::Card>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subscription.
    pub customer_balance: Option<stripe_types::customer_balance::CustomerBalance>,
    /// This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription.
    pub konbini: Option<stripe_types::konbini::Konbini>,
    /// This sub-hash contains details about the ACH direct debit payment method options to pass to invoices created by the subscription.
    pub us_bank_account: Option<stripe_types::us_bank_account::UsBankAccount>,
}
