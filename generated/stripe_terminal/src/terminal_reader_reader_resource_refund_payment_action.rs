/// Represents a reader action to refund a payment
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceRefundPaymentAction {
    /// The amount being refunded.
    pub amount: Option<i64>,
    /// Charge that is being refunded.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Payment intent that is being refunded.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    /// The reason for the refund.
    pub reason: Option<TerminalReaderReaderResourceRefundPaymentActionReason>,
    /// Unique identifier for the refund object.
    pub refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
    /// Boolean indicating whether the application fee should be refunded when refunding this charge.
    /// If a full charge refund is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded in an amount proportional to the amount of the charge refunded.
    /// An application fee can be refunded only by the application that created the charge.
    pub refund_application_fee: Option<bool>,
    /// Boolean indicating whether the transfer should be reversed when refunding this charge.
    /// The transfer will be reversed proportionally to the amount being refunded (either the entire or partial amount).
    /// A transfer can be reversed only by the application that created the charge.
    pub reverse_transfer: Option<bool>,
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceRefundPaymentActionBuilder {
    amount: Option<Option<i64>>,
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    reason: Option<Option<TerminalReaderReaderResourceRefundPaymentActionReason>>,
    refund: Option<Option<stripe_types::Expandable<stripe_shared::Refund>>>,
    refund_application_fee: Option<Option<bool>>,
    reverse_transfer: Option<Option<bool>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for TerminalReaderReaderResourceRefundPaymentAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceRefundPaymentAction>,
        builder: TerminalReaderReaderResourceRefundPaymentActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceRefundPaymentAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceRefundPaymentActionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceRefundPaymentActionBuilder {
        type Out = TerminalReaderReaderResourceRefundPaymentAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "charge" => Deserialize::begin(&mut self.charge),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "reason" => Deserialize::begin(&mut self.reason),
                "refund" => Deserialize::begin(&mut self.refund),
                "refund_application_fee" => Deserialize::begin(&mut self.refund_application_fee),
                "reverse_transfer" => Deserialize::begin(&mut self.reverse_transfer),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                charge: Deserialize::default(),
                metadata: Deserialize::default(),
                payment_intent: Deserialize::default(),
                reason: Deserialize::default(),
                refund: Deserialize::default(),
                refund_application_fee: Deserialize::default(),
                reverse_transfer: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                charge: self.charge.take()?,
                metadata: self.metadata.take()?,
                payment_intent: self.payment_intent.take()?,
                reason: self.reason?,
                refund: self.refund.take()?,
                refund_application_fee: self.refund_application_fee?,
                reverse_transfer: self.reverse_transfer?,
            })
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

    impl ObjectDeser for TerminalReaderReaderResourceRefundPaymentAction {
        type Builder = TerminalReaderReaderResourceRefundPaymentActionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceRefundPaymentAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceRefundPaymentActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "charge" => b.charge = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "payment_intent" => b.payment_intent = Some(FromValueOpt::from_value(v)?),
                    "reason" => b.reason = Some(FromValueOpt::from_value(v)?),
                    "refund" => b.refund = Some(FromValueOpt::from_value(v)?),
                    "refund_application_fee" => {
                        b.refund_application_fee = Some(FromValueOpt::from_value(v)?)
                    }
                    "reverse_transfer" => b.reverse_transfer = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The reason for the refund.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceRefundPaymentActionReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
}
impl TerminalReaderReaderResourceRefundPaymentActionReason {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceRefundPaymentActionReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            RequestedByCustomer => "requested_by_customer",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceRefundPaymentActionReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceRefundPaymentActionReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "requested_by_customer" => Ok(RequestedByCustomer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TerminalReaderReaderResourceRefundPaymentActionReason>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TerminalReaderReaderResourceRefundPaymentActionReason::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TerminalReaderReaderResourceRefundPaymentActionReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TerminalReaderReaderResourceRefundPaymentActionReason",
            )
        })
    }
}
