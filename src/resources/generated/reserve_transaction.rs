// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::ReserveTransactionId;
use crate::params::Object;
use crate::resources::Currency;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ReserveTransaction".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ReserveTransaction {
    /// Unique identifier for the object.
    pub id: ReserveTransactionId,

    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
}

impl Object for ReserveTransaction {
    type Id = ReserveTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "reserve_transaction"
    }
}
