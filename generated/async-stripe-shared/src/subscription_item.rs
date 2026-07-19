/// Subscription items allow you to create customer subscriptions with more than
/// one plan, making it easy to represent complex billing relationships.
///
/// For more details see <<https://stripe.com/docs/api/subscription_items/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionItemBillingThresholds>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    /// The end time of this subscription item's current billing period.
    pub current_period_end: stripe_types::Timestamp,
    /// The start time of this subscription item's current billing period.
    pub current_period_start: stripe_types::Timestamp,
    /// The discounts applied to the subscription item.
    /// Subscription item discounts are applied before subscription discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Vec<stripe_types::Expandable<stripe_shared::Discount>>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionItemId,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    pub plan: stripe_shared::Plan,
    pub price: stripe_shared::Price,
    /// The [quantity](https://docs.stripe.com/subscriptions/quantities) of the plan to which the customer should be subscribed.
    pub quantity: Option<u64>,
    /// The `subscription` this `subscription_item` belongs to.
    pub subscription: String,
    /// The tax rates which apply to this `subscription_item`.
    /// When set, the `default_tax_rates` on the subscription do not apply to this `subscription_item`.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SubscriptionItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SubscriptionItem").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SubscriptionItemBuilder {
    billing_thresholds: Option<Option<stripe_shared::SubscriptionItemBillingThresholds>>,
    created: Option<i64>,
    current_period_end: Option<stripe_types::Timestamp>,
    current_period_start: Option<stripe_types::Timestamp>,
    discounts: Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>,
    id: Option<stripe_shared::SubscriptionItemId>,
    metadata: Option<std::collections::HashMap<String, String>>,
    plan: Option<stripe_shared::Plan>,
    price: Option<stripe_shared::Price>,
    quantity: Option<Option<u64>>,
    subscription: Option<String>,
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

    impl Deserialize for SubscriptionItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionItem>,
        builder: SubscriptionItemBuilder,
    }

    impl Visitor for Place<SubscriptionItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionItemBuilder {
                    billing_thresholds: Deserialize::default(),
                    created: Deserialize::default(),
                    current_period_end: Deserialize::default(),
                    current_period_start: Deserialize::default(),
                    discounts: Deserialize::default(),
                    id: Deserialize::default(),
                    metadata: Deserialize::default(),
                    plan: Deserialize::default(),
                    price: Deserialize::default(),
                    quantity: Deserialize::default(),
                    subscription: Deserialize::default(),
                    tax_rates: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_thresholds" => Deserialize::begin(&mut self.builder.billing_thresholds),
                "created" => Deserialize::begin(&mut self.builder.created),
                "current_period_end" => Deserialize::begin(&mut self.builder.current_period_end),
                "current_period_start" => {
                    Deserialize::begin(&mut self.builder.current_period_start)
                }
                "discounts" => Deserialize::begin(&mut self.builder.discounts),
                "id" => Deserialize::begin(&mut self.builder.id),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "plan" => Deserialize::begin(&mut self.builder.plan),
                "price" => Deserialize::begin(&mut self.builder.price),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                "subscription" => Deserialize::begin(&mut self.builder.subscription),
                "tax_rates" => Deserialize::begin(&mut self.builder.tax_rates),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(billing_thresholds),
                Some(created),
                Some(current_period_end),
                Some(current_period_start),
                Some(discounts),
                Some(id),
                Some(metadata),
                Some(plan),
                Some(price),
                Some(quantity),
                Some(subscription),
                Some(tax_rates),
            ) = (
                self.builder.billing_thresholds,
                self.builder.created,
                self.builder.current_period_end,
                self.builder.current_period_start,
                self.builder.discounts.take(),
                self.builder.id.take(),
                self.builder.metadata.take(),
                self.builder.plan.take(),
                self.builder.price.take(),
                self.builder.quantity,
                self.builder.subscription.take(),
                self.builder.tax_rates.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SubscriptionItem {
                billing_thresholds,
                created,
                current_period_end,
                current_period_start,
                discounts,
                id,
                metadata,
                plan,
                price,
                quantity,
                subscription,
                tax_rates,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("SubscriptionItem", 13)?;
        s.serialize_field("billing_thresholds", &self.billing_thresholds)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("current_period_end", &self.current_period_end)?;
        s.serialize_field("current_period_start", &self.current_period_start)?;
        s.serialize_field("discounts", &self.discounts)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("plan", &self.plan)?;
        s.serialize_field("price", &self.price)?;
        s.serialize_field("quantity", &self.quantity)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("tax_rates", &self.tax_rates)?;

        s.serialize_field("object", "subscription_item")?;
        s.end()
    }
}
impl stripe_types::Object for SubscriptionItem {
    type Id = stripe_shared::SubscriptionItemId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(SubscriptionItemId);
