/// [Tax codes](https://stripe.com/docs/tax/tax-categories) classify goods and services for tax purposes.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxCode {
    /// A detailed description of which types of products the tax code represents.
    pub description: String,
    /// Unique identifier for the object.
    pub id: stripe_types::tax_code::TaxCodeId,
    /// A short name for the tax code.
    pub name: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: TaxCodeObject,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaxCodeObject {
    TaxCode,
}

impl TaxCodeObject {
    pub fn as_str(self) -> &'static str {
        use TaxCodeObject::*;
        match self {
            TaxCode => "tax_code",
        }
    }
}

impl std::str::FromStr for TaxCodeObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxCodeObject::*;
        match s {
            "tax_code" => Ok(TaxCode),
            _ => Err(()),
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
impl serde::Serialize for TaxCodeObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxCodeObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for TaxCodeObject"))
    }
}
impl stripe_types::Object for TaxCode {
    type Id = stripe_types::tax_code::TaxCodeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(TaxCodeId, "txcd_");
