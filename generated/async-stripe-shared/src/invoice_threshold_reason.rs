#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceThresholdReason {
    /// The total invoice amount threshold boundary if it triggered the threshold invoice.
    pub amount_gte: Option<i64>,
    /// Indicates which line items triggered a threshold invoice.
    pub item_reasons: Vec<stripe_shared::InvoiceItemThresholdReason>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceThresholdReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceThresholdReason").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceThresholdReasonBuilder {
    amount_gte: Option<Option<i64>>,
    item_reasons: Option<Vec<stripe_shared::InvoiceItemThresholdReason>>,
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
                builder: InvoiceThresholdReasonBuilder {
                    amount_gte: Deserialize::default(),
                    item_reasons: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_gte" => Deserialize::begin(&mut self.builder.amount_gte),
                "item_reasons" => Deserialize::begin(&mut self.builder.item_reasons),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount_gte), Some(item_reasons)) =
                (self.builder.amount_gte, self.builder.item_reasons.take())
            else {
                return Ok(());
            };
            *self.out = Some(InvoiceThresholdReason { amount_gte, item_reasons });
            Ok(())
        }
    }
};
