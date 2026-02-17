#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceItemThresholdReason {
    /// The IDs of the line items that triggered the threshold invoice.
    pub line_item_ids: Vec<String>,
    /// The quantity threshold boundary that applied to the given line item.
    pub usage_gte: i64,
}
#[doc(hidden)]
pub struct InvoiceItemThresholdReasonBuilder {
    line_item_ids: Option<Vec<String>>,
    usage_gte: Option<i64>,
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

    impl Deserialize for InvoiceItemThresholdReason {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceItemThresholdReason>,
        builder: InvoiceItemThresholdReasonBuilder,
    }

    impl Visitor for Place<InvoiceItemThresholdReason> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceItemThresholdReasonBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoiceItemThresholdReasonBuilder {
        type Out = InvoiceItemThresholdReason;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "line_item_ids" => Deserialize::begin(&mut self.line_item_ids),
                "usage_gte" => Deserialize::begin(&mut self.usage_gte),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { line_item_ids: Deserialize::default(), usage_gte: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(line_item_ids), Some(usage_gte)) =
                (self.line_item_ids.take(), self.usage_gte)
            else {
                return None;
            };
            Some(Self::Out { line_item_ids, usage_gte })
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

    impl ObjectDeser for InvoiceItemThresholdReason {
        type Builder = InvoiceItemThresholdReasonBuilder;
    }

    impl FromValueOpt for InvoiceItemThresholdReason {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceItemThresholdReasonBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "line_item_ids" => b.line_item_ids = FromValueOpt::from_value(v),
                    "usage_gte" => b.usage_gte = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
