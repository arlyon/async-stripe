use crate::client::{Client, Response};
use crate::ids::AccountId;
use crate::resources::Balance;

impl Balance {
    /// Retrieves balance object by AccountId. Does not change stripe_account of the client.
    ///
    /// For more details see <https://stripe.com/docs/api/balance/balance_retrieve>.
    pub fn retrieve(client: &Client, account_id: Option<AccountId>) -> Response<Balance> {
        match account_id {
            Some(account_id) => client.clone().with_stripe_account(account_id).get("/balance"),
            None => client.get("/balance"),
        }
    }
}
