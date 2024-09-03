#[derive(Clone, Debug)]
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
    clippy::let_unit_value,
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
                builder: InvoicesResourceShippingCostBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesResourceShippingCostBuilder {
        type Out = InvoicesResourceShippingCost;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_subtotal" => Deserialize::begin(&mut self.amount_subtotal),
                "amount_tax" => Deserialize::begin(&mut self.amount_tax),
                "amount_total" => Deserialize::begin(&mut self.amount_total),
                "shipping_rate" => Deserialize::begin(&mut self.shipping_rate),
                "taxes" => Deserialize::begin(&mut self.taxes),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_subtotal: Deserialize::default(),
                amount_tax: Deserialize::default(),
                amount_total: Deserialize::default(),
                shipping_rate: Deserialize::default(),
                taxes: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount_subtotal),
                Some(amount_tax),
                Some(amount_total),
                Some(shipping_rate),
                Some(taxes),
            ) = (
                self.amount_subtotal,
                self.amount_tax,
                self.amount_total,
                self.shipping_rate.take(),
                self.taxes.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { amount_subtotal, amount_tax, amount_total, shipping_rate, taxes })
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

    impl ObjectDeser for InvoicesResourceShippingCost {
        type Builder = InvoicesResourceShippingCostBuilder;
    }

    impl FromValueOpt for InvoicesResourceShippingCost {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesResourceShippingCostBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_subtotal" => b.amount_subtotal = FromValueOpt::from_value(v),
                    "amount_tax" => b.amount_tax = FromValueOpt::from_value(v),
                    "amount_total" => b.amount_total = FromValueOpt::from_value(v),
                    "shipping_rate" => b.shipping_rate = FromValueOpt::from_value(v),
                    "taxes" => b.taxes = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
