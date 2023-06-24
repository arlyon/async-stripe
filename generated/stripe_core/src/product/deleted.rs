#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedProduct {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_core::product::ProductId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedProductObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedProduct {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedProductObject {
    Product,
}

impl DeletedProductObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Product => "product",
        }
    }
}

impl std::str::FromStr for DeletedProductObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "product" => Ok(Self::Product),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedProductObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedProductObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedProductObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedProductObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedProductObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedProductObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<DeletedProductObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedProductObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedProduct {
    type Id = stripe_core::product::ProductId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
