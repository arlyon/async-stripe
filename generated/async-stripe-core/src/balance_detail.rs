#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceDetail {
    /// Funds that are available for use.
    pub available: Vec<stripe_core::BalanceAmount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceDetail").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceDetailBuilder {
    available: Option<Vec<stripe_core::BalanceAmount>>,
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

    impl Deserialize for BalanceDetail {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BalanceDetail>,
        builder: BalanceDetailBuilder,
    }

    impl Visitor for Place<BalanceDetail> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BalanceDetailBuilder { available: Deserialize::default() },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.builder.available),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(available),) = (self.builder.available.take(),) else {
                return Ok(());
            };
            *self.out = Some(BalanceDetail { available });
            Ok(())
        }
    }
};
