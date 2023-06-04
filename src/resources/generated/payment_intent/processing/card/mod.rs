#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notification: Option<
        crate::payment_intent::processing::card::customer_notification::CustomerNotification,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Card {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod customer_notification;
pub use customer_notification::CustomerNotification;
