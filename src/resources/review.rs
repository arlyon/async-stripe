use crate::params::{Paginated, Timestamp};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe review of a payment.
///
/// For more details see https://stripe.com/docs/api#review_object.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Review {
    pub id: String,
    pub object: String,
    pub charge: String,
    pub created: Timestamp,
    pub livemode: bool,
    pub open: bool,
    pub reason: String,
}

impl Paginated for Review {
    fn cursor(&self) -> &str {
        &self.id
    }
}
