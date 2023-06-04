#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeletedSku {
    /// Always true for a deleted object.
    pub deleted: bool,
    /// Unique identifier for the object.
    pub id: crate::sku::SkuId,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for DeletedSku {
    type Id = crate::sku::SkuId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
