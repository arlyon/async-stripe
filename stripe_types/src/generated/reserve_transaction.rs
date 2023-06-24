#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct ReserveTransaction {
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_types::reserve_transaction::ReserveTransactionId,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for ReserveTransactionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "reserve_transaction" => Ok(Self::ReserveTransaction),

            _ => Err(()),
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
impl serde::Serialize for ReserveTransactionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReserveTransactionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for ReserveTransactionObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for ReserveTransactionObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<ReserveTransactionObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ReserveTransactionObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for ReserveTransaction {
    type Id = stripe_types::reserve_transaction::ReserveTransactionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(ReserveTransactionId, "rtx_");
