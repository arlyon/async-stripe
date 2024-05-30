use crate::client::{Client, Response};
use crate::ids::CheckoutSessionId;
use crate::params::Expand;
use crate::resources::CheckoutSession;
use crate::{CheckoutSessionItem, List};

/// The parameters for `CheckoutSession::retrieve_line_items`.
#[derive(Clone, Debug, serde::Serialize, Default)]
pub struct RetrieveCheckoutSessionLineItems {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<CheckoutSessionId>,

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
    pub starting_after: Option<CheckoutSessionId>,
}

impl CheckoutSession {
    /// Retrieves a Session object.
    ///
    /// For more details see <https://stripe.com/docs/api/checkout/sessions/retrieve>.
    pub fn retrieve(
        client: &Client,
        id: &CheckoutSessionId,
        expand: &[&str],
    ) -> Response<CheckoutSession> {
        client.get_query(&format!("/checkout/sessions/{}", id), Expand { expand })
    }

    /// Expires a checkout session.
    ///
    /// For more details see <https://stripe.com/docs/api/checkout/sessions/expire>.
    pub fn expire(client: &Client, id: &CheckoutSessionId) -> Response<CheckoutSession> {
        client.post(&format!("/checkout/sessions/{}/expire", id))
    }

    /// Retrieves a Checkout Session's line items
    ///
    /// For more details see <https://docs.stripe.com/api/checkout/sessions/line_items>
    pub fn retrieve_line_items(
        client: &Client,
        id: &CheckoutSessionId,
        params: &RetrieveCheckoutSessionLineItems,
    ) -> Response<List<CheckoutSessionItem>> {
        client.get_query(&format!("/checkout/sessions/{}/line_items", id), params)
    }
}
