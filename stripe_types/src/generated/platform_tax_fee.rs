#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PlatformTaxFee {
    /// The Connected account that incurred this charge.
    pub account: String,
    /// Unique identifier for the object.
    pub id: stripe_types::platform_tax_fee::PlatformTaxFeeId,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: PlatformTaxFeeObject,
    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,
    /// The type of tax (VAT).
    #[serde(rename = "type")]
    pub type_: String,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PlatformTaxFeeObject {
    PlatformTaxFee,
}

impl PlatformTaxFeeObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PlatformTaxFee => "platform_tax_fee",
        }
    }
}

impl std::str::FromStr for PlatformTaxFeeObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "platform_tax_fee" => Ok(Self::PlatformTaxFee),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PlatformTaxFeeObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PlatformTaxFeeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PlatformTaxFeeObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PlatformTaxFeeObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PlatformTaxFeeObject"))
    }
}
impl stripe_types::Object for PlatformTaxFee {
    type Id = stripe_types::platform_tax_fee::PlatformTaxFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PlatformTaxFeeId, "ptf_");
