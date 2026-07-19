#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent {
    /// The subscription that generated this invoice item
    pub subscription: String,
    /// The subscription item that generated this invoice item
    pub subscription_item: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParentBuilder {
    subscription: Option<String>,
    subscription_item: Option<Option<String>>,
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

    impl Deserialize for BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent>,
        builder: BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParentBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParentBuilder {
                        subscription: Deserialize::default(),
                        subscription_item: Deserialize::default(),
                    },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "subscription" => Deserialize::begin(&mut self.builder.subscription),
                "subscription_item" => Deserialize::begin(&mut self.builder.subscription_item),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(subscription), Some(subscription_item)) =
                (self.builder.subscription.take(), self.builder.subscription_item.take())
            else {
                return Ok(());
            };
            *self.out = Some(BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent {
                subscription,
                subscription_item,
            });
            Ok(())
        }
    }
};
