/// Indicates the type of tax rate applied to the taxable amount.
/// This value can be `null` when no tax applies to the location.
/// This field is only present for TaxRates created by Stripe Tax.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum RateType {
    FlatAmount,
    Percentage,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl RateType {
    pub fn as_str(&self) -> &str {
        use RateType::*;
        match self {
            FlatAmount => "flat_amount",
            Percentage => "percentage",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for RateType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RateType::*;
        match s {
            "flat_amount" => Ok(FlatAmount),
            "percentage" => Ok(Percentage),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "RateType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for RateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for RateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(RateType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for RateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for RateType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<RateType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(RateType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(RateType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for RateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
