#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlatformTax {
    /// The Connected account that incurred this charge.
    pub account: String,
    /// Unique identifier for the object.
    pub id: stripe_types::platform_tax::PlatformTaxFeeId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: PlatformTaxObject,
    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,
    /// The type of tax (VAT).
    #[serde(rename = "type")]
    pub type_: String,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PlatformTaxObject {
    PlatformTaxFee,
}

impl PlatformTaxObject {
    pub fn as_str(self) -> &'static str {
        use PlatformTaxObject::*;
        match self {
            PlatformTaxFee => "platform_tax_fee",
        }
    }
}

impl std::str::FromStr for PlatformTaxObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PlatformTaxObject::*;
        match s {
            "platform_tax_fee" => Ok(PlatformTaxFee),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlatformTaxObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlatformTaxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PlatformTaxObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PlatformTaxObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PlatformTaxObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for PlatformTaxObject"))
    }
}
impl stripe_types::Object for PlatformTax {
    type Id = stripe_types::platform_tax::PlatformTaxFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PlatformTaxFeeId, "ptf_");
