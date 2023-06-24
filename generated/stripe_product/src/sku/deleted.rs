#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedSku {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_product::sku::SkuId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedSkuObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedSku {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedSkuObject {
    Sku,
}

impl DeletedSkuObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sku => "sku",
        }
    }
}

impl std::str::FromStr for DeletedSkuObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sku" => Ok(Self::Sku),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedSkuObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedSkuObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedSkuObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedSkuObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedSkuObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedSkuObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<DeletedSkuObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedSkuObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedSku {
    type Id = stripe_product::sku::SkuId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
