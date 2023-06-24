#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ConnectCollectionTransfer {
    /// Amount transferred, in %s.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the account that funds are being collected for.
    pub destination: stripe_types::Expandable<stripe_core::account::Account>,
    /// Unique identifier for the object.
    pub id: stripe_core::connect_collection_transfer::ConnectCollectionTransferId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ConnectCollectionTransferObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ConnectCollectionTransfer {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ConnectCollectionTransferObject {
    ConnectCollectionTransfer,
}

impl ConnectCollectionTransferObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ConnectCollectionTransfer => "connect_collection_transfer",
        }
    }
}

impl std::str::FromStr for ConnectCollectionTransferObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "connect_collection_transfer" => Ok(Self::ConnectCollectionTransfer),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for ConnectCollectionTransferObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ConnectCollectionTransferObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ConnectCollectionTransferObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConnectCollectionTransferObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ConnectCollectionTransferObject")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ConnectCollectionTransferObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ConnectCollectionTransferObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ConnectCollectionTransferObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for ConnectCollectionTransfer {
    type Id = stripe_core::connect_collection_transfer::ConnectCollectionTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ConnectCollectionTransferId, "connct_");
