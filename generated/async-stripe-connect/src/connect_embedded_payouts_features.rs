#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConnectEmbeddedPayoutsFeatures {
    /// Whether to allow payout schedule to be changed.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub edit_payout_schedule: bool,
    /// Whether to allow creation of instant payouts.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub instant_payouts: bool,
    /// Whether to allow creation of standard payouts.
    /// Default `true` when Stripe owns Loss Liability, default `false` otherwise.
    pub standard_payouts: bool,
}
#[doc(hidden)]
pub struct ConnectEmbeddedPayoutsFeaturesBuilder {
    edit_payout_schedule: Option<bool>,
    instant_payouts: Option<bool>,
    standard_payouts: Option<bool>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ConnectEmbeddedPayoutsFeatures {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConnectEmbeddedPayoutsFeatures>,
        builder: ConnectEmbeddedPayoutsFeaturesBuilder,
    }

    impl Visitor for Place<ConnectEmbeddedPayoutsFeatures> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConnectEmbeddedPayoutsFeaturesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConnectEmbeddedPayoutsFeaturesBuilder {
        type Out = ConnectEmbeddedPayoutsFeatures;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "edit_payout_schedule" => Deserialize::begin(&mut self.edit_payout_schedule),
                "instant_payouts" => Deserialize::begin(&mut self.instant_payouts),
                "standard_payouts" => Deserialize::begin(&mut self.standard_payouts),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                edit_payout_schedule: Deserialize::default(),
                instant_payouts: Deserialize::default(),
                standard_payouts: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                edit_payout_schedule: self.edit_payout_schedule?,
                instant_payouts: self.instant_payouts?,
                standard_payouts: self.standard_payouts?,
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

    impl ObjectDeser for ConnectEmbeddedPayoutsFeatures {
        type Builder = ConnectEmbeddedPayoutsFeaturesBuilder;
    }

    impl FromValueOpt for ConnectEmbeddedPayoutsFeatures {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConnectEmbeddedPayoutsFeaturesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "edit_payout_schedule" => {
                        b.edit_payout_schedule = Some(FromValueOpt::from_value(v)?)
                    }
                    "instant_payouts" => b.instant_payouts = Some(FromValueOpt::from_value(v)?),
                    "standard_payouts" => b.standard_payouts = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
