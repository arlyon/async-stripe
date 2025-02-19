// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::ConnectCollectionTransferId;
use crate::params::{Expandable, Object};
use crate::resources::{Account, Currency};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ConnectCollectionTransfer".
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ConnectCollectionTransfer {
    /// Unique identifier for the object.
    pub id: ConnectCollectionTransferId,

    /// Amount transferred, in cents (or local equivalent).
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// ID of the account that funds are being collected for.
    pub destination: Expandable<Account>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl Object for ConnectCollectionTransfer {
    type Id = ConnectCollectionTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "connect_collection_transfer"
    }
}
