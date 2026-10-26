/// Defines how a subscription behaves when a trial ends.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceTrialSettingsEndBehavior {
    /// Indicates how the subscription's billing cycle anchor is reset when a trial ends.
    /// If not set, the default is `now`.
    pub billing_cycle_anchor:
        Option<SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor>,
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method: SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceTrialSettingsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceTrialSettingsEndBehavior").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceTrialSettingsEndBehaviorBuilder {
    billing_cycle_anchor:
        Option<Option<SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor>>,
    missing_payment_method:
        Option<SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod>,
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

    impl Deserialize for SubscriptionsResourceTrialSettingsEndBehavior {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceTrialSettingsEndBehavior>,
        builder: SubscriptionsResourceTrialSettingsEndBehaviorBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceTrialSettingsEndBehavior> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceTrialSettingsEndBehaviorBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceTrialSettingsEndBehaviorBuilder {
        type Out = SubscriptionsResourceTrialSettingsEndBehavior;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_cycle_anchor" => Deserialize::begin(&mut self.billing_cycle_anchor),
                "missing_payment_method" => Deserialize::begin(&mut self.missing_payment_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { billing_cycle_anchor: Some(None), missing_payment_method: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(billing_cycle_anchor), Some(missing_payment_method)) =
                (self.billing_cycle_anchor.take(), self.missing_payment_method.take())
            else {
                return None;
            };
            Some(Self::Out { billing_cycle_anchor, missing_payment_method })
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

    impl ObjectDeser for SubscriptionsResourceTrialSettingsEndBehavior {
        type Builder = SubscriptionsResourceTrialSettingsEndBehaviorBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceTrialSettingsEndBehavior {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourceTrialSettingsEndBehaviorBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_cycle_anchor" => b.billing_cycle_anchor = FromValueOpt::from_value(v),
                    "missing_payment_method" => {
                        b.missing_payment_method = FromValueOpt::from_value(v)
                    }
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates how the subscription's billing cycle anchor is reset when a trial ends.
/// If not set, the default is `now`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    Now,
    Unchanged,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    pub fn as_str(&self) -> &str {
        use SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::*;
        match self {
            Now => "now",
            Unchanged => "unchanged",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::*;
        match s {
            "now" => Ok(Now),
            "unchanged" => Ok(Unchanged),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionsResourceTrialSettingsEndBehaviorBillingCycleAnchor
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(&self) -> &str {
        use SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match self {
            Cancel => "cancel",
            CreateInvoice => "create_invoice",
            Pause => "pause",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::*;
        match s {
            "cancel" => Ok(Cancel),
            "create_invoice" => Ok(CreateInvoice),
            "pause" => Ok(Pause),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(
            SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod
        ))
        .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for SubscriptionsResourceTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
