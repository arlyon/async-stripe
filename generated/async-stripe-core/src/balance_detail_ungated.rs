#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceDetailUngated {
    /// Funds that are available for use.
    pub available: Vec<stripe_core::BalanceAmount>,
    /// Funds that are pending
    pub pending: Vec<stripe_core::BalanceAmount>,
}
#[doc(hidden)]
pub struct BalanceDetailUngatedBuilder {
    available: Option<Vec<stripe_core::BalanceAmount>>,
    pending: Option<Vec<stripe_core::BalanceAmount>>,
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

    impl Deserialize for BalanceDetailUngated {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceDetailUngated>,
        builder: BalanceDetailUngatedBuilder,
    }

    impl Visitor for Place<BalanceDetailUngated> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceDetailUngatedBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceDetailUngatedBuilder {
        type Out = BalanceDetailUngated;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.available),
                "pending" => Deserialize::begin(&mut self.pending),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { available: Deserialize::default(), pending: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(available), Some(pending)) = (self.available.take(), self.pending.take())
            else {
                return None;
            };
            Some(Self::Out { available, pending })
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

    impl ObjectDeser for BalanceDetailUngated {
        type Builder = BalanceDetailUngatedBuilder;
    }

    impl FromValueOpt for BalanceDetailUngated {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceDetailUngatedBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "available" => b.available = FromValueOpt::from_value(v),
                    "pending" => b.pending = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
