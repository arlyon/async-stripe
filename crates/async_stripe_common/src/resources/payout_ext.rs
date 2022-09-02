use crate::{
    client::{Client, Response},
    ids::{PayoutDestinationId, PayoutId},
    params::Object,
    resources::{Payout, PayoutDestinationUnion},
};

impl Payout {
    /// Cancels the payout.
    ///
    /// For more details see <https://stripe.com/docs/api/payouts/cancel>.
    pub fn cancel(client: &Client, id: &PayoutId) -> Response<Payout> {
        client.post(&format!("/payouts/{}/cancel", id))
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
