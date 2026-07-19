#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    pub total_details: stripe_billing::QuotesResourceTotalDetails,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for QuotesResourceUpfront {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("QuotesResourceUpfront").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct QuotesResourceUpfrontBuilder {
    amount_subtotal: Option<i64>,
    amount_total: Option<i64>,
    line_items: Option<Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>>,
    total_details: Option<stripe_billing::QuotesResourceTotalDetails>,
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
                builder: QuotesResourceUpfrontBuilder {
                    amount_subtotal: Deserialize::default(),
                    amount_total: Deserialize::default(),
                    line_items: Deserialize::default(),
                    total_details: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_subtotal" => Deserialize::begin(&mut self.builder.amount_subtotal),
                "amount_total" => Deserialize::begin(&mut self.builder.amount_total),
                "line_items" => Deserialize::begin(&mut self.builder.line_items),
                "total_details" => Deserialize::begin(&mut self.builder.total_details),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount_subtotal), Some(amount_total), Some(line_items), Some(total_details)) = (
                self.builder.amount_subtotal,
                self.builder.amount_total,
                self.builder.line_items.take(),
                self.builder.total_details.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(QuotesResourceUpfront {
                amount_subtotal,
                amount_total,
                line_items,
                total_details,
            });
            Ok(())
        }
    }
};
