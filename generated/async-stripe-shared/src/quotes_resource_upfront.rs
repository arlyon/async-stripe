#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct QuotesResourceUpfront {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,
    /// Total after discounts and taxes are applied.
    pub amount_total: i64,
    /// The line items that will appear on the next invoice after this quote is accepted.
    /// This does not include pending invoice items that exist on the customer but may still be included in the next invoice.
    pub line_items: Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>,
    pub total_details: stripe_shared::QuotesResourceTotalDetails,
}
#[doc(hidden)]
pub struct QuotesResourceUpfrontBuilder {
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    line_items: Option<Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>>,
    total_details: Option<stripe_shared::QuotesResourceTotalDetails>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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

    impl Deserialize for QuotesResourceUpfront {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<QuotesResourceUpfront>,
        builder: QuotesResourceUpfrontBuilder,
    }

    impl Visitor for Place<QuotesResourceUpfront> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: QuotesResourceUpfrontBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for QuotesResourceUpfrontBuilder {
        type Out = QuotesResourceUpfront;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_subtotal" => Deserialize::begin(&mut self.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.amount_total),
                "line_items" => Deserialize::begin(&mut self.line_items),
                "total_details" => Deserialize::begin(&mut self.total_details),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_subtotal: Deserialize::default(),
                amount_total: Deserialize::default(),
                line_items: Deserialize::default(),
                total_details: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_subtotal), Some(amount_total), Some(line_items), Some(total_details)) = (
                self.amount_subtotal,
                self.amount_total,
                self.line_items.take(),
                self.total_details.take(),
            ) else {
                return None;
            };
            Some(Self::Out { amount_subtotal, amount_total, line_items, total_details })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for QuotesResourceUpfront {
        type Builder = QuotesResourceUpfrontBuilder;
    }

    impl FromValueOpt for QuotesResourceUpfront {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = QuotesResourceUpfrontBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_subtotal" => b.amount_subtotal = FromValueOpt::from_value(v),
                    "amount_total" => b.amount_total = FromValueOpt::from_value(v),
                    "line_items" => b.line_items = FromValueOpt::from_value(v),
                    "total_details" => b.total_details = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
