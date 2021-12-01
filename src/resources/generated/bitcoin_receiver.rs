// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::BitcoinReceiverId;
use crate::params::{List, Metadata, Object, Timestamp};
use crate::resources::{BitcoinTransaction, Currency};

/// The resource representing a Stripe "BitcoinReceiver".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BitcoinReceiver {
    /// Unique identifier for the object.
    pub id: BitcoinReceiverId,

    /// True when this bitcoin receiver has received a non-zero amount of bitcoin.
    pub active: Box<Option<bool>>,

    /// The amount of `currency` that you are collecting as payment.
    pub amount: Box<Option<i64>>,

    /// The amount of `currency` to which `bitcoin_amount_received` has been converted.
    pub amount_received: Box<Option<i64>>,

    /// The amount of bitcoin that the customer should send to fill the receiver.
    ///
    /// The `bitcoin_amount` is denominated in Satoshi: there are 10^8 Satoshi in one bitcoin.
    pub bitcoin_amount: Box<Option<i64>>,

    /// The amount of bitcoin that has been sent by the customer to this receiver.
    pub bitcoin_amount_received: Box<Option<i64>>,

    /// This URI can be displayed to the customer as a clickable link (to activate their bitcoin client) or as a QR code (for mobile wallets).
    pub bitcoin_uri: Box<Option<String>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) to which the bitcoin will be converted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The customer ID of the bitcoin receiver.
    pub customer: Box<Option<String>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Box<Option<String>>,

    /// The customer's email address, set by the API call that creates the receiver.
    pub email: Box<Option<String>>,

    /// This flag is initially false and updates to true when the customer sends the `bitcoin_amount` to this receiver.
    pub filled: Box<Option<bool>>,

    /// A bitcoin address that is specific to this receiver.
    ///
    /// The customer can send bitcoin to this address to fill the receiver.
    pub inbound_address: Box<Option<String>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: Box<Option<bool>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The ID of the payment created from the receiver, if any.
    ///
    /// Hidden when viewing the receiver with a publishable key.
    pub payment: Box<Option<String>>,

    /// The refund address of this bitcoin receiver.
    pub refund_address: Box<Option<String>>,

    /// A list with one entry for each time that the customer sent bitcoin to the receiver.
    ///
    /// Hidden when viewing the receiver with a publishable key.
    #[serde(default)]
    pub transactions: List<BitcoinTransaction>,

    /// This receiver contains uncaptured funds that can be used for a payment or refunded.
    pub uncaptured_funds: Box<Option<bool>>,

    /// Indicate if this source is used for payment.
    pub used_for_payment: Box<Option<bool>>,
}

impl Object for BitcoinReceiver {
    type Id = BitcoinReceiverId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "bitcoin_receiver"
    }
}
