/// Represents a reader action to refund a payment
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceRefundPaymentAction {
    /// The amount being refunded.
    pub amount: Option<i64>,
    /// Charge that is being refunded.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
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
    pub refund_payment_config:
        Option<stripe_terminal::TerminalReaderReaderResourceRefundPaymentConfig>,
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
    refund_payment_config:
        Option<Option<stripe_terminal::TerminalReaderReaderResourceRefundPaymentConfig>>,
    reverse_transfer: Option<Option<bool>>,
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
                "refund_payment_config" => Deserialize::begin(&mut self.refund_payment_config),
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
                refund_payment_config: Deserialize::default(),
                reverse_transfer: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(amount),
                Some(charge),
                Some(metadata),
                Some(payment_intent),
                Some(reason),
                Some(refund),
                Some(refund_application_fee),
                Some(refund_payment_config),
                Some(reverse_transfer),
            ) = (
                self.amount,
                self.charge.take(),
                self.metadata.take(),
                self.payment_intent.take(),
                self.reason.take(),
                self.refund.take(),
                self.refund_application_fee,
                self.refund_payment_config,
                self.reverse_transfer,
            )
            else {
                return None;
            };
            Some(Self::Out {
                amount,
                charge,
                metadata,
                payment_intent,
                reason,
                refund,
                refund_application_fee,
                refund_payment_config,
                reverse_transfer,
            })
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
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "charge" => b.charge = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "payment_intent" => b.payment_intent = FromValueOpt::from_value(v),
                    "reason" => b.reason = FromValueOpt::from_value(v),
                    "refund" => b.refund = FromValueOpt::from_value(v),
                    "refund_application_fee" => {
                        b.refund_application_fee = FromValueOpt::from_value(v)
                    }
                    "refund_payment_config" => {
                        b.refund_payment_config = FromValueOpt::from_value(v)
                    }
                    "reverse_transfer" => b.reverse_transfer = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The reason for the refund.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalReaderReaderResourceRefundPaymentActionReason {
    Duplicate,
    Fraudulent,
    RequestedByCustomer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalReaderReaderResourceRefundPaymentActionReason {
    pub fn as_str(&self) -> &str {
        use TerminalReaderReaderResourceRefundPaymentActionReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            RequestedByCustomer => "requested_by_customer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceRefundPaymentActionReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceRefundPaymentActionReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "requested_by_customer" => Ok(RequestedByCustomer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TerminalReaderReaderResourceRefundPaymentActionReason"
                );
                Ok(Unknown(v.to_owned()))
            }
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
            TerminalReaderReaderResourceRefundPaymentActionReason::from_str(s).expect("infallible"),
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
