/// An Add Invoice Item describes the prices and quantities that will be added as pending invoice items when entering a phase.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionScheduleAddInvoiceItem {
    /// The stackable discounts that will be applied to the item.
    pub discounts: Vec<stripe_shared::DiscountsResourceStackableDiscount>,
    /// ID of the price used to generate the invoice item.
    pub price: stripe_types::Expandable<stripe_shared::Price>,
    /// The quantity of the invoice item.
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
}
#[doc(hidden)]
pub struct SubscriptionScheduleAddInvoiceItemBuilder {
    discounts: Option<Vec<stripe_shared::DiscountsResourceStackableDiscount>>,
    price: Option<stripe_types::Expandable<stripe_shared::Price>>,
    quantity: Option<Option<u64>>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
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

    impl Deserialize for SubscriptionScheduleAddInvoiceItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionScheduleAddInvoiceItem>,
        builder: SubscriptionScheduleAddInvoiceItemBuilder,
    }

    impl Visitor for Place<SubscriptionScheduleAddInvoiceItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionScheduleAddInvoiceItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionScheduleAddInvoiceItemBuilder {
        type Out = SubscriptionScheduleAddInvoiceItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discounts" => Deserialize::begin(&mut self.discounts),
                "price" => Deserialize::begin(&mut self.price),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "tax_rates" => Deserialize::begin(&mut self.tax_rates),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                discounts: Deserialize::default(),
                price: Deserialize::default(),
                quantity: Deserialize::default(),
                tax_rates: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(discounts), Some(price), Some(quantity), Some(tax_rates)) =
                (self.discounts.take(), self.price.take(), self.quantity, self.tax_rates.take())
            else {
                return None;
            };
            Some(Self::Out { discounts, price, quantity, tax_rates })
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

    impl ObjectDeser for SubscriptionScheduleAddInvoiceItem {
        type Builder = SubscriptionScheduleAddInvoiceItemBuilder;
    }

    impl FromValueOpt for SubscriptionScheduleAddInvoiceItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionScheduleAddInvoiceItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "discounts" => b.discounts = FromValueOpt::from_value(v),
                    "price" => b.price = FromValueOpt::from_value(v),
                    "quantity" => b.quantity = FromValueOpt::from_value(v),
                    "tax_rates" => b.tax_rates = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
