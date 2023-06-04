#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReserveTransaction {
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: crate::reserve_transaction::ReserveTransactionId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ReserveTransactionObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReserveTransaction {
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
pub enum ReserveTransactionObject {
    ReserveTransaction,
}

impl ReserveTransactionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ReserveTransaction => "reserve_transaction",
        }
    }
}

impl AsRef<str> for ReserveTransactionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReserveTransactionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for ReserveTransaction {
    type Id = crate::reserve_transaction::ReserveTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(ReserveTransactionId, "rtx_");
