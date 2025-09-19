#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PromotionCodeCurrencyOption {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: i64,
}
#[doc(hidden)]
pub struct PromotionCodeCurrencyOptionBuilder {
    minimum_amount: Option<i64>,
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
                builder: PromotionCodeCurrencyOptionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PromotionCodeCurrencyOptionBuilder {
        type Out = PromotionCodeCurrencyOption;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "minimum_amount" => Deserialize::begin(&mut self.minimum_amount),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { minimum_amount: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(minimum_amount),) = (self.minimum_amount,) else {
                return None;
            };
            Some(Self::Out { minimum_amount })
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

    impl ObjectDeser for PromotionCodeCurrencyOption {
        type Builder = PromotionCodeCurrencyOptionBuilder;
    }

    impl FromValueOpt for PromotionCodeCurrencyOption {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PromotionCodeCurrencyOptionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "minimum_amount" => b.minimum_amount = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
