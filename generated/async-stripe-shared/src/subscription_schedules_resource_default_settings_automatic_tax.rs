#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
    /// Whether Stripe automatically computes tax on invoices created during this phase.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<stripe_shared::ConnectAccountReference>,
}
#[doc(hidden)]
pub struct SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder {
    enabled: Option<bool>,
    liability: Option<Option<stripe_shared::ConnectAccountReference>>,
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

    impl Deserialize for SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionSchedulesResourceDefaultSettingsAutomaticTax>,
        builder: SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder,
    }

    impl Visitor for Place<SubscriptionSchedulesResourceDefaultSettingsAutomaticTax> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder {
        type Out = SubscriptionSchedulesResourceDefaultSettingsAutomaticTax;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "enabled" => Deserialize::begin(&mut self.enabled),
                "liability" => Deserialize::begin(&mut self.liability),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { enabled: Deserialize::default(), liability: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(enabled), Some(liability)) = (self.enabled, self.liability.take()) else {
                return None;
            };
            Some(Self::Out { enabled, liability })
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

    impl ObjectDeser for SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
        type Builder = SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder;
    }

    impl FromValueOpt for SubscriptionSchedulesResourceDefaultSettingsAutomaticTax {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                SubscriptionSchedulesResourceDefaultSettingsAutomaticTaxBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "enabled" => b.enabled = FromValueOpt::from_value(v),
                    "liability" => b.liability = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
