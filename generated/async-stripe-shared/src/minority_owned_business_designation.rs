/// Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum MinorityOwnedBusinessDesignation {
    LgbtqiOwnedBusiness,
    MinorityOwnedBusiness,
    NoneOfTheseApply,
    PreferNotToAnswer,
    WomenOwnedBusiness,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl MinorityOwnedBusinessDesignation {
    pub fn as_str(&self) -> &str {
        use MinorityOwnedBusinessDesignation::*;
        match self {
            LgbtqiOwnedBusiness => "lgbtqi_owned_business",
            MinorityOwnedBusiness => "minority_owned_business",
            NoneOfTheseApply => "none_of_these_apply",
            PreferNotToAnswer => "prefer_not_to_answer",
            WomenOwnedBusiness => "women_owned_business",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for MinorityOwnedBusinessDesignation {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use MinorityOwnedBusinessDesignation::*;
        match s {
            "lgbtqi_owned_business" => Ok(LgbtqiOwnedBusiness),
            "minority_owned_business" => Ok(MinorityOwnedBusiness),
            "none_of_these_apply" => Ok(NoneOfTheseApply),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "women_owned_business" => Ok(WomenOwnedBusiness),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "MinorityOwnedBusinessDesignation"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for MinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for MinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(MinorityOwnedBusinessDesignation)).finish_non_exhaustive()
    }
}
impl serde::Serialize for MinorityOwnedBusinessDesignation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for MinorityOwnedBusinessDesignation {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<MinorityOwnedBusinessDesignation> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(MinorityOwnedBusinessDesignation::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(MinorityOwnedBusinessDesignation);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for MinorityOwnedBusinessDesignation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
