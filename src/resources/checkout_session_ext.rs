use crate::{
    client::{Client, Response},
    ids::CheckoutSessionId,
    params::Expand,
    resources::CheckoutSession,
};

impl CheckoutSession {
    /// Retrieves a Session object.
    ///
    /// For more details see <https://stripe.com/docs/api/checkout/sessions/retrieve>.
    pub fn retrieve(
        client: &Client,
        id: &CheckoutSessionId,
        expand: &[&str],
    ) -> Response<CheckoutSession> {
        client.get_query(&format!("/checkout/sessions/{}", id), &Expand { expand })
    }
}
