/// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
/// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum OwnershipExemptionReason {
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl OwnershipExemptionReason {
    pub fn as_str(&self) -> &str {
        use OwnershipExemptionReason::*;
        match self {
            QualifiedEntityExceedsOwnershipThreshold => {
                "qualified_entity_exceeds_ownership_threshold"
            }
            QualifiesAsFinancialInstitution => "qualifies_as_financial_institution",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for OwnershipExemptionReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use OwnershipExemptionReason::*;
        match s {
            "qualified_entity_exceeds_ownership_threshold" => {
                Ok(QualifiedEntityExceedsOwnershipThreshold)
            }
            "qualifies_as_financial_institution" => Ok(QualifiesAsFinancialInstitution),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "OwnershipExemptionReason");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for OwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for OwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for OwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(OwnershipExemptionReason)).finish_non_exhaustive()
    }
}
impl serde::Serialize for OwnershipExemptionReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for OwnershipExemptionReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<OwnershipExemptionReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(OwnershipExemptionReason::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(OwnershipExemptionReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for OwnershipExemptionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
