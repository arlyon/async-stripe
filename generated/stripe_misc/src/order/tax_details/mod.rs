#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TaxDetails {
    /// Describes the purchaser's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    pub tax_exempt: TaxDetailsTaxExempt,
    /// The purchaser's tax IDs to be used in calculation of tax for this Order.
    pub tax_ids: Vec<stripe_misc::order::tax_details::tax_id::TaxId>,
}
/// Describes the purchaser's tax exemption status.
///
/// One of `none`, `exempt`, or `reverse`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl TaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for TaxDetailsTaxExempt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exempt" => Ok(Self::Exempt),
            "none" => Ok(Self::None),
            "reverse" => Ok(Self::Reverse),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for TaxDetailsTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TaxDetailsTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for TaxDetailsTaxExempt"))
    }
}
pub mod tax_id;
pub use tax_id::TaxId;
