#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct InvoicesPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    pub acss_debit: Option<stripe_shared::InvoicePaymentMethodOptionsAcssDebit>,
    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    pub bancontact: Option<stripe_shared::InvoicePaymentMethodOptionsBancontact>,
    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    pub card: Option<stripe_shared::InvoicePaymentMethodOptionsCard>,
    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    pub customer_balance: Option<stripe_shared::InvoicePaymentMethodOptionsCustomerBalance>,
    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    pub konbini: Option<stripe_shared::InvoicePaymentMethodOptionsKonbini>,
    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    pub us_bank_account: Option<stripe_shared::InvoicePaymentMethodOptionsUsBankAccount>,
}
