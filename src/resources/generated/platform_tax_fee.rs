#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PlatformTaxFee {
    /// The Connected account that incurred this charge.
    pub account: String,
    /// Unique identifier for the object.
    pub id: crate::platform_tax_fee::PlatformTaxFeeId,
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
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PlatformTaxFee {
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
impl crate::Object for PlatformTaxFee {
    type Id = crate::platform_tax_fee::PlatformTaxFeeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(PlatformTaxFeeId, "ptf_");
