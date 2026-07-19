#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourceSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,
    pub invoice_settings: stripe_shared::PaymentLinksResourceSubscriptionDataInvoiceSettings,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will set metadata on [Subscriptions](https://docs.stripe.com/api/subscriptions) generated from this payment link.
    pub metadata: std::collections::HashMap<String, String>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,
    /// Settings related to subscription trials.
    pub trial_settings: Option<stripe_shared::SubscriptionsTrialsResourceTrialSettings>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourceSubscriptionData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourceSubscriptionData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourceSubscriptionDataBuilder {
    description: Option<Option<String>>,
    invoice_settings: Option<stripe_shared::PaymentLinksResourceSubscriptionDataInvoiceSettings>,
    metadata: Option<std::collections::HashMap<String, String>>,
    trial_period_days: Option<Option<u32>>,
    trial_settings: Option<Option<stripe_shared::SubscriptionsTrialsResourceTrialSettings>>,
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
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourceSubscriptionDataBuilder {
                    description: Deserialize::default(),
                    invoice_settings: Deserialize::default(),
                    metadata: Deserialize::default(),
                    trial_period_days: Deserialize::default(),
                    trial_settings: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "description" => Deserialize::begin(&mut self.builder.description),
                "invoice_settings" => Deserialize::begin(&mut self.builder.invoice_settings),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "trial_period_days" => Deserialize::begin(&mut self.builder.trial_period_days),
                "trial_settings" => Deserialize::begin(&mut self.builder.trial_settings),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(description),
                Some(invoice_settings),
                Some(metadata),
                Some(trial_period_days),
                Some(trial_settings),
            ) = (
                self.builder.description.take(),
                self.builder.invoice_settings.take(),
                self.builder.metadata.take(),
                self.builder.trial_period_days,
                self.builder.trial_settings.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourceSubscriptionData {
                description,
                invoice_settings,
                metadata,
                trial_period_days,
                trial_settings,
            });
            Ok(())
        }
    }
};
