#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNoteRefund {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreditNoteRefund").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: CreditNoteRefundBuilder {
                    amount_refunded: Deserialize::default(),
                    payment_record_refund: Deserialize::default(),
                    refund: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount_refunded" => Deserialize::begin(&mut self.builder.amount_refunded),
                "payment_record_refund" => {
                    Deserialize::begin(&mut self.builder.payment_record_refund)
                }
                "refund" => Deserialize::begin(&mut self.builder.refund),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(amount_refunded), Some(payment_record_refund), Some(refund), Some(type_)) = (
                self.builder.amount_refunded,
                self.builder.payment_record_refund.take(),
                self.builder.refund.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(CreditNoteRefund { amount_refunded, payment_record_refund, refund, type_ });
            Ok(())
        }
    }
};
/// Type of the refund, one of `refund` or `payment_record_refund`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreditNoteRefundType {
    PaymentRecordRefund,
    Refund,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreditNoteRefundType {
    pub fn as_str(&self) -> &str {
        use CreditNoteRefundType::*;
        match self {
            PaymentRecordRefund => "payment_record_refund",
            Refund => "refund",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreditNoteRefundType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteRefundType::*;
        match s {
            "payment_record_refund" => Ok(PaymentRecordRefund),
            "refund" => Ok(Refund),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreditNoteRefundType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreditNoteRefundType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreditNoteRefundType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNoteRefundType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreditNoteRefundType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for CreditNoteRefundType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CreditNoteRefundType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteRefundType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNoteRefundType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
