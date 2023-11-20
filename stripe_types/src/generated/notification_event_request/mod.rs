#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct NotificationEventRequest {
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
