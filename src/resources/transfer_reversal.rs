// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::TransferReversalId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{BalanceTransaction, Currency, Refund, Transfer};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "TransferReversal".
///
/// For more details see [https://stripe.com/docs/api/transfer_reversals/object](https://stripe.com/docs/api/transfer_reversals/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TransferReversal {
    /// Unique identifier for the object.
    pub id: TransferReversalId,

    /// Amount, in %s.
    pub amount: i64,

    /// Balance transaction that describes the impact on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_refund: Option<Expandable<Refund>>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// ID of the refund responsible for the transfer reversal.
    #[serde(skip_serializing_if = "Option::is_none")]
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
