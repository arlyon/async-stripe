/// A `Transfer` object is created when you move funds between Stripe accounts as
/// part of Connect.
///
/// Before April 6, 2017, transfers also represented movement of funds from a
/// Stripe account to a card or bank account.
///
/// This behavior has since been split out into a [Payout](https://stripe.com/docs/api#payout_object) object, with corresponding payout endpoints.
/// For more information, read about the [transfer/payout split](https://stripe.com/docs/transfer-payout-split).  Related guide: [Creating Separate Charges and Transfers](https://stripe.com/docs/connect/charges-transfers).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Transfer {
    /// Amount in %s to be transferred.
    pub amount: i64,
    /// Amount in %s reversed (can be less than the amount attribute on the transfer if a partial reversal was issued).
    pub amount_reversed: i64,
    /// Balance transaction that describes the impact of this transfer on your account balance.
    pub balance_transaction:
        Option<crate::Expandable<crate::balance_transaction::BalanceTransaction>>,
    /// Time that this record of the transfer was first created.
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// ID of the Stripe account the transfer was sent to.
    pub destination: Option<crate::Expandable<crate::account::Account>>,
    /// If the destination is a Stripe account, this will be the ID of the payment that the destination account received for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment: Option<crate::Expandable<crate::charge::Charge>>,
    /// Unique identifier for the object.
    pub id: crate::transfer::TransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TransferObject,
    /// A list of reversals that have been applied to the transfer.
    pub reversals: crate::List<crate::transfer_reversal::TransferReversal>,
    /// Whether the transfer has been fully reversed.
    ///
    /// If the transfer is only partially reversed, this attribute will still be false.
    pub reversed: bool,
    /// ID of the charge or payment that was used to fund the transfer.
    ///
    /// If null, the transfer was funded from the available balance.
    pub source_transaction: Option<crate::Expandable<crate::charge::Charge>>,
    /// The source balance this transfer came from.
    ///
    /// One of `card`, `fpx`, or `bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Transfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum TransferObject {
    Transfer,
}

impl TransferObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Transfer => "transfer",
        }
    }
}

impl AsRef<str> for TransferObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransferObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for Transfer {
    type Id = crate::transfer::TransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(TransferId, "tr_");
pub mod requests;
