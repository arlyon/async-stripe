#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsFlow {
    pub after_completion: stripe_billing::PortalFlowsFlowAfterCompletion,
    /// Configuration when `flow.type=subscription_cancel`.
    pub subscription_cancel: Option<stripe_billing::PortalFlowsFlowSubscriptionCancel>,
    /// Configuration when `flow.type=subscription_update`.
    pub subscription_update: Option<stripe_billing::PortalFlowsFlowSubscriptionUpdate>,
    /// Configuration when `flow.type=subscription_update_confirm`.
    pub subscription_update_confirm:
        Option<stripe_billing::PortalFlowsFlowSubscriptionUpdateConfirm>,
    /// Type of flow that the customer will go through.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PortalFlowsFlowType,
}
#[doc(hidden)]
pub struct PortalFlowsFlowBuilder {
    after_completion: Option<stripe_billing::PortalFlowsFlowAfterCompletion>,
    subscription_cancel: Option<Option<stripe_billing::PortalFlowsFlowSubscriptionCancel>>,
    subscription_update: Option<Option<stripe_billing::PortalFlowsFlowSubscriptionUpdate>>,
    subscription_update_confirm:
        Option<Option<stripe_billing::PortalFlowsFlowSubscriptionUpdateConfirm>>,
    type_: Option<PortalFlowsFlowType>,
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

    impl Deserialize for PortalFlowsFlow {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsFlow>,
        builder: PortalFlowsFlowBuilder,
    }

    impl Visitor for Place<PortalFlowsFlow> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsFlowBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsFlowBuilder {
        type Out = PortalFlowsFlow;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "after_completion" => Deserialize::begin(&mut self.after_completion),
                "subscription_cancel" => Deserialize::begin(&mut self.subscription_cancel),
                "subscription_update" => Deserialize::begin(&mut self.subscription_update),
                "subscription_update_confirm" => {
                    Deserialize::begin(&mut self.subscription_update_confirm)
                }
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                after_completion: Deserialize::default(),
                subscription_cancel: Deserialize::default(),
                subscription_update: Deserialize::default(),
                subscription_update_confirm: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(after_completion),
                Some(subscription_cancel),
                Some(subscription_update),
                Some(subscription_update_confirm),
                Some(type_),
            ) = (
                self.after_completion.take(),
                self.subscription_cancel.take(),
                self.subscription_update.take(),
                self.subscription_update_confirm.take(),
                self.type_.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                after_completion,
                subscription_cancel,
                subscription_update,
                subscription_update_confirm,
                type_,
            })
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

    impl ObjectDeser for PortalFlowsFlow {
        type Builder = PortalFlowsFlowBuilder;
    }

    impl FromValueOpt for PortalFlowsFlow {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsFlowBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "after_completion" => b.after_completion = FromValueOpt::from_value(v),
                    "subscription_cancel" => b.subscription_cancel = FromValueOpt::from_value(v),
                    "subscription_update" => b.subscription_update = FromValueOpt::from_value(v),
                    "subscription_update_confirm" => {
                        b.subscription_update_confirm = FromValueOpt::from_value(v)
                    }
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of flow that the customer will go through.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PortalFlowsFlowType {
    PaymentMethodUpdate,
    SubscriptionCancel,
    SubscriptionUpdate,
    SubscriptionUpdateConfirm,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PortalFlowsFlowType {
    pub fn as_str(&self) -> &str {
        use PortalFlowsFlowType::*;
        match self {
            PaymentMethodUpdate => "payment_method_update",
            SubscriptionCancel => "subscription_cancel",
            SubscriptionUpdate => "subscription_update",
            SubscriptionUpdateConfirm => "subscription_update_confirm",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PortalFlowsFlowType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PortalFlowsFlowType::*;
        match s {
            "payment_method_update" => Ok(PaymentMethodUpdate),
            "subscription_cancel" => Ok(SubscriptionCancel),
            "subscription_update" => Ok(SubscriptionUpdate),
            "subscription_update_confirm" => Ok(SubscriptionUpdateConfirm),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PortalFlowsFlowType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PortalFlowsFlowType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PortalFlowsFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PortalFlowsFlowType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalFlowsFlowType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PortalFlowsFlowType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalFlowsFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
