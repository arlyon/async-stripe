#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditNotesPaymentRecordRefund {
    /// ID of the payment record.
    pub payment_record: String,
    /// ID of the refund group.
    pub refund_group: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNotesPaymentRecordRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreditNotesPaymentRecordRefund").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CreditNotesPaymentRecordRefundBuilder {
    payment_record: Option<String>,
    refund_group: Option<String>,
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

    impl Deserialize for CreditNotesPaymentRecordRefund {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CreditNotesPaymentRecordRefund>,
        builder: CreditNotesPaymentRecordRefundBuilder,
    }

    impl Visitor for Place<CreditNotesPaymentRecordRefund> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CreditNotesPaymentRecordRefundBuilder {
                    payment_record: Deserialize::default(),
                    refund_group: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_record" => Deserialize::begin(&mut self.builder.payment_record),
                "refund_group" => Deserialize::begin(&mut self.builder.refund_group),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(payment_record), Some(refund_group)) =
                (self.builder.payment_record.take(), self.builder.refund_group.take())
            else {
                return Ok(());
            };
            *self.out = Some(CreditNotesPaymentRecordRefund { payment_record, refund_group });
            Ok(())
        }
    }
};
