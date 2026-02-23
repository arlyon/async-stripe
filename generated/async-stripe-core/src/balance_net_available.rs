#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceNetAvailable {
    /// Net balance amount, subtracting fees from platform-set pricing.
    pub amount: i64,
    /// ID of the external account for this net balance (not expandable).
    pub destination: String,
    pub source_types: Option<stripe_core::BalanceAmountBySourceType>,
}
#[doc(hidden)]
pub struct BalanceNetAvailableBuilder {
    amount: Option<i64>,
    destination: Option<String>,
    source_types: Option<Option<stripe_core::BalanceAmountBySourceType>>,
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

    impl Deserialize for BalanceNetAvailable {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceNetAvailable>,
        builder: BalanceNetAvailableBuilder,
    }

    impl Visitor for Place<BalanceNetAvailable> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceNetAvailableBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BalanceNetAvailableBuilder {
        type Out = BalanceNetAvailable;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "destination" => Deserialize::begin(&mut self.destination),
                "source_types" => Deserialize::begin(&mut self.source_types),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                destination: Deserialize::default(),
                source_types: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount), Some(destination), Some(source_types)) =
                (self.amount, self.destination.take(), self.source_types)
            else {
                return None;
            };
            Some(Self::Out { amount, destination, source_types })
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

    impl ObjectDeser for BalanceNetAvailable {
        type Builder = BalanceNetAvailableBuilder;
    }

    impl FromValueOpt for BalanceNetAvailable {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BalanceNetAvailableBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "destination" => b.destination = FromValueOpt::from_value(v),
                    "source_types" => b.source_types = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
