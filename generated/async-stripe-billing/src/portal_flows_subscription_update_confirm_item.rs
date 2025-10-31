#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsSubscriptionUpdateConfirmItem {
    /// The ID of the [subscription item](https://stripe.com/docs/api/subscriptions/object#subscription_object-items-data-id) to be updated.
    pub id: Option<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItemId>,
    /// The price the customer should subscribe to through this flow.
    /// The price must also be included in the configuration's [`features.subscription_update.products`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
    pub price: Option<String>,
    /// [Quantity](https://stripe.com/docs/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
    pub quantity: Option<u64>,
}
#[doc(hidden)]
pub struct PortalFlowsSubscriptionUpdateConfirmItemBuilder {
    id: Option<Option<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItemId>>,
    price: Option<Option<String>>,
    quantity: Option<Option<u64>>,
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
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PortalFlowsSubscriptionUpdateConfirmItem {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PortalFlowsSubscriptionUpdateConfirmItem>,
        builder: PortalFlowsSubscriptionUpdateConfirmItemBuilder,
    }

    impl Visitor for Place<PortalFlowsSubscriptionUpdateConfirmItem> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PortalFlowsSubscriptionUpdateConfirmItemBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PortalFlowsSubscriptionUpdateConfirmItemBuilder {
        type Out = PortalFlowsSubscriptionUpdateConfirmItem;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "price" => Deserialize::begin(&mut self.price),
                "quantity" => Deserialize::begin(&mut self.quantity),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                id: Deserialize::default(),
                price: Deserialize::default(),
                quantity: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(id), Some(price), Some(quantity)) =
                (self.id.take(), self.price.take(), self.quantity)
            else {
                return None;
            };
            Some(Self::Out { id, price, quantity })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PortalFlowsSubscriptionUpdateConfirmItem {
        type Builder = PortalFlowsSubscriptionUpdateConfirmItemBuilder;
    }

    impl FromValueOpt for PortalFlowsSubscriptionUpdateConfirmItem {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PortalFlowsSubscriptionUpdateConfirmItemBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = FromValueOpt::from_value(v),
                    "price" => b.price = FromValueOpt::from_value(v),
                    "quantity" => b.quantity = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
impl stripe_types::Object for PortalFlowsSubscriptionUpdateConfirmItem {
    type Id = Option<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItemId>;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PortalFlowsSubscriptionUpdateConfirmItemId);
