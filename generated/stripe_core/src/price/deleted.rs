#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedPrice {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_core::price::PriceId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedPriceObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedPrice {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedPriceObject {
    Price,
}

impl DeletedPriceObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Price => "price",
        }
    }
}

impl std::str::FromStr for DeletedPriceObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "price" => Ok(Self::Price),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedPriceObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedPriceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedPriceObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedPriceObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedPriceObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedPriceObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<DeletedPriceObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedPriceObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedPrice {
    type Id = stripe_core::price::PriceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
