#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Features {
pub customer_update: stripe_billing::billing_portal::configuration::features::customer_update::CustomerUpdate,
pub invoice_history: stripe_billing::billing_portal::configuration::features::invoice_history::InvoiceHistory,
pub payment_method_update: stripe_billing::billing_portal::configuration::features::payment_method_update::PaymentMethodUpdate,
pub subscription_cancel: stripe_billing::billing_portal::configuration::features::subscription_cancel::SubscriptionCancel,
pub subscription_pause: stripe_billing::billing_portal::configuration::features::subscription_pause::SubscriptionPause,
pub subscription_update: stripe_billing::billing_portal::configuration::features::subscription_update::SubscriptionUpdate,

}
pub mod customer_update;
pub use customer_update::CustomerUpdate;
pub mod invoice_history;
pub use invoice_history::InvoiceHistory;
pub mod payment_method_update;
pub use payment_method_update::PaymentMethodUpdate;
pub mod subscription_cancel;
pub use subscription_cancel::SubscriptionCancel;
pub mod subscription_pause;
pub use subscription_pause::SubscriptionPause;
pub mod subscription_update;
pub use subscription_update::SubscriptionUpdate;
