#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditNoteRefund {
    /// Amount of the refund that applies to this credit note, in cents (or local equivalent).
    pub amount_refunded: i64,
    /// ID of the refund.
    pub refund: stripe_types::Expandable<stripe_shared::Refund>,
}
#[doc(hidden)]
pub struct CreditNoteRefundBuilder {
    amount_refunded: Option<i64>,
    refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
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

    impl Deserialize for CreditNoteRefund {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CreditNoteRefund>,
        builder: CreditNoteRefundBuilder,
    }

    impl Visitor for Place<CreditNoteRefund> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CreditNoteRefundBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CreditNoteRefundBuilder {
        type Out = CreditNoteRefund;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_refunded" => Deserialize::begin(&mut self.amount_refunded),
                "refund" => Deserialize::begin(&mut self.refund),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { amount_refunded: Deserialize::default(), refund: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_refunded), Some(refund)) = (self.amount_refunded, self.refund.take())
            else {
                return None;
            };
            Some(Self::Out { amount_refunded, refund })
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

    impl ObjectDeser for CreditNoteRefund {
        type Builder = CreditNoteRefundBuilder;
    }

    impl FromValueOpt for CreditNoteRefund {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CreditNoteRefundBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount_refunded" => b.amount_refunded = FromValueOpt::from_value(v),
                    "refund" => b.refund = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
