#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourcePauseScheduleResumeSettings {
    /// The billing cycle anchor that applies when the subscription is resumed.
    pub billing_cycle_anchor:
        SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor,
    /// Controls whether Stripe attempts payment on the resumption invoice and how that affects the subscription's status.
    pub payment_behavior: SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior,
    /// Determines how to handle prorations resulting from the billing_cycle_anchor change on resume.
    pub proration_behavior:
        SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleResumeSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionSchedulesResourcePauseScheduleResumeSettings")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourcePauseScheduleResumeSettingsBuilder {
    billing_cycle_anchor:
        Option<SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor>,
    payment_behavior:
        Option<SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior>,
    proration_behavior:
        Option<SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior>,
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

    impl Deserialize for SubscriptionSchedulesResourcePauseScheduleResumeSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourcePauseScheduleResumeSettings>,
        builder: SubscriptionSchedulesResourcePauseScheduleResumeSettingsBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourcePauseScheduleResumeSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    SubscriptionSchedulesResourcePauseScheduleResumeSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourcePauseScheduleResumeSettingsBuilder {
        type Out = SubscriptionSchedulesResourcePauseScheduleResumeSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_cycle_anchor" => Deserialize::begin(&mut self.billing_cycle_anchor),
                "payment_behavior" => Deserialize::begin(&mut self.payment_behavior),
                "proration_behavior" => Deserialize::begin(&mut self.proration_behavior),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { billing_cycle_anchor: None, payment_behavior: None, proration_behavior: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(billing_cycle_anchor), Some(payment_behavior), Some(proration_behavior)) = (
                self.billing_cycle_anchor.take(),
                self.payment_behavior.take(),
                self.proration_behavior.take(),
            ) else {
                return None;
            };
            Some(Self::Out { billing_cycle_anchor, payment_behavior, proration_behavior })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for SubscriptionSchedulesResourcePauseScheduleResumeSettings {
        type Builder = SubscriptionSchedulesResourcePauseScheduleResumeSettingsBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourcePauseScheduleResumeSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                SubscriptionSchedulesResourcePauseScheduleResumeSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_cycle_anchor" => b.billing_cycle_anchor = FromValueOpt::from_value(v),
                    "payment_behavior" => b.payment_behavior = FromValueOpt::from_value(v),
                    "proration_behavior" => b.proration_behavior = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The billing cycle anchor that applies when the subscription is resumed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor {
    ResumeAt,
    Unchanged,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor::*;
        match self {
            ResumeAt => "resume_at",
            Unchanged => "unchanged",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor::*;
        match s {
            "resume_at" => Ok(ResumeAt),
            "unchanged" => Ok(Unchanged),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsBillingCycleAnchor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Controls whether Stripe attempts payment on the resumption invoice and how that affects the subscription's status.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    ResumeOnPaymentAttempt,
    ResumeOnPaymentSuccess,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::*;
        match self {
            ResumeOnPaymentAttempt => "resume_on_payment_attempt",
            ResumeOnPaymentSuccess => "resume_on_payment_success",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::*;
        match s {
            "resume_on_payment_attempt" => Ok(ResumeOnPaymentAttempt),
            "resume_on_payment_success" => Ok(ResumeOnPaymentSuccess),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsPaymentBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Determines how to handle prorations resulting from the billing_cycle_anchor change on resume.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior {
    pub fn as_str(&self) -> &str {
        use SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior::*;
        match self {
            AlwaysInvoice => "always_invoice",
            CreateProrations => "create_prorations",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior
{
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior::*;
        match s {
            "always_invoice" => Ok(AlwaysInvoice),
            "create_prorations" => Ok(CreateProrations),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionSchedulesResourcePauseScheduleResumeSettingsProrationBehavior
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
