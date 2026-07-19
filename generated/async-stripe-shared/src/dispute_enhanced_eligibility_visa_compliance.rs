#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEligibilityVisaCompliance {
    /// Visa compliance eligibility status.
    pub status: DisputeEnhancedEligibilityVisaComplianceStatus,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DisputeEnhancedEligibilityVisaCompliance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DisputeEnhancedEligibilityVisaCompliance").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct DisputeEnhancedEligibilityVisaComplianceBuilder {
    status: Option<DisputeEnhancedEligibilityVisaComplianceStatus>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for DisputeEnhancedEligibilityVisaCompliance {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<DisputeEnhancedEligibilityVisaCompliance>,
        builder: DisputeEnhancedEligibilityVisaComplianceBuilder,
    }

    impl Visitor for Place<DisputeEnhancedEligibilityVisaCompliance> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: DisputeEnhancedEligibilityVisaComplianceBuilder {
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(status),) = (self.builder.status.take(),) else {
                return Ok(());
            };
            *self.out = Some(DisputeEnhancedEligibilityVisaCompliance { status });
            Ok(())
        }
    }
};
/// Visa compliance eligibility status.
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
impl stripe_miniserde::Deserialize for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<DisputeEnhancedEligibilityVisaComplianceStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(DisputeEnhancedEligibilityVisaComplianceStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
