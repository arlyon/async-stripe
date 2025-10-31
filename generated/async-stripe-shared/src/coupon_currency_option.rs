#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CouponCurrencyOption {
    /// Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: i64,
}
#[doc(hidden)]
pub struct CouponCurrencyOptionBuilder {
    amount_off: Option<i64>,
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

    impl Deserialize for CouponCurrencyOption {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CouponCurrencyOption>,
        builder: CouponCurrencyOptionBuilder,
    }

    impl Visitor for Place<CouponCurrencyOption> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CouponCurrencyOptionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CouponCurrencyOptionBuilder {
        type Out = CouponCurrencyOption;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_off" => Deserialize::begin(&mut self.amount_off),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount_off: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_off),) = (self.amount_off,) else {
                return None;
            };
            Some(Self::Out { amount_off })
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

    impl ObjectDeser for CouponCurrencyOption {
        type Builder = CouponCurrencyOptionBuilder;
    }

    impl FromValueOpt for CouponCurrencyOption {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CouponCurrencyOptionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_off" => b.amount_off = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
