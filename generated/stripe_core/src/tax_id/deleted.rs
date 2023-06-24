#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedTaxId {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_core::tax_id::TaxIdId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedTaxIdObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedTaxId {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedTaxIdObject {
    TaxId,
}

impl DeletedTaxIdObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TaxId => "tax_id",
        }
    }
}

impl std::str::FromStr for DeletedTaxIdObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tax_id" => Ok(Self::TaxId),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedTaxIdObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedTaxIdObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedTaxIdObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedTaxIdObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedTaxIdObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedTaxIdObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<DeletedTaxIdObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedTaxIdObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedTaxId {
    type Id = stripe_core::tax_id::TaxIdId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
