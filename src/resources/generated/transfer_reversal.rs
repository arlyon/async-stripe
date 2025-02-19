// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::TransferReversalId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{BalanceTransaction, Currency, Refund, Transfer};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "TransferReversal".
///
/// For more details see <https://stripe.com/docs/api/transfer_reversals/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TransferReversal {
    /// Unique identifier for the object.
    pub id: TransferReversalId,

    /// Amount, in cents (or local equivalent).
    pub amount: i64,

    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Linked payment refund for the transfer reversal.
    pub destination_payment_refund: Option<Expandable<Refund>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// ID of the refund responsible for the transfer reversal.
    pub source_refund: Option<Expandable<Refund>>,

    /// ID of the transfer that was reversed.
    pub transfer: Expandable<Transfer>,
}

impl Object for TransferReversal {
    type Id = TransferReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "transfer_reversal"
    }
}
