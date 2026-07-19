#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesPaymentsInvoicePaymentAssociatedPayment {
    /// ID of the successful charge for this payment when `type` is `charge`.Note: charge is only surfaced if the charge object is not associated with a payment intent.
    /// If the charge object does have a payment intent, the Invoice Payment surfaces the payment intent instead.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// ID of the PaymentIntent associated with this payment when `type` is `payment_intent`.
    /// Note: This property is only populated for invoices finalized on or after March 15th, 2019.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    /// ID of the PaymentRecord associated with this payment when `type` is `payment_record`.
    pub payment_record: Option<stripe_types::Expandable<stripe_shared::PaymentRecord>>,
    /// Type of payment object associated with this invoice payment.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: InvoicesPaymentsInvoicePaymentAssociatedPaymentType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicesPaymentsInvoicePaymentAssociatedPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("InvoicesPaymentsInvoicePaymentAssociatedPayment").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct InvoicesPaymentsInvoicePaymentAssociatedPaymentBuilder {
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    payment_record: Option<Option<stripe_types::Expandable<stripe_shared::PaymentRecord>>>,
    type_: Option<InvoicesPaymentsInvoicePaymentAssociatedPaymentType>,
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

    impl Deserialize for InvoicesPaymentsInvoicePaymentAssociatedPayment {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<InvoicesPaymentsInvoicePaymentAssociatedPayment>,
        builder: InvoicesPaymentsInvoicePaymentAssociatedPaymentBuilder,
    }

    impl Visitor for Place<InvoicesPaymentsInvoicePaymentAssociatedPayment> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: InvoicesPaymentsInvoicePaymentAssociatedPaymentBuilder {
                    charge: Deserialize::default(),
                    payment_intent: Deserialize::default(),
                    payment_record: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.builder.charge),
                "payment_intent" => Deserialize::begin(&mut self.builder.payment_intent),
                "payment_record" => Deserialize::begin(&mut self.builder.payment_record),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(charge), Some(payment_intent), Some(payment_record), Some(type_)) = (
                self.builder.charge.take(),
                self.builder.payment_intent.take(),
                self.builder.payment_record.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(InvoicesPaymentsInvoicePaymentAssociatedPayment {
                charge,
                payment_intent,
                payment_record,
                type_,
            });
            Ok(())
        }
    }
};
/// Type of payment object associated with this invoice payment.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    Charge,
    PaymentIntent,
    PaymentRecord,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    pub fn as_str(&self) -> &str {
        use InvoicesPaymentsInvoicePaymentAssociatedPaymentType::*;
        match self {
            Charge => "charge",
            PaymentIntent => "payment_intent",
            PaymentRecord => "payment_record",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicesPaymentsInvoicePaymentAssociatedPaymentType::*;
        match s {
            "charge" => Ok(Charge),
            "payment_intent" => Ok(PaymentIntent),
            "payment_record" => Ok(PaymentRecord),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "InvoicesPaymentsInvoicePaymentAssociatedPaymentType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(InvoicesPaymentsInvoicePaymentAssociatedPaymentType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<InvoicesPaymentsInvoicePaymentAssociatedPaymentType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InvoicesPaymentsInvoicePaymentAssociatedPaymentType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
