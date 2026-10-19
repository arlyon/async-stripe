#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingPersonalizationDesignStatus {
    Active,
    Inactive,
    Rejected,
    Review,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingPersonalizationDesignStatus {
    pub fn as_str(&self) -> &str {
        use IssuingPersonalizationDesignStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Rejected => "rejected",
            Review => "review",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingPersonalizationDesignStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPersonalizationDesignStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "rejected" => Ok(Rejected),
            "review" => Ok(Review),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingPersonalizationDesignStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingPersonalizationDesignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingPersonalizationDesignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPersonalizationDesignStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingPersonalizationDesignStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for IssuingPersonalizationDesignStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for IssuingPersonalizationDesignStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<IssuingPersonalizationDesignStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingPersonalizationDesignStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(IssuingPersonalizationDesignStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPersonalizationDesignStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
