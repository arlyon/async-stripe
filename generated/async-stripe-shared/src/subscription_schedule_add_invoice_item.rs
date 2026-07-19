/// An Add Invoice Item describes the prices and quantities that will be added as pending invoice items when entering a phase.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionScheduleAddInvoiceItem {
    /// The stackable discounts that will be applied to the item.
    pub discounts: Vec<stripe_shared::DiscountsResourceStackableDiscountWithDiscountEnd>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub period: stripe_shared::SubscriptionScheduleAddInvoiceItemPeriod,
    /// ID of the price used to generate the invoice item.
    pub price: stripe_types::Expandable<stripe_shared::Price>,
    /// The quantity of the invoice item.
    pub quantity: Option<u64>,
    /// The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionScheduleAddInvoiceItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionScheduleAddInvoiceItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionScheduleAddInvoiceItemBuilder {
    discounts: Option<Vec<stripe_shared::DiscountsResourceStackableDiscountWithDiscountEnd>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    period: Option<stripe_shared::SubscriptionScheduleAddInvoiceItemPeriod>,
    price: Option<stripe_types::Expandable<stripe_shared::Price>>,
    quantity: Option<Option<u64>>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
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
                builder: SubscriptionScheduleAddInvoiceItemBuilder {
                    discounts: Deserialize::default(),
                    metadata: Deserialize::default(),
                    period: Deserialize::default(),
                    price: Deserialize::default(),
                    quantity: Deserialize::default(),
                    tax_rates: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "discounts" => Deserialize::begin(&mut self.builder.discounts),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "period" => Deserialize::begin(&mut self.builder.period),
                "price" => Deserialize::begin(&mut self.builder.price),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "tax_rates" => Deserialize::begin(&mut self.builder.tax_rates),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(discounts),
                Some(metadata),
                Some(period),
                Some(price),
                Some(quantity),
                Some(tax_rates),
            ) = (
                self.builder.discounts.take(),
                self.builder.metadata.take(),
                self.builder.period.take(),
                self.builder.price.take(),
                self.builder.quantity,
                self.builder.tax_rates.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionScheduleAddInvoiceItem {
                discounts,
                metadata,
                period,
                price,
                quantity,
                tax_rates,
            });
            Ok(())
        }
    }
};
