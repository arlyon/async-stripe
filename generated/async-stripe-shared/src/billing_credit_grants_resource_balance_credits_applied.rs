#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct BillingCreditGrantsResourceBalanceCreditsApplied {
    /// The invoice to which the billing credits were applied.
    pub invoice: stripe_types::Expandable<stripe_shared::Invoice>,
    /// The invoice line item to which the billing credits were applied.
    pub invoice_line_item: String,
}
#[doc(hidden)]
pub struct BillingCreditGrantsResourceBalanceCreditsAppliedBuilder {
    invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    invoice_line_item: Option<String>,
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

    impl Deserialize for BillingCreditGrantsResourceBalanceCreditsApplied {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<BillingCreditGrantsResourceBalanceCreditsApplied>,
        builder: BillingCreditGrantsResourceBalanceCreditsAppliedBuilder,
    }

    impl Visitor for Place<BillingCreditGrantsResourceBalanceCreditsApplied> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: BillingCreditGrantsResourceBalanceCreditsAppliedBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for BillingCreditGrantsResourceBalanceCreditsAppliedBuilder {
        type Out = BillingCreditGrantsResourceBalanceCreditsApplied;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "invoice" => Deserialize::begin(&mut self.invoice),
                "invoice_line_item" => Deserialize::begin(&mut self.invoice_line_item),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { invoice: Deserialize::default(), invoice_line_item: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(invoice), Some(invoice_line_item)) =
                (self.invoice.take(), self.invoice_line_item.take())
            else {
                return None;
            };
            Some(Self::Out { invoice, invoice_line_item })
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

    impl ObjectDeser for BillingCreditGrantsResourceBalanceCreditsApplied {
        type Builder = BillingCreditGrantsResourceBalanceCreditsAppliedBuilder;
    }

    impl FromValueOpt for BillingCreditGrantsResourceBalanceCreditsApplied {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = BillingCreditGrantsResourceBalanceCreditsAppliedBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "invoice" => b.invoice = FromValueOpt::from_value(v),
                    "invoice_line_item" => b.invoice_line_item = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
