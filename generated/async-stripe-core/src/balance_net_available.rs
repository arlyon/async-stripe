#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceNetAvailable {
    /// Net balance amount, subtracting fees from platform-set pricing.
    pub amount: i64,
    /// ID of the external account for this net balance (not expandable).
    pub destination: String,
    pub source_types: Option<stripe_core::BalanceAmountBySourceType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceNetAvailable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceNetAvailable").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BalanceNetAvailableBuilder {
                    amount: Deserialize::default(),
                    destination: Deserialize::default(),
                    source_types: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "destination" => Deserialize::begin(&mut self.builder.destination),
                "source_types" => Deserialize::begin(&mut self.builder.source_types),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount), Some(destination), Some(source_types)) =
                (self.builder.amount, self.builder.destination.take(), self.builder.source_types)
            else {
                return Ok(());
            };
            *self.out = Some(BalanceNetAvailable { amount, destination, source_types });
            Ok(())
        }
    }
};
