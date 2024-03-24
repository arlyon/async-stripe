#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLinksResourceSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    pub metadata: std::collections::HashMap<String, String>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentLinksResourceSubscriptionDataBuilder {
    description: Option<Option<String>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    trial_period_days: Option<Option<u32>>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLinksResourceSubscriptionData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourceSubscriptionData>,
        builder: PaymentLinksResourceSubscriptionDataBuilder,
    }

    impl Visitor for Place<PaymentLinksResourceSubscriptionData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentLinksResourceSubscriptionDataBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentLinksResourceSubscriptionDataBuilder {
        type Out = PaymentLinksResourceSubscriptionData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "description" => Deserialize::begin(&mut self.description),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "trial_period_days" => Deserialize::begin(&mut self.trial_period_days),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { description: Deserialize::default(), metadata: Deserialize::default(), trial_period_days: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let description = self.description.take()?;
            let metadata = self.metadata.take()?;
            let trial_period_days = self.trial_period_days.take()?;

            Some(Self::Out { description, metadata, trial_period_days })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentLinksResourceSubscriptionData {
        type Builder = PaymentLinksResourceSubscriptionDataBuilder;
    }

    impl FromValueOpt for PaymentLinksResourceSubscriptionData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourceSubscriptionDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "trial_period_days" => b.trial_period_days = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
