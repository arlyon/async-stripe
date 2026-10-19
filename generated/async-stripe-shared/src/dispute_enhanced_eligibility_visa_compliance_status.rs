#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum DisputeEnhancedEligibilityVisaComplianceStatus {
    FeeAcknowledged,
    RequiresFeeAcknowledgement,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl DisputeEnhancedEligibilityVisaComplianceStatus {
    pub fn as_str(&self) -> &str {
        use DisputeEnhancedEligibilityVisaComplianceStatus::*;
        match self {
            FeeAcknowledged => "fee_acknowledged",
            RequiresFeeAcknowledgement => "requires_fee_acknowledgement",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for DisputeEnhancedEligibilityVisaComplianceStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use DisputeEnhancedEligibilityVisaComplianceStatus::*;
        match s {
            "fee_acknowledged" => Ok(FeeAcknowledged),
            "requires_fee_acknowledgement" => Ok(RequiresFeeAcknowledgement),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "DisputeEnhancedEligibilityVisaComplianceStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(DisputeEnhancedEligibilityVisaComplianceStatus))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<DisputeEnhancedEligibilityVisaComplianceStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(DisputeEnhancedEligibilityVisaComplianceStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(DisputeEnhancedEligibilityVisaComplianceStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
