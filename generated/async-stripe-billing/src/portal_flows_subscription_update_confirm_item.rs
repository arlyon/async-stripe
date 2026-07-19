#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PortalFlowsSubscriptionUpdateConfirmItem {
    /// The ID of the [subscription item](https://docs.stripe.com/api/subscriptions/object#subscription_object-items-data-id) to be updated.
    pub id: Option<stripe_billing::PortalFlowsSubscriptionUpdateConfirmItemId>,
    /// The price the customer should subscribe to through this flow.
    /// The price must also be included in the configuration's [`features.subscription_update.products`](https://docs.stripe.com/api/customer_portal/configuration#portal_configuration_object-features-subscription_update-products).
    pub price: Option<String>,
    /// [Quantity](https://docs.stripe.com/subscriptions/quantities) for this item that the customer should subscribe to through this flow.
    pub quantity: Option<u64>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PortalFlowsSubscriptionUpdateConfirmItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PortalFlowsSubscriptionUpdateConfirmItem").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PortalFlowsSubscriptionUpdateConfirmItemBuilder {
                    id: Deserialize::default(),
                    price: Deserialize::default(),
                    quantity: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.builder.id),
                "price" => Deserialize::begin(&mut self.builder.price),
                "quantity" => Deserialize::begin(&mut self.builder.quantity),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(id), Some(price), Some(quantity)) =
                (self.builder.id.take(), self.builder.price.take(), self.builder.quantity)
            else {
                return Ok(());
            };
            *self.out = Some(PortalFlowsSubscriptionUpdateConfirmItem { id, price, quantity });
            Ok(())
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
