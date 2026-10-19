/// The billing mode of the quote.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceSubscriptionDataBillingMode {
    pub flexible: Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>,
    /// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: stripe_shared::CreateSubscriptionBillingModeType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceSubscriptionDataBillingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceSubscriptionDataBillingMode").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct QuotesResourceSubscriptionDataBillingModeBuilder {
    flexible: Option<Option<stripe_shared::SubscriptionsResourceBillingModeFlexible>>,
    type_: Option<stripe_shared::CreateSubscriptionBillingModeType>,
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

    impl Deserialize for QuotesResourceSubscriptionDataBillingMode {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceSubscriptionDataBillingMode>,
        builder: QuotesResourceSubscriptionDataBillingModeBuilder,
    }

    impl Visitor for Place<QuotesResourceSubscriptionDataBillingMode> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceSubscriptionDataBillingModeBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for QuotesResourceSubscriptionDataBillingModeBuilder {
        type Out = QuotesResourceSubscriptionDataBillingMode;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "flexible" => Deserialize::begin(&mut self.flexible),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { flexible: Some(None), type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(flexible), Some(type_)) = (self.flexible.take(), self.type_.take()) else {
                return None;
            };
            Some(Self::Out { flexible, type_ })
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

    impl ObjectDeser for QuotesResourceSubscriptionDataBillingMode {
        type Builder = QuotesResourceSubscriptionDataBillingModeBuilder;
    }

    impl FromValueOpt for QuotesResourceSubscriptionDataBillingMode {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = QuotesResourceSubscriptionDataBillingModeBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "flexible" => b.flexible = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls how prorations and invoices for subscriptions are calculated and orchestrated.
pub type QuotesResourceSubscriptionDataBillingModeType =
    stripe_shared::CreateSubscriptionBillingModeType;
