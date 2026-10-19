/// The billing mode of the subscription.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourceBillingMode {
    /// Configure behavior for flexible billing mode
    pub flexible: Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>,
    /// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreateSubscriptionBillingModeType,
    /// Details on when the current billing_mode was adopted.
    pub updated_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionsResourceBillingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionsResourceBillingMode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionsResourceBillingModeBuilder {
    flexible: Option<Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>>,
    type_: Option<stripe_shared::CreateSubscriptionBillingModeType>,
    updated_at: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for SubscriptionsResourceBillingMode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourceBillingMode>,
        builder: SubscriptionsResourceBillingModeBuilder,
    }

    impl Visitor for Place<SubscriptionsResourceBillingMode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourceBillingModeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourceBillingModeBuilder {
        type Out = SubscriptionsResourceBillingMode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "flexible" => Deserialize::begin(&mut self.flexible),
                "type" => Deserialize::begin(&mut self.type_),
                "updated_at" => Deserialize::begin(&mut self.updated_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { flexible: Some(None), type_: None, updated_at: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(flexible), Some(type_), Some(updated_at)) =
                (self.flexible.take(), self.type_.take(), self.updated_at)
            else {
                return None;
            };
            Some(Self::Out { flexible, type_, updated_at })
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

    impl ObjectDeser for SubscriptionsResourceBillingMode {
        type Builder = SubscriptionsResourceBillingModeBuilder;
    }

    impl FromValueOpt for SubscriptionsResourceBillingMode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourceBillingModeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "flexible" => b.flexible = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "updated_at" => b.updated_at = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
pub type SubscriptionsResourceBillingModeType = stripe_shared::CreateSubscriptionBillingModeType;
