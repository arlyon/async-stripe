// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::EventId;
use crate::params::{Expand, List, Object, Paginable, RangeQuery, Timestamp};
use crate::resources::{EventType, NotificationEventData};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "NotificationEvent".
///
/// For more details see <https://stripe.com/docs/api/events/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Event {
    /// Unique identifier for the object.
    pub id: EventId,

    /// The connected account that originates the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// The Stripe API version used to render `data`.
    ///
    /// This property is populated only for events on or after October 31, 2014.
    pub api_version: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    pub data: NotificationEventData,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Number of webhooks that haven't been successfully delivered (for example, to return a 20x response) to the URLs you specify.
    pub pending_webhooks: i64,

    /// Information on the API request that triggers the event.
    pub request: Option<NotificationEventRequest>,

    /// Description of the event (for example, `invoice.created` or `charge.refunded`).
    #[serde(rename = "type")]
    pub type_: EventType,
}

impl Event {
    /// List events, going back up to 30 days.
    ///
    /// Each event data is rendered according to Stripe API version at its creation time, specified in [event object](https://stripe.com/docs/api/events/object) `api_version` attribute (not according to your current Stripe API version or `Stripe-Version` header).
    pub fn list(client: &Client, params: &ListEvents<'_>) -> Response<List<Event>> {
        client.get_query("/events", params)
    }

    /// Retrieves the details of an event.
    ///
    /// Supply the unique identifier of the event, which you might have received in a webhook.
    pub fn retrieve(client: &Client, id: &EventId, expand: &[&str]) -> Response<Event> {
        client.get_query(&format!("/events/{}", id), Expand { expand })
    }
}

impl Object for Event {
    type Id = EventId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "event"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

/// The parameters for `Event::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListEvents<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Filter events by whether all webhooks were successfully delivered.
    ///
    /// If false, events which are still pending or have failed all delivery attempts to a webhook endpoint will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_success: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<EventId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<EventId>,

    /// A string containing a specific event name, or group of events using * as a wildcard.
    ///
    /// The list will be filtered to include only events with a matching event property.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<&'a str>,

    /// An array of up to 20 strings containing specific event names.
    ///
    /// The list will be filtered to include only events with a matching event property.
    /// You may pass either `type` or `types`, but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

impl<'a> ListEvents<'a> {
    pub fn new() -> Self {
        ListEvents {
            created: Default::default(),
            delivery_success: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            type_: Default::default(),
            types: Default::default(),
        }
    }
}
impl Paginable for ListEvents<'_> {
    type O = Event;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
