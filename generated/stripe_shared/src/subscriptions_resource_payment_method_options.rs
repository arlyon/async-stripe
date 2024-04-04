#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct SubscriptionsResourcePaymentMethodOptions {
    /// This sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to invoices created by the subscription.
    pub acss_debit: Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>,
    /// This sub-hash contains details about the Bancontact payment method options to pass to invoices created by the subscription.
    pub bancontact: Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>,
    /// This sub-hash contains details about the Card payment method options to pass to invoices created by the subscription.
    pub card: Option<stripe_shared::SubscriptionPaymentMethodOptionsCard>,
    /// This sub-hash contains details about the Bank transfer payment method options to pass to invoices created by the subscription.
    pub customer_balance: Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>,
    /// This sub-hash contains details about the Konbini payment method options to pass to invoices created by the subscription.
    pub konbini: Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>,
    /// This sub-hash contains details about the ACH direct debit payment method options to pass to invoices created by the subscription.
    pub us_bank_account: Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccount>,
}
