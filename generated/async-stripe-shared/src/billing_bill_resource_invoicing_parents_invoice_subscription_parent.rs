#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingParentsInvoiceSubscriptionParent {
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) defined as subscription metadata when an invoice is created.
    /// Becomes an immutable snapshot of the subscription metadata at the time of invoice finalization.
    ///  *Note: This attribute is populated only for invoices created on or after June 29, 2023.*
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The subscription that generated this invoice
    pub subscription: stripe_types::Expandable<stripe_shared::Subscription>,
    /// Only set for upcoming invoices that preview prorations. The time used to calculate prorations.
    pub subscription_proration_date: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingBillResourceInvoicingParentsInvoiceSubscriptionParent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingBillResourceInvoicingParentsInvoiceSubscriptionParent")
            .finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: BillingBillResourceInvoicingParentsInvoiceSubscriptionParentBuilder {
                    metadata: Deserialize::default(),
                    subscription: Deserialize::default(),
                    subscription_proration_date: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "subscription" => Deserialize::begin(&mut self.builder.subscription),
                "subscription_proration_date" => {
                    Deserialize::begin(&mut self.builder.subscription_proration_date)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(metadata), Some(subscription), Some(subscription_proration_date)) = (
                self.builder.metadata.take(),
                self.builder.subscription.take(),
                self.builder.subscription_proration_date,
            ) else {
                return Ok(());
            };
            *self.out = Some(BillingBillResourceInvoicingParentsInvoiceSubscriptionParent {
                metadata,
                subscription,
                subscription_proration_date,
            });
            Ok(())
        }
    }
};
