#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandatePixPaymentSchedule {
    Halfyearly,
    Monthly,
    Quarterly,
    Weekly,
    Yearly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandatePixPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use MandatePixPaymentSchedule::*;
        match self {
            Halfyearly => "halfyearly",
            Monthly => "monthly",
            Quarterly => "quarterly",
            Weekly => "weekly",
            Yearly => "yearly",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandatePixPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandatePixPaymentSchedule::*;
        match s {
            "halfyearly" => Ok(Halfyearly),
            "monthly" => Ok(Monthly),
            "quarterly" => Ok(Quarterly),
            "weekly" => Ok(Weekly),
            "yearly" => Ok(Yearly),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "MandatePixPaymentSchedule");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandatePixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandatePixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandatePixPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandatePixPaymentSchedule)).finish_non_exhaustive()
    }
}
impl serde::Serialize for MandatePixPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandatePixPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandatePixPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandatePixPaymentSchedule::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandatePixPaymentSchedule);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandatePixPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
