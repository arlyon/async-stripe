#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceSubscriptionDataSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    /// This date is ignored if it is in the past when the quote is accepted.
    /// Measured in seconds since the Unix epoch.
    pub effective_date: Option<stripe_types::Timestamp>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on the subscription or subscription schedule when the quote is accepted.
    /// If a recurring price is included in `line_items`, this field will be passed to the resulting subscription's `metadata` field.
    /// If `subscription_data.effective_date` is used, this field will be passed to the resulting subscription schedule's `phases.metadata` field.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,
}
#[doc(hidden)]
pub struct QuotesResourceSubscriptionDataSubscriptionDataBuilder {
    description: Option<Option<String>>,
    effective_date: Option<Option<stripe_types::Timestamp>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    trial_period_days: Option<Option<u32>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for QuotesResourceSubscriptionDataSubscriptionData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceSubscriptionDataSubscriptionData>,
        builder: QuotesResourceSubscriptionDataSubscriptionDataBuilder,
    }

    impl Visitor for Place<QuotesResourceSubscriptionDataSubscriptionData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceSubscriptionDataSubscriptionDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for QuotesResourceSubscriptionDataSubscriptionDataBuilder {
        type Out = QuotesResourceSubscriptionDataSubscriptionData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "description" => Deserialize::begin(&mut self.description),
                "effective_date" => Deserialize::begin(&mut self.effective_date),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "trial_period_days" => Deserialize::begin(&mut self.trial_period_days),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                description: Deserialize::default(),
                effective_date: Deserialize::default(),
                metadata: Deserialize::default(),
                trial_period_days: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                description: self.description.take()?,
                effective_date: self.effective_date?,
                metadata: self.metadata.take()?,
                trial_period_days: self.trial_period_days?,
            })
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

    impl ObjectDeser for QuotesResourceSubscriptionDataSubscriptionData {
        type Builder = QuotesResourceSubscriptionDataSubscriptionDataBuilder;
    }

    impl FromValueOpt for QuotesResourceSubscriptionDataSubscriptionData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = QuotesResourceSubscriptionDataSubscriptionDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "effective_date" => b.effective_date = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "trial_period_days" => b.trial_period_days = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
