// ======================================
// This file was automatically generated.
// ======================================

use crate::params::{Expandable, Object};
use crate::resources::{Account, Currency};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "ConnectCollectionTransfer".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConnectCollectionTransfer {
    /// Amount transferred, in %s.
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
    type Id = ();
    fn id(&self) -> Self::Id {}
    fn object(&self) -> &'static str {
        "connect_collection_transfer"
    }
}
