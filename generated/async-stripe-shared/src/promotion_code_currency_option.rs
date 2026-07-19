#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PromotionCodeCurrencyOption {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PromotionCodeCurrencyOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PromotionCodeCurrencyOption").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PromotionCodeCurrencyOptionBuilder {
    minimum_amount: Option<i64>,
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

    impl Deserialize for PromotionCodeCurrencyOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PromotionCodeCurrencyOption>,
        builder: PromotionCodeCurrencyOptionBuilder,
    }

    impl Visitor for Place<PromotionCodeCurrencyOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PromotionCodeCurrencyOptionBuilder {
                    minimum_amount: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "minimum_amount" => Deserialize::begin(&mut self.builder.minimum_amount),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(minimum_amount),) = (self.builder.minimum_amount,) else {
                return Ok(());
            };
            *self.out = Some(PromotionCodeCurrencyOption { minimum_amount });
            Ok(())
        }
    }
};
