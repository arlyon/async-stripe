#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent {
    /// The invoice item that generated this line item
    pub invoice_item: String,
    /// Whether this is a proration
    pub proration: bool,
    /// Additional details for proration line items
    pub proration_details:
        Option<stripe_shared::BillingBillResourceInvoicingLinesCommonProrationDetails>,
    /// The subscription that the invoice item belongs to
    pub subscription: Option<String>,
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParentBuilder {
    invoice_item: Option<String>,
    proration: Option<bool>,
    proration_details:
        Option<Option<stripe_shared::BillingBillResourceInvoicingLinesCommonProrationDetails>>,
    subscription: Option<Option<String>>,
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

    impl Deserialize for BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent,
        >,
        builder: BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParentBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParentBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder
        for BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParentBuilder
    {
        type Out = BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice_item" => Deserialize::begin(&mut self.invoice_item),
                "proration" => Deserialize::begin(&mut self.proration),
                "proration_details" => Deserialize::begin(&mut self.proration_details),
                "subscription" => Deserialize::begin(&mut self.subscription),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                invoice_item: Deserialize::default(),
                proration: Deserialize::default(),
                proration_details: Deserialize::default(),
                subscription: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(invoice_item), Some(proration), Some(proration_details), Some(subscription)) = (
                self.invoice_item.take(),
                self.proration,
                self.proration_details.take(),
                self.subscription.take(),
            ) else {
                return None;
            };
            Some(Self::Out { invoice_item, proration, proration_details, subscription })
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

    impl ObjectDeser for BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent {
        type Builder =
            BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParentBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingBillResourceInvoicingLinesParentsInvoiceLineItemInvoiceItemParentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "invoice_item" => b.invoice_item = FromValueOpt::from_value(v),
                    "proration" => b.proration = FromValueOpt::from_value(v),
                    "proration_details" => b.proration_details = FromValueOpt::from_value(v),
                    "subscription" => b.subscription = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
