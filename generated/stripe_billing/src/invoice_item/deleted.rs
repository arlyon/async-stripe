#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct DeletedInvoiceItem {
    /// Always true for a deleted object.
    deleted: stripe_types::AlwaysTrue,
    /// Unique identifier for the object.
    pub id: stripe_billing::invoice_item::InvoiceitemId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: DeletedInvoiceItemObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DeletedInvoiceItemObject {
    Invoiceitem,
}

impl DeletedInvoiceItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoiceitem => "invoiceitem",
        }
    }
}

impl std::str::FromStr for DeletedInvoiceItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "invoiceitem" => Ok(Self::Invoiceitem),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for DeletedInvoiceItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DeletedInvoiceItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for DeletedInvoiceItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletedInvoiceItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for DeletedInvoiceItemObject"))
    }
}
impl stripe_types::Object for DeletedInvoiceItem {
    type Id = stripe_billing::invoice_item::InvoiceitemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
