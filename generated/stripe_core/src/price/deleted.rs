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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl stripe_types::Object for DeletedPrice {
    type Id = stripe_core::price::PriceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
