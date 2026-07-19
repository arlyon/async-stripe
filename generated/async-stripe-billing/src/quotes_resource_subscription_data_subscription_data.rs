#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceSubscriptionDataSubscriptionData {
    pub billing_mode: stripe_billing::QuotesResourceSubscriptionDataBillingMode,
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    /// This date is ignored if it is in the past when the quote is accepted.
    /// Measured in seconds since the Unix epoch.
    pub effective_date: Option<stripe_types::Timestamp>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will set metadata on the subscription or subscription schedule when the quote is accepted.
    /// If a recurring price is included in `line_items`, this field will be passed to the resulting subscription's `metadata` field.
    /// If `subscription_data.effective_date` is used, this field will be passed to the resulting subscription schedule's `phases.metadata` field.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceSubscriptionDataSubscriptionData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceSubscriptionDataSubscriptionData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct QuotesResourceSubscriptionDataSubscriptionDataBuilder {
    billing_mode: Option<stripe_billing::QuotesResourceSubscriptionDataBillingMode>,
    description: Option<Option<String>>,
    effective_date: Option<Option<stripe_types::Timestamp>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    trial_period_days: Option<Option<u32>>,
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
                builder: QuotesResourceSubscriptionDataSubscriptionDataBuilder {
                    billing_mode: Deserialize::default(),
                    description: Deserialize::default(),
                    effective_date: Deserialize::default(),
                    metadata: Deserialize::default(),
                    trial_period_days: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_mode" => Deserialize::begin(&mut self.builder.billing_mode),
                "description" => Deserialize::begin(&mut self.builder.description),
                "effective_date" => Deserialize::begin(&mut self.builder.effective_date),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "trial_period_days" => Deserialize::begin(&mut self.builder.trial_period_days),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(billing_mode),
                Some(description),
                Some(effective_date),
                Some(metadata),
                Some(trial_period_days),
            ) = (
                self.builder.billing_mode.take(),
                self.builder.description.take(),
                self.builder.effective_date,
                self.builder.metadata.take(),
                self.builder.trial_period_days,
            )
            else {
                return Ok(());
            };
            *self.out = Some(QuotesResourceSubscriptionDataSubscriptionData {
                billing_mode,
                description,
                effective_date,
                metadata,
                trial_period_days,
            });
            Ok(())
        }
    }
};
