#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedInvoice {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: stripe_core::invoice::InvoiceId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedInvoiceObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedInvoice {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedInvoiceObject {
    Invoice,
}

impl DeletedInvoiceObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
        }
    }
}

impl std::str::FromStr for DeletedInvoiceObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "invoice" => Ok(Self::Invoice),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedInvoiceObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedInvoiceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedInvoiceObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedInvoiceObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedInvoiceObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeletedInvoiceObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<DeletedInvoiceObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(DeletedInvoiceObject::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for DeletedInvoice {
    type Id = stripe_core::invoice::InvoiceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
