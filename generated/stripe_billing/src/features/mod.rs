#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Features {
    pub customer_update: stripe_billing::customer_update::CustomerUpdate,
    pub invoice_history: stripe_billing::invoice_history::InvoiceHistory,
    pub payment_method_update: stripe_billing::payment_method_update::PaymentMethodUpdate,
    pub subscription_cancel: stripe_billing::subscription_cancel::SubscriptionCancel,
    pub subscription_pause: stripe_billing::subscription_pause::SubscriptionPause,
    pub subscription_update: stripe_billing::subscription_update::SubscriptionUpdate,
}
