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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl stripe_types::Object for ConnectCollectionTransfer {
    type Id = stripe_core::connect_collection_transfer::ConnectCollectionTransferId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ConnectCollectionTransferId, "connct_");
