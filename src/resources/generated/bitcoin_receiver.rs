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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// The amount of `currency` that you are collecting as payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The amount of `currency` to which `bitcoin_amount_received` has been converted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_received: Option<i64>,

    /// The amount of bitcoin that the customer should send to fill the receiver.
    ///
    /// The `bitcoin_amount` is denominated in Satoshi: there are 10^8 Satoshi in one bitcoin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitcoin_amount: Option<i64>,

    /// The amount of bitcoin that has been sent by the customer to this receiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitcoin_amount_received: Option<i64>,

    /// This URI can be displayed to the customer as a clickable link (to activate their bitcoin client) or as a QR code (for mobile wallets).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitcoin_uri: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) to which the bitcoin will be converted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The customer ID of the bitcoin receiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The customer's email address, set by the API call that creates the receiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// This flag is initially false and updates to true when the customer sends the `bitcoin_amount` to this receiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled: Option<bool>,

    /// A bitcoin address that is specific to this receiver.
    ///
    /// The customer can send bitcoin to this address to fill the receiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_address: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The ID of the payment created from the receiver, if any.
    ///
    /// Hidden when viewing the receiver with a publishable key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<String>,

    /// The refund address of this bitcoin receiver.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_address: Option<String>,

    /// A list with one entry for each time that the customer sent bitcoin to the receiver.
    ///
    /// Hidden when viewing the receiver with a publishable key.
    #[serde(default)]
    pub transactions: List<BitcoinTransaction>,

    /// This receiver contains uncaptured funds that can be used for a payment or refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncaptured_funds: Option<bool>,

    /// Indicate if this source is used for payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_for_payment: Option<bool>,
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
