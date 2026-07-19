#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesResourceShippingCost {
    /// Total shipping cost before any taxes are applied.
    pub amount_subtotal: i64,
    /// Total tax amount applied due to shipping costs. If no tax was applied, defaults to 0.
    pub amount_tax: i64,
    /// Total shipping cost after taxes are applied.
    pub amount_total: i64,
    /// The ID of the ShippingRate for this invoice.
    pub shipping_rate: Option<stripe_types::Expandable<stripe_shared::ShippingRate>>,
    /// The taxes applied to the shipping rate.
    pub taxes: Option<Vec<stripe_shared::LineItemsTaxAmount>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicesResourceShippingCost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicesResourceShippingCost").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicesResourceShippingCostBuilder {
    amount_subtotal: Option<i64>,
    amount_tax: Option<i64>,
    amount_total: Option<i64>,
    shipping_rate: Option<Option<stripe_types::Expandable<stripe_shared::ShippingRate>>>,
    taxes: Option<Option<Vec<stripe_shared::LineItemsTaxAmount>>>,
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

    impl Deserialize for InvoicesResourceShippingCost {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesResourceShippingCost>,
        builder: InvoicesResourceShippingCostBuilder,
    }

    impl Visitor for Place<InvoicesResourceShippingCost> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesResourceShippingCostBuilder {
                    amount_subtotal: Deserialize::default(),
                    amount_tax: Deserialize::default(),
                    amount_total: Deserialize::default(),
                    shipping_rate: Deserialize::default(),
                    taxes: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_subtotal" => Deserialize::begin(&mut self.builder.amount_subtotal),
                "amount_tax" => Deserialize::begin(&mut self.builder.amount_tax),
                "amount_total" => Deserialize::begin(&mut self.builder.amount_total),
                "shipping_rate" => Deserialize::begin(&mut self.builder.shipping_rate),
                "taxes" => Deserialize::begin(&mut self.builder.taxes),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount_subtotal),
                Some(amount_tax),
                Some(amount_total),
                Some(shipping_rate),
                Some(taxes),
            ) = (
                self.builder.amount_subtotal,
                self.builder.amount_tax,
                self.builder.amount_total,
                self.builder.shipping_rate.take(),
                self.builder.taxes.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(InvoicesResourceShippingCost {
                amount_subtotal,
                amount_tax,
                amount_total,
                shipping_rate,
                taxes,
            });
            Ok(())
        }
    }
};
