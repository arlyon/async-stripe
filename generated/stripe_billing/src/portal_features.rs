#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalFeatures {
    pub customer_update: stripe_billing::PortalCustomerUpdate,
    pub invoice_history: stripe_billing::PortalInvoiceList,
    pub payment_method_update: stripe_billing::PortalPaymentMethodUpdate,
    pub subscription_cancel: stripe_billing::PortalSubscriptionCancel,
    pub subscription_pause: stripe_billing::PortalSubscriptionPause,
    pub subscription_update: stripe_billing::PortalSubscriptionUpdate,
}
