/// A phase item describes the price and quantity of a phase.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionScheduleConfigurationItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionItemBillingThresholds>,
    /// The discounts applied to the subscription item.
    /// Subscription item discounts are applied before subscription discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Vec<stripe_shared::StackableDiscountWithDiscountSettings>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an item.
    /// Metadata on this item will update the underlying subscription item's `metadata` when the phase is entered.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// ID of the plan to which the customer should be subscribed.
    pub plan: stripe_types::Expandable<stripe_shared::Plan>,
    /// ID of the price to which the customer should be subscribed.
    pub price: stripe_types::Expandable<stripe_shared::Price>,
    /// Quantity of the plan to which the customer should be subscribed.
    pub quantity: Option<u64>,
    /// The tax rates which apply to this `phase_item`.
    /// When set, the `default_tax_rates` on the phase do not apply to this `phase_item`.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionScheduleConfigurationItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionScheduleConfigurationItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionScheduleConfigurationItemBuilder {
    billing_thresholds: Option<Option<stripe_shared::SubscriptionItemBillingThresholds>>,
    discounts: Option<Vec<stripe_shared::StackableDiscountWithDiscountSettings>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    plan: Option<stripe_types::Expandable<stripe_shared::Plan>>,
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

    impl Deserialize for SubscriptionScheduleConfigurationItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionScheduleConfigurationItem>,
        builder: SubscriptionScheduleConfigurationItemBuilder,
    }

    impl Visitor for Place<SubscriptionScheduleConfigurationItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionScheduleConfigurationItemBuilder {
                    billing_thresholds: Deserialize::default(),
                    discounts: Deserialize::default(),
                    metadata: Deserialize::default(),
                    plan: Deserialize::default(),
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
                "billing_thresholds" => Deserialize::begin(&mut self.builder.billing_thresholds),
                "discounts" => Deserialize::begin(&mut self.builder.discounts),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "plan" => Deserialize::begin(&mut self.builder.plan),
                "price" => Deserialize::begin(&mut self.builder.price),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "tax_rates" => Deserialize::begin(&mut self.builder.tax_rates),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(billing_thresholds),
                Some(discounts),
                Some(metadata),
                Some(plan),
                Some(price),
                Some(quantity),
                Some(tax_rates),
            ) = (
                self.builder.billing_thresholds,
                self.builder.discounts.take(),
                self.builder.metadata.take(),
                self.builder.plan.take(),
                self.builder.price.take(),
                self.builder.quantity,
                self.builder.tax_rates.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionScheduleConfigurationItem {
                billing_thresholds,
                discounts,
                metadata,
                plan,
                price,
                quantity,
                tax_rates,
            });
            Ok(())
        }
    }
};
