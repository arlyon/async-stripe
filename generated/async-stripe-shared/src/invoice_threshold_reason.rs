#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceThresholdReason {
    /// The total invoice amount threshold boundary if it triggered the threshold invoice.
    pub amount_gte: Option<i64>,
    /// Indicates which line items triggered a threshold invoice.
    pub item_reasons: Vec<stripe_shared::InvoiceItemThresholdReason>,
}
#[doc(hidden)]
pub struct InvoiceThresholdReasonBuilder {
    amount_gte: Option<Option<i64>>,
    item_reasons: Option<Vec<stripe_shared::InvoiceItemThresholdReason>>,
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

    impl Deserialize for InvoiceThresholdReason {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceThresholdReason>,
        builder: InvoiceThresholdReasonBuilder,
    }

    impl Visitor for Place<InvoiceThresholdReason> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceThresholdReasonBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceThresholdReasonBuilder {
        type Out = InvoiceThresholdReason;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_gte" => Deserialize::begin(&mut self.amount_gte),
                "item_reasons" => Deserialize::begin(&mut self.item_reasons),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount_gte: Deserialize::default(), item_reasons: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_gte), Some(item_reasons)) =
                (self.amount_gte, self.item_reasons.take())
            else {
                return None;
            };
            Some(Self::Out { amount_gte, item_reasons })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for InvoiceThresholdReason {
        type Builder = InvoiceThresholdReasonBuilder;
    }

    impl FromValueOpt for InvoiceThresholdReason {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceThresholdReasonBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_gte" => b.amount_gte = FromValueOpt::from_value(v),
                    "item_reasons" => b.item_reasons = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
