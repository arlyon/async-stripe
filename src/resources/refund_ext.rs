use crate::client::{Client, Response};
use crate::{generated::core::refund::Refund, RefundId};
impl Refund {
    /// Cancel a refund
    ///
    /// For more details see <https://docs.stripe.com/api/refunds/cancel>.
    pub fn cancel(client: &Client, id: &RefundId) -> Response<Refund> {
        client.post(&format!("/refunds/{id}/cancel"))
    }
}
