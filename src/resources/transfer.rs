use crate::ids::TransferId;
use crate::params::{Expandable, List, Metadata, Object, Timestamp};
use crate::resources::{Account, BalanceTransaction, Charge, Currency, TransferReversal};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Transfer".
///
/// For more details see [https://stripe.com/docs/api/transfers/object](https://stripe.com/docs/api/transfers/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transfer {
    /// Unique identifier for the object.
    pub id: TransferId,

    /// Amount in %s to be transferred.
    pub amount: i64,

    /// Amount in %s reversed (can be less than the amount attribute on the transfer if a partial reversal was issued).
    pub amount_reversed: i64,

    /// Balance transaction that describes the impact of this transfer on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Expandable<BalanceTransaction>>,

    /// Time that this record of the transfer was first created.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// ID of the Stripe account the transfer was sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<Expandable<Account>>,

    /// If the destination is a Stripe account, this will be the ID of the payment that the destination account received for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment: Option<Expandable<Charge>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// A set of key-value pairs that you can attach to a transfer object.
    ///
    /// It can be useful for storing additional information about the transfer in a structured format.
    pub metadata: Metadata,

    /// A list of reversals that have been applied to the transfer.
    pub reversals: List<TransferReversal>,

    /// Whether the transfer has been fully reversed.
    ///
    /// If the transfer is only partially reversed, this attribute will still be false.
    pub reversed: bool,

    /// ID of the charge or payment that was used to fund the transfer.
    ///
    /// If null, the transfer was funded from the available balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<Expandable<Charge>>,

    /// The source balance this transfer came from.
    ///
    /// One of `card` or `bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#grouping-transactions) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

impl Object for Transfer {
    type Id = TransferId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
    fn object(&self) -> &'static str {
        "transfer"
    }
}
