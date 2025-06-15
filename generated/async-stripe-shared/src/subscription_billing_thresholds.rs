#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionBillingThresholds {
    /// Monetary threshold that triggers the subscription to create an invoice
    pub amount_gte: Option<i64>,
    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    /// This value may not be `true` if the subscription contains items with plans that have `aggregate_usage=last_ever`.
    pub reset_billing_cycle_anchor: Option<bool>,
}
#[doc(hidden)]
pub struct SubscriptionBillingThresholdsBuilder {
    amount_gte: Option<Option<i64>>,
    reset_billing_cycle_anchor: Option<Option<bool>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SubscriptionBillingThresholds {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionBillingThresholds>,
        builder: SubscriptionBillingThresholdsBuilder,
    }

    impl Visitor for Place<SubscriptionBillingThresholds> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionBillingThresholdsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionBillingThresholdsBuilder {
        type Out = SubscriptionBillingThresholds;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_gte" => Deserialize::begin(&mut self.amount_gte),
                "reset_billing_cycle_anchor" => {
                    Deserialize::begin(&mut self.reset_billing_cycle_anchor)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_gte: Deserialize::default(),
                reset_billing_cycle_anchor: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_gte), Some(reset_billing_cycle_anchor)) =
                (self.amount_gte, self.reset_billing_cycle_anchor)
            else {
                return None;
            };
            Some(Self::Out { amount_gte, reset_billing_cycle_anchor })
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

    impl ObjectDeser for SubscriptionBillingThresholds {
        type Builder = SubscriptionBillingThresholdsBuilder;
    }

    impl FromValueOpt for SubscriptionBillingThresholds {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionBillingThresholdsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_gte" => b.amount_gte = FromValueOpt::from_value(v),
                    "reset_billing_cycle_anchor" => {
                        b.reset_billing_cycle_anchor = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
