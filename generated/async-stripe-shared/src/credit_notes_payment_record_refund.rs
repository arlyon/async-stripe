#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditNotesPaymentRecordRefund {
    /// ID of the payment record.
    pub payment_record: String,
    /// ID of the refund group.
    pub refund_group: String,
}
#[doc(hidden)]
pub struct CreditNotesPaymentRecordRefundBuilder {
    payment_record: Option<String>,
    refund_group: Option<String>,
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
                builder: CreditNotesPaymentRecordRefundBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CreditNotesPaymentRecordRefundBuilder {
        type Out = CreditNotesPaymentRecordRefund;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_record" => Deserialize::begin(&mut self.payment_record),
                "refund_group" => Deserialize::begin(&mut self.refund_group),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { payment_record: Deserialize::default(), refund_group: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(payment_record), Some(refund_group)) =
                (self.payment_record.take(), self.refund_group.take())
            else {
                return None;
            };
            Some(Self::Out { payment_record, refund_group })
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

    impl ObjectDeser for CreditNotesPaymentRecordRefund {
        type Builder = CreditNotesPaymentRecordRefundBuilder;
    }

    impl FromValueOpt for CreditNotesPaymentRecordRefund {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CreditNotesPaymentRecordRefundBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_record" => b.payment_record = FromValueOpt::from_value(v),
                    "refund_group" => b.refund_group = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
