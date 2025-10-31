#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingParentsInvoiceQuoteParent {
    /// The quote that generated this invoice
    pub quote: String,
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingParentsInvoiceQuoteParentBuilder {
    quote: Option<String>,
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

    impl Deserialize for BillingBillResourceInvoicingParentsInvoiceQuoteParent {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingParentsInvoiceQuoteParent>,
        builder: BillingBillResourceInvoicingParentsInvoiceQuoteParentBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingParentsInvoiceQuoteParent> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    BillingBillResourceInvoicingParentsInvoiceQuoteParentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingBillResourceInvoicingParentsInvoiceQuoteParentBuilder {
        type Out = BillingBillResourceInvoicingParentsInvoiceQuoteParent;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "quote" => Deserialize::begin(&mut self.quote),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { quote: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(quote),) = (self.quote.take(),) else {
                return None;
            };
            Some(Self::Out { quote })
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

    impl ObjectDeser for BillingBillResourceInvoicingParentsInvoiceQuoteParent {
        type Builder = BillingBillResourceInvoicingParentsInvoiceQuoteParentBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingParentsInvoiceQuoteParent {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BillingBillResourceInvoicingParentsInvoiceQuoteParentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "quote" => b.quote = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
