#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ChargeFraudDetails {
    /// Assessments from Stripe. If set, the value is `fraudulent`.
    pub stripe_report: Option<String>,
    /// Assessments reported by you. If set, possible values of are `safe` and `fraudulent`.
    pub user_report: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ChargeFraudDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ChargeFraudDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ChargeFraudDetailsBuilder {
    stripe_report: Option<Option<String>>,
    user_report: Option<Option<String>>,
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
                builder: ChargeFraudDetailsBuilder {
                    stripe_report: Deserialize::default(),
                    user_report: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "stripe_report" => Deserialize::begin(&mut self.builder.stripe_report),
                "user_report" => Deserialize::begin(&mut self.builder.user_report),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(stripe_report), Some(user_report)) =
                (self.builder.stripe_report.take(), self.builder.user_report.take())
            else {
                return Ok(());
            };
            *self.out = Some(ChargeFraudDetails { stripe_report, user_report });
            Ok(())
        }
    }
};
