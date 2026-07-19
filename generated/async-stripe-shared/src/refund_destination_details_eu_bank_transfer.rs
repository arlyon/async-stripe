#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RefundDestinationDetailsEuBankTransfer {
    /// The reference assigned to the refund.
    pub reference: Option<String>,
    /// Status of the reference on the refund. This can be `pending`, `available` or `unavailable`.
    pub reference_status: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RefundDestinationDetailsEuBankTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RefundDestinationDetailsEuBankTransfer").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct RefundDestinationDetailsEuBankTransferBuilder {
    reference: Option<Option<String>>,
    reference_status: Option<Option<String>>,
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

    impl Deserialize for RefundDestinationDetailsEuBankTransfer {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RefundDestinationDetailsEuBankTransfer>,
        builder: RefundDestinationDetailsEuBankTransferBuilder,
    }

    impl Visitor for Place<RefundDestinationDetailsEuBankTransfer> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RefundDestinationDetailsEuBankTransferBuilder {
                    reference: Deserialize::default(),
                    reference_status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "reference" => Deserialize::begin(&mut self.builder.reference),
                "reference_status" => Deserialize::begin(&mut self.builder.reference_status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(reference), Some(reference_status)) =
                (self.builder.reference.take(), self.builder.reference_status.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(RefundDestinationDetailsEuBankTransfer { reference, reference_status });
            Ok(())
        }
    }
};
