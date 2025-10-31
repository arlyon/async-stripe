#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent {
    /// The subscription that generated this invoice item
    pub subscription: String,
    /// The subscription item that generated this invoice item
    pub subscription_item: Option<String>,
}
#[doc(hidden)]
pub struct BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParentBuilder {
    subscription: Option<String>,
    subscription_item: Option<Option<String>>,
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
            builder: BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParentBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParentBuilder {
        type Out = BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "subscription" => Deserialize::begin(&mut self.subscription),
                "subscription_item" => Deserialize::begin(&mut self.subscription_item),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { subscription: Deserialize::default(), subscription_item: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(subscription), Some(subscription_item)) =
                (self.subscription.take(), self.subscription_item.take())
            else {
                return None;
            };
            Some(Self::Out { subscription, subscription_item })
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

    impl ObjectDeser for BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent {
        type Builder = BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParentBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingBillResourceInvoiceItemParentsInvoiceItemSubscriptionParentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "subscription" => b.subscription = FromValueOpt::from_value(v),
                    "subscription_item" => b.subscription_item = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
