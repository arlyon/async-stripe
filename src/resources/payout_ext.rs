use crate::config::{Client, Response};
use crate::ids::PayoutId;
use crate::resources::Payout;

impl Payout {
    /// Cancels the payout.
    ///
    /// For more details see [https://stripe.com/docs/api/payouts/cancel](https://stripe.com/docs/api/payouts/cancel).
    pub fn cancel(client: &Client, id: &PayoutId) -> Response<Payout> {
        client.post(&format!("/payouts/{}/cancel", id))
    }
}
