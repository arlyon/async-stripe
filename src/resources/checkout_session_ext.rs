use crate::client::{Client, Response};
use crate::ids::CheckoutSessionId;
use crate::params::Expand;
use crate::resources::CheckoutSession;

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
}
