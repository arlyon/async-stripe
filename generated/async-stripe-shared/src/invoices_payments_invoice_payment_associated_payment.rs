#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct InvoicesPaymentsInvoicePaymentAssociatedPayment {
    /// ID of the successful charge for this payment when `type` is `charge`.Note: charge is only surfaced if the charge object is not associated with a payment intent.
    /// If the charge object does have a payment intent, the Invoice Payment surfaces the payment intent instead.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// ID of the PaymentIntent associated with this payment when `type` is `payment_intent`.
    /// Note: This property is only populated for invoices finalized on or after March 15th, 2019.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    /// Type of payment object associated with this invoice payment.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: InvoicesPaymentsInvoicePaymentAssociatedPaymentType,
}
#[doc(hidden)]
pub struct InvoicesPaymentsInvoicePaymentAssociatedPaymentBuilder {
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    type_: Option<InvoicesPaymentsInvoicePaymentAssociatedPaymentType>,
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
                builder: InvoicesPaymentsInvoicePaymentAssociatedPaymentBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for InvoicesPaymentsInvoicePaymentAssociatedPaymentBuilder {
        type Out = InvoicesPaymentsInvoicePaymentAssociatedPayment;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "charge" => Deserialize::begin(&mut self.charge),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                charge: Deserialize::default(),
                payment_intent: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(charge), Some(payment_intent), Some(type_)) =
                (self.charge.take(), self.payment_intent.take(), self.type_)
            else {
                return None;
            };
            Some(Self::Out { charge, payment_intent, type_ })
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

    impl ObjectDeser for InvoicesPaymentsInvoicePaymentAssociatedPayment {
        type Builder = InvoicesPaymentsInvoicePaymentAssociatedPaymentBuilder;
    }

    impl FromValueOpt for InvoicesPaymentsInvoicePaymentAssociatedPayment {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoicesPaymentsInvoicePaymentAssociatedPaymentBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "charge" => b.charge = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of payment object associated with this invoice payment.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    Charge,
    PaymentIntent,
}
impl InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    pub fn as_str(self) -> &'static str {
        use InvoicesPaymentsInvoicePaymentAssociatedPaymentType::*;
        match self {
            Charge => "charge",
            PaymentIntent => "payment_intent",
        }
    }
}

impl std::str::FromStr for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoicesPaymentsInvoicePaymentAssociatedPaymentType::*;
        match s {
            "charge" => Ok(Charge),
            "payment_intent" => Ok(PaymentIntent),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoicesPaymentsInvoicePaymentAssociatedPaymentType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            InvoicesPaymentsInvoicePaymentAssociatedPaymentType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoicesPaymentsInvoicePaymentAssociatedPaymentType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for InvoicesPaymentsInvoicePaymentAssociatedPaymentType",
            )
        })
    }
}
