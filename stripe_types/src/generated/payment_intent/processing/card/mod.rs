#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notification: Option<
        stripe_types::payment_intent::processing::card::customer_notification::CustomerNotification,
    >,
}
pub mod customer_notification;
pub use customer_notification::CustomerNotification;
