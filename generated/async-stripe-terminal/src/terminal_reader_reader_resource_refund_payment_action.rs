/// Represents a reader action to refund a payment
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceRefundPaymentAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceRefundPaymentAction").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: TerminalReaderReaderResourceRefundPaymentActionBuilder {
                    amount: Deserialize::default(),
                    charge: Deserialize::default(),
                    metadata: Deserialize::default(),
                    payment_intent: Deserialize::default(),
                    reason: Deserialize::default(),
                    refund: Deserialize::default(),
                    refund_application_fee: Deserialize::default(),
                    refund_payment_config: Deserialize::default(),
                    reverse_transfer: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "charge" => Deserialize::begin(&mut self.builder.charge),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "payment_intent" => Deserialize::begin(&mut self.builder.payment_intent),
                "reason" => Deserialize::begin(&mut self.builder.reason),
                "refund" => Deserialize::begin(&mut self.builder.refund),
                "refund_application_fee" => {
                    Deserialize::begin(&mut self.builder.refund_application_fee)
                }
                "refund_payment_config" => {
                    Deserialize::begin(&mut self.builder.refund_payment_config)
                }
                "reverse_transfer" => Deserialize::begin(&mut self.builder.reverse_transfer),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                self.builder.amount,
                self.builder.charge.take(),
                self.builder.metadata.take(),
                self.builder.payment_intent.take(),
                self.builder.reason.take(),
                self.builder.refund.take(),
                self.builder.refund_application_fee,
                self.builder.refund_payment_config,
                self.builder.reverse_transfer,
            )
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceRefundPaymentAction {
                amount,
                charge,
                metadata,
                payment_intent,
                reason,
                refund,
                refund_application_fee,
                refund_payment_config,
                reverse_transfer,
            });
            Ok(())
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

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TerminalReaderReaderResourceRefundPaymentActionReason))
            .finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<TerminalReaderReaderResourceRefundPaymentActionReason>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TerminalReaderReaderResourceRefundPaymentActionReason::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceRefundPaymentActionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
