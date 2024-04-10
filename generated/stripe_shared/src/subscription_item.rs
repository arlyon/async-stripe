/// Subscription items allow you to create customer subscriptions with more than
/// one plan, making it easy to represent complex billing relationships.
///
/// For more details see <<https://stripe.com/docs/api/subscription_items/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    pub billing_thresholds: Option<stripe_shared::SubscriptionItemBillingThresholds>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    /// Unique identifier for the object.
    pub id: stripe_shared::SubscriptionItemId,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    pub plan: stripe_shared::Plan,
    pub price: stripe_shared::Price,
    /// The [quantity](https://stripe.com/docs/subscriptions/quantities) of the plan to which the customer should be subscribed.
    pub quantity: Option<u64>,
    /// The `subscription` this `subscription_item` belongs to.
    pub subscription: String,
    /// The tax rates which apply to this `subscription_item`.
    /// When set, the `default_tax_rates` on the subscription do not apply to this `subscription_item`.
    pub tax_rates: Option<Vec<stripe_shared::TaxRate>>,
}
#[doc(hidden)]
pub struct SubscriptionItemBuilder {
    billing_thresholds: Option<Option<stripe_shared::SubscriptionItemBillingThresholds>>,
    created: Option<i64>,
    id: Option<stripe_shared::SubscriptionItemId>,
    metadata: Option<std::collections::HashMap<String, String>>,
    plan: Option<stripe_shared::Plan>,
    price: Option<stripe_shared::Price>,
    quantity: Option<Option<u64>>,
    subscription: Option<String>,
    tax_rates: Option<Option<Vec<stripe_shared::TaxRate>>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SubscriptionItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionItemBuilder {
        type Out = SubscriptionItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "billing_thresholds" => Deserialize::begin(&mut self.billing_thresholds),
                "created" => Deserialize::begin(&mut self.created),
                "id" => Deserialize::begin(&mut self.id),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "plan" => Deserialize::begin(&mut self.plan),
                "price" => Deserialize::begin(&mut self.price),
                "quantity" => Deserialize::begin(&mut self.quantity),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "tax_rates" => Deserialize::begin(&mut self.tax_rates),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                billing_thresholds: Deserialize::default(),
                created: Deserialize::default(),
                id: Deserialize::default(),
                metadata: Deserialize::default(),
                plan: Deserialize::default(),
                price: Deserialize::default(),
                quantity: Deserialize::default(),
                subscription: Deserialize::default(),
                tax_rates: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                billing_thresholds: self.billing_thresholds?,
                created: self.created?,
                id: self.id.take()?,
                metadata: self.metadata.take()?,
                plan: self.plan.take()?,
                price: self.price.take()?,
                quantity: self.quantity?,
                subscription: self.subscription.take()?,
                tax_rates: self.tax_rates.take()?,
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

    impl ObjectDeser for SubscriptionItem {
        type Builder = SubscriptionItemBuilder;
    }

    impl FromValueOpt for SubscriptionItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "billing_thresholds" => {
                        b.billing_thresholds = Some(FromValueOpt::from_value(v)?)
                    }
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "plan" => b.plan = Some(FromValueOpt::from_value(v)?),
                    "price" => b.price = Some(FromValueOpt::from_value(v)?),
                    "quantity" => b.quantity = Some(FromValueOpt::from_value(v)?),
                    "subscription" => b.subscription = Some(FromValueOpt::from_value(v)?),
                    "tax_rates" => b.tax_rates = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("SubscriptionItem", 10)?;
        s.serialize_field("billing_thresholds", &self.billing_thresholds)?;
        s.serialize_field("created", &self.created)?;
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
}
stripe_types::def_id!(SubscriptionItemId);
