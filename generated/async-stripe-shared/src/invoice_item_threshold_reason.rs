#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceItemThresholdReason {
    /// The IDs of the line items that triggered the threshold invoice.
    pub line_item_ids: Vec<String>,
    /// The quantity threshold boundary that applied to the given line item.
    pub usage_gte: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceItemThresholdReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceItemThresholdReason").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceItemThresholdReasonBuilder {
    line_item_ids: Option<Vec<String>>,
    usage_gte: Option<i64>,
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
                builder: InvoiceItemThresholdReasonBuilder {
                    line_item_ids: Deserialize::default(),
                    usage_gte: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "line_item_ids" => Deserialize::begin(&mut self.builder.line_item_ids),
                "usage_gte" => Deserialize::begin(&mut self.builder.usage_gte),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(line_item_ids), Some(usage_gte)) =
                (self.builder.line_item_ids.take(), self.builder.usage_gte)
            else {
                return Ok(());
            };
            *self.out = Some(InvoiceItemThresholdReason { line_item_ids, usage_gte });
            Ok(())
        }
    }
};
