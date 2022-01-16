use crate::config::{Client, Response};
use crate::ids::{PayoutDestinationId, PayoutId};
use crate::params::Object;
use crate::resources::{Payout, PayoutDestinationUnion};

impl Payout {
    /// Cancels the payout.
    ///
    /// For more details see <https://stripe.com/docs/api/payouts/cancel>.
    pub fn cancel(client: &Client, id: &PayoutId, idem_key: Option<&str>) -> Response<Payout> {
        client.post(&format!("/payouts/{}/cancel", id), idem_key)
    }
}

impl Object for PayoutDestinationUnion {
    type Id = PayoutDestinationId;
    fn id(&self) -> Self::Id {
        match self {
            PayoutDestinationUnion::BankAccount(x) => PayoutDestinationId::BankAccount(x.id()),
            PayoutDestinationUnion::Card(x) => PayoutDestinationId::Card(x.id()),
        }
    }
    fn object(&self) -> &'static str {
        match self {
            PayoutDestinationUnion::BankAccount(x) => x.object(),
            PayoutDestinationUnion::Card(x) => x.object(),
        }
    }
}
