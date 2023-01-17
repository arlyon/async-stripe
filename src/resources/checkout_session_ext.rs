use crate::client::{Client, Response};
use crate::ids::CheckoutSessionId;
use crate::params::Expand;
use crate::resources::CheckoutSession;

impl CheckoutSession {
    /// Retrieves a Session object.
    ///
    /// For more details see <https://stripe.com/docs/api/checkout/sessions/retrieve>.
    pub fn retrieve<'a>(
        client: &'a Client,
        id: &'_ CheckoutSessionId,
        expand: &'a [&str],
    ) -> Response<'a, CheckoutSession> {
        client.get_query(&format!("/checkout/sessions/{}", id), &Expand { expand })
    }
}
