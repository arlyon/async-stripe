/// The periodicity at which payments will be collected. Defaults to `adhoc`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MandatePaytoPaymentSchedule {
    Adhoc,
    Annual,
    Daily,
    Fortnightly,
    Monthly,
    Quarterly,
    SemiAnnual,
    Weekly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MandatePaytoPaymentSchedule {
    pub fn as_str(&self) -> &str {
        use MandatePaytoPaymentSchedule::*;
        match self {
            Adhoc => "adhoc",
            Annual => "annual",
            Daily => "daily",
            Fortnightly => "fortnightly",
            Monthly => "monthly",
            Quarterly => "quarterly",
            SemiAnnual => "semi_annual",
            Weekly => "weekly",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MandatePaytoPaymentSchedule {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MandatePaytoPaymentSchedule::*;
        match s {
            "adhoc" => Ok(Adhoc),
            "annual" => Ok(Annual),
            "daily" => Ok(Daily),
            "fortnightly" => Ok(Fortnightly),
            "monthly" => Ok(Monthly),
            "quarterly" => Ok(Quarterly),
            "semi_annual" => Ok(SemiAnnual),
            "weekly" => Ok(Weekly),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MandatePaytoPaymentSchedule"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MandatePaytoPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MandatePaytoPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MandatePaytoPaymentSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MandatePaytoPaymentSchedule)).finish_non_exhaustive()
    }
}
impl serde::Serialize for MandatePaytoPaymentSchedule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MandatePaytoPaymentSchedule {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MandatePaytoPaymentSchedule> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MandatePaytoPaymentSchedule::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MandatePaytoPaymentSchedule);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MandatePaytoPaymentSchedule {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
