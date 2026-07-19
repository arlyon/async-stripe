#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BalanceDetailUngated {
    /// Funds that are available for use.
    pub available: Vec<stripe_core::BalanceAmount>,
    /// Funds that are pending
    pub pending: Vec<stripe_core::BalanceAmount>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BalanceDetailUngated {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BalanceDetailUngated").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BalanceDetailUngatedBuilder {
    available: Option<Vec<stripe_core::BalanceAmount>>,
    pending: Option<Vec<stripe_core::BalanceAmount>>,
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
                builder: BalanceDetailUngatedBuilder {
                    available: Deserialize::default(),
                    pending: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "available" => Deserialize::begin(&mut self.builder.available),
                "pending" => Deserialize::begin(&mut self.builder.pending),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(available), Some(pending)) =
                (self.builder.available.take(), self.builder.pending.take())
            else {
                return Ok(());
            };
            *self.out = Some(BalanceDetailUngated { available, pending });
            Ok(())
        }
    }
};
