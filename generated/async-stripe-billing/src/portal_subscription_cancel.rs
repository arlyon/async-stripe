#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalSubscriptionCancel {
    pub cancellation_reason: stripe_billing::PortalSubscriptionCancellationReason,
    /// Whether the feature is enabled.
    pub enabled: bool,
    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    pub mode: PortalSubscriptionCancelMode,
    /// Whether to create prorations when canceling subscriptions.
    /// Possible values are `none` and `create_prorations`.
    pub proration_behavior: PortalSubscriptionCancelProrationBehavior,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalSubscriptionCancel").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PortalSubscriptionCancelBuilder {
    cancellation_reason: Option<stripe_billing::PortalSubscriptionCancellationReason>,
    enabled: Option<bool>,
    mode: Option<PortalSubscriptionCancelMode>,
    proration_behavior: Option<PortalSubscriptionCancelProrationBehavior>,
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

    impl Deserialize for PortalSubscriptionCancel {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalSubscriptionCancel>,
        builder: PortalSubscriptionCancelBuilder,
    }

    impl Visitor for Place<PortalSubscriptionCancel> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalSubscriptionCancelBuilder {
                    cancellation_reason: Deserialize::default(),
                    enabled: Deserialize::default(),
                    mode: Deserialize::default(),
                    proration_behavior: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "cancellation_reason" => Deserialize::begin(&mut self.builder.cancellation_reason),
                "enabled" => Deserialize::begin(&mut self.builder.enabled),
                "mode" => Deserialize::begin(&mut self.builder.mode),
                "proration_behavior" => Deserialize::begin(&mut self.builder.proration_behavior),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(cancellation_reason), Some(enabled), Some(mode), Some(proration_behavior)) = (
                self.builder.cancellation_reason.take(),
                self.builder.enabled,
                self.builder.mode.take(),
                self.builder.proration_behavior.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(PortalSubscriptionCancel {
                cancellation_reason,
                enabled,
                mode,
                proration_behavior,
            });
            Ok(())
        }
    }
};
/// Whether to cancel subscriptions immediately or at the end of the billing period.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalSubscriptionCancelMode {
    pub fn as_str(&self) -> &str {
        use PortalSubscriptionCancelMode::*;
        match self {
            AtPeriodEnd => "at_period_end",
            Immediately => "immediately",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalSubscriptionCancelMode {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionCancelMode::*;
        match s {
            "at_period_end" => Ok(AtPeriodEnd),
            "immediately" => Ok(Immediately),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalSubscriptionCancelMode"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalSubscriptionCancelMode)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalSubscriptionCancelMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PortalSubscriptionCancelMode {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PortalSubscriptionCancelMode> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalSubscriptionCancelMode::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalSubscriptionCancelMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Whether to create prorations when canceling subscriptions.
/// Possible values are `none` and `create_prorations`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalSubscriptionCancelProrationBehavior {
    pub fn as_str(&self) -> &str {
        use PortalSubscriptionCancelProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalSubscriptionCancelProrationBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalSubscriptionCancelProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PortalSubscriptionCancelProrationBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalSubscriptionCancelProrationBehavior))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalSubscriptionCancelProrationBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PortalSubscriptionCancelProrationBehavior {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PortalSubscriptionCancelProrationBehavior> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PortalSubscriptionCancelProrationBehavior::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalSubscriptionCancelProrationBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
