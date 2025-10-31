#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CreditNoteRefund {
    /// Amount of the refund that applies to this credit note, in cents (or local equivalent).
    pub amount_refunded: i64,
    /// The PaymentRecord refund details associated with this credit note refund.
    pub payment_record_refund: Option<stripe_shared::CreditNotesPaymentRecordRefund>,
    /// ID of the refund.
    pub refund: stripe_types::Expandable<stripe_shared::Refund>,
    /// Type of the refund, one of `refund` or `payment_record_refund`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: Option<CreditNoteRefundType>,
}
#[doc(hidden)]
pub struct CreditNoteRefundBuilder {
    amount_refunded: Option<i64>,
    payment_record_refund: Option<Option<stripe_shared::CreditNotesPaymentRecordRefund>>,
    refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
    type_: Option<Option<CreditNoteRefundType>>,
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
                "payment_record_refund" => Deserialize::begin(&mut self.payment_record_refund),
                "refund" => Deserialize::begin(&mut self.refund),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount_refunded: Deserialize::default(),
                payment_record_refund: Deserialize::default(),
                refund: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(amount_refunded), Some(payment_record_refund), Some(refund), Some(type_)) = (
                self.amount_refunded,
                self.payment_record_refund.take(),
                self.refund.take(),
                self.type_,
            ) else {
                return None;
            };
            Some(Self::Out { amount_refunded, payment_record_refund, refund, type_ })
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
                    "payment_record_refund" => {
                        b.payment_record_refund = FromValueOpt::from_value(v)
                    }
                    "refund" => b.refund = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of the refund, one of `refund` or `payment_record_refund`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreditNoteRefundType {
    PaymentRecordRefund,
    Refund,
}
impl CreditNoteRefundType {
    pub fn as_str(self) -> &'static str {
        use CreditNoteRefundType::*;
        match self {
            PaymentRecordRefund => "payment_record_refund",
            Refund => "refund",
        }
    }
}

impl std::str::FromStr for CreditNoteRefundType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteRefundType::*;
        match s {
            "payment_record_refund" => Ok(PaymentRecordRefund),
            "refund" => Ok(Refund),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreditNoteRefundType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteRefundType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNoteRefundType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreditNoteRefundType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreditNoteRefundType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteRefundType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreditNoteRefundType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNoteRefundType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteRefundType"))
    }
}
