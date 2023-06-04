/// [Tax codes](https://stripe.com/docs/tax/tax-categories) classify goods and services for tax purposes.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct TaxCode {
    /// A detailed description of which types of products the tax code represents.
    pub description: String,
    /// Unique identifier for the object.
    pub id: crate::tax_code::TaxCodeId,
    /// A short name for the tax code.
    pub name: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TaxCodeObject,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for TaxCode {
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
pub enum TaxCodeObject {
    TaxCode,
}

impl TaxCodeObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TaxCode => "tax_code",
        }
    }
}

impl AsRef<str> for TaxCodeObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxCodeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for TaxCode {
    type Id = crate::tax_code::TaxCodeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(TaxCodeId, "txcd_");
pub mod requests;
