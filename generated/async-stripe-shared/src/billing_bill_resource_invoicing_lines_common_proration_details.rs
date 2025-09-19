#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingBillResourceInvoicingLinesCommonProrationDetails {
    /// For a credit proration `line_item`, the original debit line_items to which the credit proration applies.
    pub credited_items: Option<stripe_shared::BillingBillResourceInvoicingLinesCommonCreditedItems>,
}
#[doc(hidden)]
pub struct BillingBillResourceInvoicingLinesCommonProrationDetailsBuilder {
    credited_items:
        Option<Option<stripe_shared::BillingBillResourceInvoicingLinesCommonCreditedItems>>,
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

    impl Deserialize for BillingBillResourceInvoicingLinesCommonProrationDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingBillResourceInvoicingLinesCommonProrationDetails>,
        builder: BillingBillResourceInvoicingLinesCommonProrationDetailsBuilder,
    }

    impl Visitor for Place<BillingBillResourceInvoicingLinesCommonProrationDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    BillingBillResourceInvoicingLinesCommonProrationDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingBillResourceInvoicingLinesCommonProrationDetailsBuilder {
        type Out = BillingBillResourceInvoicingLinesCommonProrationDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "credited_items" => Deserialize::begin(&mut self.credited_items),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { credited_items: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(credited_items),) = (self.credited_items.take(),) else {
                return None;
            };
            Some(Self::Out { credited_items })
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

    impl ObjectDeser for BillingBillResourceInvoicingLinesCommonProrationDetails {
        type Builder = BillingBillResourceInvoicingLinesCommonProrationDetailsBuilder;
    }

    impl FromValueOpt for BillingBillResourceInvoicingLinesCommonProrationDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                BillingBillResourceInvoicingLinesCommonProrationDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "credited_items" => b.credited_items = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
