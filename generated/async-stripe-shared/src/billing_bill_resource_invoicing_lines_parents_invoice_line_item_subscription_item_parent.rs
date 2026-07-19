#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent {
    /// The invoice item that generated this line item
    pub invoice_item: Option<String>,
    /// Whether this is a proration
    pub proration: bool,
    /// Additional details for proration line items
    pub proration_details:
        Option<stripe_shared::BillingBillResourceInvoicingLinesCommonProrationDetails>,
    /// The subscription that the subscription item belongs to
    pub subscription: Option<String>,
    /// The subscription item that generated this line item
    pub subscription_item: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug
    for BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(
            "BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent",
        )
        .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParentBuilder {
    invoice_item: Option<Option<String>>,
    proration: Option<bool>,
    proration_details:
        Option<Option<stripe_shared::BillingBillResourceInvoicingLinesCommonProrationDetails>>,
    subscription: Option<Option<String>>,
    subscription_item: Option<String>,
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

    impl Deserialize for BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent,
        >,
        builder:
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParentBuilder,
    }

    impl Visitor
        for Place<BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent>
    {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParentBuilder { invoice_item: Deserialize::default(),
proration: Deserialize::default(),
proration_details: Deserialize::default(),
subscription: Deserialize::default(),
subscription_item: Deserialize::default(),
 },
        }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice_item" => Deserialize::begin(&mut self.builder.invoice_item),
                "proration" => Deserialize::begin(&mut self.builder.proration),
                "proration_details" => Deserialize::begin(&mut self.builder.proration_details),
                "subscription" => Deserialize::begin(&mut self.builder.subscription),
                "subscription_item" => Deserialize::begin(&mut self.builder.subscription_item),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(invoice_item),
                Some(proration),
                Some(proration_details),
                Some(subscription),
                Some(subscription_item),
            ) = (
                self.builder.invoice_item.take(),
                self.builder.proration,
                self.builder.proration_details.take(),
                self.builder.subscription.take(),
                self.builder.subscription_item.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(
                BillingBillResourceInvoicingLinesParentsInvoiceLineItemSubscriptionItemParent {
                    invoice_item,
                    proration,
                    proration_details,
                    subscription,
                    subscription_item,
                },
            );
            Ok(())
        }
    }
};
