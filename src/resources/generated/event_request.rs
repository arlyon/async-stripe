#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct EventRequest {
    /// ID of the API request that caused the event.
    ///
    /// If null, the event was automatic (e.g., Stripe's automatic subscription handling).
    /// Request logs are available in the [dashboard](https://dashboard.stripe.com/logs), but currently not in the API.
    pub id: Option<String>,
    /// The idempotency key transmitted during the request, if any.
    ///
    /// *Note: This property is populated only for events on or after May 23, 2017*.
    pub idempotency_key: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for EventRequest {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

impl crate::Object for EventRequest {
    type Id = Option<String>;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
