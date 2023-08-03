#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedInvoice {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_types::invoice::InvoiceId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedInvoiceObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DeletedInvoiceObject {
    Invoice,
}

impl DeletedInvoiceObject {
    pub fn as_str(self) -> &'static str {
        use DeletedInvoiceObject::*;
        match self {
            Invoice => "invoice",
        }
    }
}

impl std::str::FromStr for DeletedInvoiceObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DeletedInvoiceObject::*;
        match s {
            "invoice" => Ok(Invoice),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for DeletedInvoiceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for DeletedInvoiceObject"))
    }
}
impl stripe_types::Object for DeletedInvoice {
    type Id = stripe_types::invoice::InvoiceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
