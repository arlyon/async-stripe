/// Events are our way of letting you know when something interesting happens in
/// your account.
///
/// When an interesting event occurs, we create a new `Event` object.
/// For example, when a charge succeeds, we create a `charge.succeeded` event; and when an invoice payment attempt fails, we create an `invoice.payment_failed` event.
/// Note that many API requests may cause multiple events to be created.
/// For example, if you create a new subscription for a customer, you will receive both a `customer.subscription.created` event and a `charge.succeeded` event.  Events occur when the state of another API resource changes.
/// The state of that resource at the time of the change is embedded in the event's data field.
/// For example, a `charge.succeeded` event will contain a charge, and an `invoice.payment_failed` event will contain an invoice.  As with other API resources, you can use endpoints to retrieve an [individual event](https://stripe.com/docs/api#retrieve_event) or a [list of events](https://stripe.com/docs/api#list_events) from the API.
/// We also have a separate [webhooks](http://en.wikipedia.org/wiki/Webhook) system for sending the `Event` objects directly to an endpoint on your server.
/// Webhooks are managed in your [account settings](https://dashboard.stripe.com/account/webhooks), and our [Using Webhooks](https://stripe.com/docs/webhooks) guide will help you get set up.  When using [Connect](https://stripe.com/docs/connect), you can also receive notifications of events that occur in connected accounts.
/// For these events, there will be an additional `account` attribute in the received `Event` object.  **NOTE:** Right now, access to events through the [Retrieve Event API](https://stripe.com/docs/api#retrieve_event) is guaranteed only for 30 days.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct NotificationEvent {
    /// The connected account that originated the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The Stripe API version used to render `data`.
    ///
    /// *Note: This property is populated only for events on or after October 31, 2014*.
    pub api_version: Option<String>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub data: stripe_types::NotificationEventData,
    /// Unique identifier for the object.
    pub id: stripe_types::notification_event::EventId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Number of webhooks that have yet to be successfully delivered (i.e., to return a 20x response) to the URLs you've specified.
    pub pending_webhooks: i64,
    /// Information on the API request that instigated the event.
    pub request: Option<stripe_types::NotificationEventRequest>,
    /// Description of the event (e.g., `invoice.created` or `charge.refunded`).
    #[serde(rename = "type")]
    pub type_: String,
}
impl stripe_types::Object for NotificationEvent {
    type Id = stripe_types::notification_event::EventId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(EventId, "evt_");
