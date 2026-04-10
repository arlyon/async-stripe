/// Defines how a subscription behaves when a trial ends.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceTrialSettingsEndBehavior {
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
                "missing_payment_method" => Deserialize::begin(&mut self.missing_payment_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { missing_payment_method: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(missing_payment_method),) = (self.missing_payment_method.take(),) else {
                return None;
            };
            Some(Self::Out { missing_payment_method })
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
