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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl stripe_types::Object for DeletedInvoice {
    type Id = stripe_core::invoice::InvoiceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
