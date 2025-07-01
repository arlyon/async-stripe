#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingParentsInvoiceSubscriptionParent {
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) defined as subscription metadata when an invoice is created.
    /// Becomes an immutable snapshot of the subscription metadata at the time of invoice finalization.
    ///  *Note: This attribute is populated only for invoices created on or after June 29, 2023.*
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The subscription that generated this invoice
    pub subscription: stripe_types::Expandable<stripe_shared::Subscription>,
    /// Only set for upcoming invoices that preview prorations. The time used to calculate prorations.
    pub subscription_proration_date: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingParentsInvoiceSubscriptionParentBuilder {
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    subscription_proration_date: Option<Option<stripe_types::Timestamp>>,
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

    impl Deserialize for BillingBillResourceInvoicingParentsInvoiceSubscriptionParent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingParentsInvoiceSubscriptionParent>,
        builder: BillingBillResourceInvoicingParentsInvoiceSubscriptionParentBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingParentsInvoiceSubscriptionParent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: BillingBillResourceInvoicingParentsInvoiceSubscriptionParentBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for BillingBillResourceInvoicingParentsInvoiceSubscriptionParentBuilder {
        type Out = BillingBillResourceInvoicingParentsInvoiceSubscriptionParent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "metadata" => Deserialize::begin(&mut self.metadata),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "subscription_proration_date" => {
                    Deserialize::begin(&mut self.subscription_proration_date)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                metadata: Deserialize::default(),
                subscription: Deserialize::default(),
                subscription_proration_date: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(metadata), Some(subscription), Some(subscription_proration_date)) =
                (self.metadata.take(), self.subscription.take(), self.subscription_proration_date)
            else {
                return None;
            };
            Some(Self::Out { metadata, subscription, subscription_proration_date })
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

    impl ObjectDeser for BillingBillResourceInvoicingParentsInvoiceSubscriptionParent {
        type Builder = BillingBillResourceInvoicingParentsInvoiceSubscriptionParentBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingParentsInvoiceSubscriptionParent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BillingBillResourceInvoicingParentsInvoiceSubscriptionParentBuilder::deser_default(
                );
            for (k, v) in obj {
                match k.as_str() {
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "subscription" => b.subscription = FromValueOpt::from_value(v),
                    "subscription_proration_date" => {
                        b.subscription_proration_date = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
