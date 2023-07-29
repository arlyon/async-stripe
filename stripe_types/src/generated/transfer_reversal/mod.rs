/// [Stripe Connect](https://stripe.com/docs/connect) platforms can reverse transfers made to a
/// connected account, either entirely or partially, and can also specify whether
/// to refund any related application fees.
///
/// Transfer reversals add to the platform's balance and subtract from the destination account's balance.  Reversing a transfer that was made for a [destination charge](/docs/connect/destination-charges) is allowed only up to the amount of the charge.
/// It is possible to reverse a [transfer_group](https://stripe.com/docs/connect/charges-transfers#transfer-options) transfer only if the destination account has enough balance to cover the reversal.  Related guide: [Reversing Transfers](https://stripe.com/docs/connect/charges-transfers#reversing-transfers).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TransferReversal {
    /// Amount, in %s.
    pub amount: i64,
    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction:
        Option<stripe_types::Expandable<stripe_types::balance_transaction::BalanceTransaction>>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Linked payment refund for the transfer reversal.
    pub destination_payment_refund: Option<stripe_types::Expandable<stripe_types::refund::Refund>>,
    /// Unique identifier for the object.
    pub id: stripe_types::transfer_reversal::TransferReversalId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TransferReversalObject,
    /// ID of the refund responsible for the transfer reversal.
    pub source_refund: Option<stripe_types::Expandable<stripe_types::refund::Refund>>,
    /// ID of the transfer that was reversed.
    pub transfer: stripe_types::Expandable<stripe_types::transfer::Transfer>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransferReversal {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TransferReversalObject {
    TransferReversal,
}

impl TransferReversalObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TransferReversal => "transfer_reversal",
        }
    }
}

impl std::str::FromStr for TransferReversalObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "transfer_reversal" => Ok(Self::TransferReversal),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TransferReversalObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TransferReversalObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TransferReversalObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TransferReversalObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TransferReversalObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TransferReversalObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<TransferReversalObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TransferReversalObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for TransferReversal {
    type Id = stripe_types::transfer_reversal::TransferReversalId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TransferReversalId, "trr_");
