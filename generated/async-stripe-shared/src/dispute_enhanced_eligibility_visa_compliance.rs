#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct DisputeEnhancedEligibilityVisaCompliance {
    /// Visa compliance eligibility status.
    pub status: DisputeEnhancedEligibilityVisaComplianceStatus,
}
#[doc(hidden)]
pub struct DisputeEnhancedEligibilityVisaComplianceBuilder {
    status: Option<DisputeEnhancedEligibilityVisaComplianceStatus>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: DisputeEnhancedEligibilityVisaComplianceBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for DisputeEnhancedEligibilityVisaComplianceBuilder {
        type Out = DisputeEnhancedEligibilityVisaCompliance;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "status" => Deserialize::begin(&mut self.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { status: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(status),) = (self.status.take(),) else {
                return None;
            };
            Some(Self::Out { status })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for DisputeEnhancedEligibilityVisaCompliance {
        type Builder = DisputeEnhancedEligibilityVisaComplianceBuilder;
    }

    impl FromValueOpt for DisputeEnhancedEligibilityVisaCompliance {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = DisputeEnhancedEligibilityVisaComplianceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "status" => b.status = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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

impl std::fmt::Debug for DisputeEnhancedEligibilityVisaComplianceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
