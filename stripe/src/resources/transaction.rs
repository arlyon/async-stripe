use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe transaction.
///
/// For more details see https://stripe.com/docs/api#transaction_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transaction {
    // missing page in Stripe API
}
