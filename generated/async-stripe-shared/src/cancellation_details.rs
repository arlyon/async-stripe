#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CancellationDetails {
    /// Additional comments about why the user canceled the subscription, if the subscription was canceled explicitly by the user.
    pub comment: Option<String>,
    /// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
    pub feedback: Option<CancellationDetailsFeedback>,
    /// Why this subscription was canceled.
    pub reason: Option<CancellationDetailsReason>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancellationDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CancellationDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CancellationDetailsBuilder {
    comment: Option<Option<String>>,
    feedback: Option<Option<CancellationDetailsFeedback>>,
    reason: Option<Option<CancellationDetailsReason>>,
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

    impl Deserialize for CancellationDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CancellationDetails>,
        builder: CancellationDetailsBuilder,
    }

    impl Visitor for Place<CancellationDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CancellationDetailsBuilder {
                    comment: Deserialize::default(),
                    feedback: Deserialize::default(),
                    reason: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "comment" => Deserialize::begin(&mut self.builder.comment),
                "feedback" => Deserialize::begin(&mut self.builder.feedback),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(comment), Some(feedback), Some(reason)) = (
                self.builder.comment.take(),
                self.builder.feedback.take(),
                self.builder.reason.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(CancellationDetails { comment, feedback, reason });
            Ok(())
        }
    }
};
/// The customer submitted reason for why they canceled, if the subscription was canceled explicitly by the user.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CancellationDetailsFeedback {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CancellationDetailsFeedback {
    pub fn as_str(&self) -> &str {
        use CancellationDetailsFeedback::*;
        match self {
            CustomerService => "customer_service",
            LowQuality => "low_quality",
            MissingFeatures => "missing_features",
            Other => "other",
            SwitchedService => "switched_service",
            TooComplex => "too_complex",
            TooExpensive => "too_expensive",
            Unused => "unused",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CancellationDetailsFeedback {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CancellationDetailsFeedback::*;
        match s {
            "customer_service" => Ok(CustomerService),
            "low_quality" => Ok(LowQuality),
            "missing_features" => Ok(MissingFeatures),
            "other" => Ok(Other),
            "switched_service" => Ok(SwitchedService),
            "too_complex" => Ok(TooComplex),
            "too_expensive" => Ok(TooExpensive),
            "unused" => Ok(Unused),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CancellationDetailsFeedback"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancellationDetailsFeedback {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CancellationDetailsFeedback)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CancellationDetailsFeedback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CancellationDetailsFeedback {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CancellationDetailsFeedback> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CancellationDetailsFeedback::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CancellationDetailsFeedback {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Why this subscription was canceled.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CancellationDetailsReason {
    CanceledByRetentionPolicy,
    CancellationRequested,
    PaymentDisputed,
    PaymentFailed,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CancellationDetailsReason {
    pub fn as_str(&self) -> &str {
        use CancellationDetailsReason::*;
        match self {
            CanceledByRetentionPolicy => "canceled_by_retention_policy",
            CancellationRequested => "cancellation_requested",
            PaymentDisputed => "payment_disputed",
            PaymentFailed => "payment_failed",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CancellationDetailsReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CancellationDetailsReason::*;
        match s {
            "canceled_by_retention_policy" => Ok(CanceledByRetentionPolicy),
            "cancellation_requested" => Ok(CancellationRequested),
            "payment_disputed" => Ok(PaymentDisputed),
            "payment_failed" => Ok(PaymentFailed),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CancellationDetailsReason");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CancellationDetailsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CancellationDetailsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CancellationDetailsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CancellationDetailsReason)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CancellationDetailsReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for CancellationDetailsReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CancellationDetailsReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CancellationDetailsReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CancellationDetailsReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
