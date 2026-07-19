#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoiceLineItemPeriod {
    /// The end of the period, which must be greater than or equal to the start. This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period. This value is inclusive.
    pub start: stripe_types::Timestamp,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoiceLineItemPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoiceLineItemPeriod").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoiceLineItemPeriodBuilder {
    end: Option<stripe_types::Timestamp>,
    start: Option<stripe_types::Timestamp>,
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

    impl Deserialize for InvoiceLineItemPeriod {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoiceLineItemPeriod>,
        builder: InvoiceLineItemPeriodBuilder,
    }

    impl Visitor for Place<InvoiceLineItemPeriod> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoiceLineItemPeriodBuilder {
                    end: Deserialize::default(),
                    start: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "end" => Deserialize::begin(&mut self.builder.end),
                "start" => Deserialize::begin(&mut self.builder.start),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(end), Some(start)) = (self.builder.end, self.builder.start) else {
                return Ok(());
            };
            *self.out = Some(InvoiceLineItemPeriod { end, start });
            Ok(())
        }
    }
};
