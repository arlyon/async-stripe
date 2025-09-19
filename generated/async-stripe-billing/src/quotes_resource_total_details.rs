#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceTotalDetails {
    /// This is the sum of all the discounts.
    pub amount_discount: i64,
    /// This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,
    /// This is the sum of all the tax amounts.
    pub amount_tax: i64,
    pub breakdown: Option<stripe_billing::QuotesResourceTotalDetailsResourceBreakdown>,
}
#[doc(hidden)]
pub struct QuotesResourceTotalDetailsBuilder {
    amount_discount: Option<i64>,
    amount_shipping: Option<Option<i64>>,
    amount_tax: Option<i64>,
    breakdown: Option<Option<stripe_billing::QuotesResourceTotalDetailsResourceBreakdown>>,
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

    impl Deserialize for QuotesResourceTotalDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceTotalDetails>,
        builder: QuotesResourceTotalDetailsBuilder,
    }

    impl Visitor for Place<QuotesResourceTotalDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceTotalDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for QuotesResourceTotalDetailsBuilder {
        type Out = QuotesResourceTotalDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_discount" => Deserialize::begin(&mut self.amount_discount),
                "amount_shipping" => Deserialize::begin(&mut self.amount_shipping),
                "amount_tax" => Deserialize::begin(&mut self.amount_tax),
                "breakdown" => Deserialize::begin(&mut self.breakdown),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_discount: Deserialize::default(),
                amount_shipping: Deserialize::default(),
                amount_tax: Deserialize::default(),
                breakdown: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_discount), Some(amount_shipping), Some(amount_tax), Some(breakdown)) =
                (self.amount_discount, self.amount_shipping, self.amount_tax, self.breakdown.take())
            else {
                return None;
            };
            Some(Self::Out { amount_discount, amount_shipping, amount_tax, breakdown })
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

    impl ObjectDeser for QuotesResourceTotalDetails {
        type Builder = QuotesResourceTotalDetailsBuilder;
    }

    impl FromValueOpt for QuotesResourceTotalDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = QuotesResourceTotalDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_discount" => b.amount_discount = FromValueOpt::from_value(v),
                    "amount_shipping" => b.amount_shipping = FromValueOpt::from_value(v),
                    "amount_tax" => b.amount_tax = FromValueOpt::from_value(v),
                    "breakdown" => b.breakdown = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
