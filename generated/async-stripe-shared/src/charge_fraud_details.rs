#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ChargeFraudDetails {
    /// Assessments from Stripe. If set, the value is `fraudulent`.
    pub stripe_report: Option<String>,
    /// Assessments reported by you. If set, possible values of are `safe` and `fraudulent`.
    pub user_report: Option<String>,
}
#[doc(hidden)]
pub struct ChargeFraudDetailsBuilder {
    stripe_report: Option<Option<String>>,
    user_report: Option<Option<String>>,
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

    impl Deserialize for ChargeFraudDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ChargeFraudDetails>,
        builder: ChargeFraudDetailsBuilder,
    }

    impl Visitor for Place<ChargeFraudDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ChargeFraudDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ChargeFraudDetailsBuilder {
        type Out = ChargeFraudDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "stripe_report" => Deserialize::begin(&mut self.stripe_report),
                "user_report" => Deserialize::begin(&mut self.user_report),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { stripe_report: Deserialize::default(), user_report: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(stripe_report), Some(user_report)) =
                (self.stripe_report.take(), self.user_report.take())
            else {
                return None;
            };
            Some(Self::Out { stripe_report, user_report })
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

    impl ObjectDeser for ChargeFraudDetails {
        type Builder = ChargeFraudDetailsBuilder;
    }

    impl FromValueOpt for ChargeFraudDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ChargeFraudDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "stripe_report" => b.stripe_report = FromValueOpt::from_value(v),
                    "user_report" => b.user_report = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
