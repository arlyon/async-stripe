#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFlowsFlow").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PortalFlowsFlowBuilder {
                    after_completion: Deserialize::default(),
                    subscription_cancel: Deserialize::default(),
                    subscription_update: Deserialize::default(),
                    subscription_update_confirm: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "after_completion" => Deserialize::begin(&mut self.builder.after_completion),
                "subscription_cancel" => Deserialize::begin(&mut self.builder.subscription_cancel),
                "subscription_update" => Deserialize::begin(&mut self.builder.subscription_update),
                "subscription_update_confirm" => {
                    Deserialize::begin(&mut self.builder.subscription_update_confirm)
                }
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(after_completion),
                Some(subscription_cancel),
                Some(subscription_update),
                Some(subscription_update_confirm),
                Some(type_),
            ) = (
                self.builder.after_completion.take(),
                self.builder.subscription_cancel.take(),
                self.builder.subscription_update.take(),
                self.builder.subscription_update_confirm.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PortalFlowsFlow {
                after_completion,
                subscription_cancel,
                subscription_update,
                subscription_update_confirm,
                type_,
            });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PortalFlowsFlowType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for PortalFlowsFlowType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PortalFlowsFlowType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PortalFlowsFlowType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PortalFlowsFlowType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
