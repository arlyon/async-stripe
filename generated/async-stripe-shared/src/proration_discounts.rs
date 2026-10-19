/// Controls how invoices and invoice items display proration amounts and discount amounts.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ProrationDiscounts {
    Included,
    Itemized,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ProrationDiscounts {
    pub fn as_str(&self) -> &str {
        use ProrationDiscounts::*;
        match self {
            Included => "included",
            Itemized => "itemized",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ProrationDiscounts {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ProrationDiscounts::*;
        match s {
            "included" => Ok(Included),
            "itemized" => Ok(Itemized),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "ProrationDiscounts");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ProrationDiscounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ProrationDiscounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ProrationDiscounts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ProrationDiscounts)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ProrationDiscounts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ProrationDiscounts {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ProrationDiscounts> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ProrationDiscounts::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ProrationDiscounts);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ProrationDiscounts {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
