/// Interval (or event) to which the amount applies.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingCardSpendingLimitInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingCardSpendingLimitInterval {
    pub fn as_str(&self) -> &str {
        use IssuingCardSpendingLimitInterval::*;
        match self {
            AllTime => "all_time",
            Daily => "daily",
            Monthly => "monthly",
            PerAuthorization => "per_authorization",
            Weekly => "weekly",
            Yearly => "yearly",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingCardSpendingLimitInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingCardSpendingLimitInterval::*;
        match s {
            "all_time" => Ok(AllTime),
            "daily" => Ok(Daily),
            "monthly" => Ok(Monthly),
            "per_authorization" => Ok(PerAuthorization),
            "weekly" => Ok(Weekly),
            "yearly" => Ok(Yearly),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingCardSpendingLimitInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingCardSpendingLimitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingCardSpendingLimitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardSpendingLimitInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingCardSpendingLimitInterval)).finish_non_exhaustive()
    }
}
impl serde::Serialize for IssuingCardSpendingLimitInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingCardSpendingLimitInterval {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingCardSpendingLimitInterval> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingCardSpendingLimitInterval::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingCardSpendingLimitInterval);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingCardSpendingLimitInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
