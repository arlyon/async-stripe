/// A phase item describes the price and quantity of a phase.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionScheduleConfigurationItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionItemBillingThresholds>,
    /// The discounts applied to the subscription item.
    /// Subscription item discounts are applied before subscription discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Vec<stripe_shared::DiscountsResourceStackableDiscount>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an item.
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
#[doc(hidden)]
pub struct SubscriptionScheduleConfigurationItemBuilder {
    billing_thresholds: Option<Option<stripe_shared::SubscriptionItemBillingThresholds>>,
    discounts: Option<Vec<stripe_shared::DiscountsResourceStackableDiscount>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    plan: Option<stripe_types::Expandable<stripe_shared::Plan>>,
    price: Option<stripe_types::Expandable<stripe_shared::Price>>,
    quantity: Option<Option<u64>>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
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
                builder: SubscriptionScheduleConfigurationItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionScheduleConfigurationItemBuilder {
        type Out = SubscriptionScheduleConfigurationItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_thresholds" => Deserialize::begin(&mut self.billing_thresholds),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "plan" => Deserialize::begin(&mut self.plan),
                "price" => Deserialize::begin(&mut self.price),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "tax_rates" => Deserialize::begin(&mut self.tax_rates),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                billing_thresholds: Deserialize::default(),
                discounts: Deserialize::default(),
                metadata: Deserialize::default(),
                plan: Deserialize::default(),
                price: Deserialize::default(),
                quantity: Deserialize::default(),
                tax_rates: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(billing_thresholds),
                Some(discounts),
                Some(metadata),
                Some(plan),
                Some(price),
                Some(quantity),
                Some(tax_rates),
            ) = (
                self.billing_thresholds,
                self.discounts.take(),
                self.metadata.take(),
                self.plan.take(),
                self.price.take(),
                self.quantity,
                self.tax_rates.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                billing_thresholds,
                discounts,
                metadata,
                plan,
                price,
                quantity,
                tax_rates,
            })
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

    impl ObjectDeser for SubscriptionScheduleConfigurationItem {
        type Builder = SubscriptionScheduleConfigurationItemBuilder;
    }

    impl FromValueOpt for SubscriptionScheduleConfigurationItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionScheduleConfigurationItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_thresholds" => b.billing_thresholds = FromValueOpt::from_value(v),
                    "discounts" => b.discounts = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "plan" => b.plan = FromValueOpt::from_value(v),
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
