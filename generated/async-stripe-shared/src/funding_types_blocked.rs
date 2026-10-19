/// Card funding types to block for this Checkout Session.
/// Supported values are `credit`, `debit`, and `prepaid`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FundingTypesBlocked {
    Credit,
    Debit,
    Prepaid,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl FundingTypesBlocked {
    pub fn as_str(&self) -> &str {
        use FundingTypesBlocked::*;
        match self {
            Credit => "credit",
            Debit => "debit",
            Prepaid => "prepaid",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for FundingTypesBlocked {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FundingTypesBlocked::*;
        match s {
            "credit" => Ok(Credit),
            "debit" => Ok(Debit),
            "prepaid" => Ok(Prepaid),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "FundingTypesBlocked");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for FundingTypesBlocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for FundingTypesBlocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for FundingTypesBlocked {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(FundingTypesBlocked)).finish_non_exhaustive()
    }
}
impl serde::Serialize for FundingTypesBlocked {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for FundingTypesBlocked {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<FundingTypesBlocked> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(FundingTypesBlocked::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(FundingTypesBlocked);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for FundingTypesBlocked {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
