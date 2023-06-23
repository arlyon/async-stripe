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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Event {
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
    pub data: stripe_types::event_data::EventData,
    /// Unique identifier for the object.
    pub id: stripe_core::event::EventId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: EventObject,
    /// Number of webhooks that have yet to be successfully delivered (i.e., to return a 20x response) to the URLs you've specified.
    pub pending_webhooks: i64,
    /// Information on the API request that instigated the event.
    pub request: Option<stripe_types::event_request::EventRequest>,
    /// Description of the event (e.g., `invoice.created` or `charge.refunded`).
    #[serde(rename = "type")]
    pub type_: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Event {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum EventObject {
    Event,
}

impl EventObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Event => "event",
        }
    }
}

impl AsRef<str> for EventObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for EventObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl stripe_types::Object for Event {
    type Id = stripe_core::event::EventId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(EventId, "evt_");
pub mod requests;
