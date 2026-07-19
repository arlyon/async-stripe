/// Represents an action performed by the reader
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceReaderAction {
    pub collect_inputs: Option<stripe_terminal::TerminalReaderReaderResourceCollectInputsAction>,
    pub collect_payment_method:
        Option<stripe_terminal::TerminalReaderReaderResourceCollectPaymentMethodAction>,
    pub confirm_payment_intent:
        Option<stripe_terminal::TerminalReaderReaderResourceConfirmPaymentIntentAction>,
    /// Failure code, only set if status is `failed`.
    pub failure_code: Option<String>,
    /// Detailed failure message, only set if status is `failed`.
    pub failure_message: Option<String>,
    pub process_payment_intent:
        Option<stripe_terminal::TerminalReaderReaderResourceProcessPaymentIntentAction>,
    pub process_setup_intent:
        Option<stripe_terminal::TerminalReaderReaderResourceProcessSetupIntentAction>,
    pub refund_payment: Option<stripe_terminal::TerminalReaderReaderResourceRefundPaymentAction>,
    pub set_reader_display:
        Option<stripe_terminal::TerminalReaderReaderResourceSetReaderDisplayAction>,
    /// Status of the action performed by the reader.
    pub status: TerminalReaderReaderResourceReaderActionStatus,
    /// Type of action performed by the reader.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TerminalReaderReaderResourceReaderActionType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceReaderAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TerminalReaderReaderResourceReaderAction").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TerminalReaderReaderResourceReaderActionBuilder {
    collect_inputs:
        Option<Option<stripe_terminal::TerminalReaderReaderResourceCollectInputsAction>>,
    collect_payment_method:
        Option<Option<stripe_terminal::TerminalReaderReaderResourceCollectPaymentMethodAction>>,
    confirm_payment_intent:
        Option<Option<stripe_terminal::TerminalReaderReaderResourceConfirmPaymentIntentAction>>,
    failure_code: Option<Option<String>>,
    failure_message: Option<Option<String>>,
    process_payment_intent:
        Option<Option<stripe_terminal::TerminalReaderReaderResourceProcessPaymentIntentAction>>,
    process_setup_intent:
        Option<Option<stripe_terminal::TerminalReaderReaderResourceProcessSetupIntentAction>>,
    refund_payment:
        Option<Option<stripe_terminal::TerminalReaderReaderResourceRefundPaymentAction>>,
    set_reader_display:
        Option<Option<stripe_terminal::TerminalReaderReaderResourceSetReaderDisplayAction>>,
    status: Option<TerminalReaderReaderResourceReaderActionStatus>,
    type_: Option<TerminalReaderReaderResourceReaderActionType>,
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

    impl Deserialize for TerminalReaderReaderResourceReaderAction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TerminalReaderReaderResourceReaderAction>,
        builder: TerminalReaderReaderResourceReaderActionBuilder,
    }

    impl Visitor for Place<TerminalReaderReaderResourceReaderAction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TerminalReaderReaderResourceReaderActionBuilder {
                    collect_inputs: Deserialize::default(),
                    collect_payment_method: Deserialize::default(),
                    confirm_payment_intent: Deserialize::default(),
                    failure_code: Deserialize::default(),
                    failure_message: Deserialize::default(),
                    process_payment_intent: Deserialize::default(),
                    process_setup_intent: Deserialize::default(),
                    refund_payment: Deserialize::default(),
                    set_reader_display: Deserialize::default(),
                    status: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "collect_inputs" => Deserialize::begin(&mut self.builder.collect_inputs),
                "collect_payment_method" => {
                    Deserialize::begin(&mut self.builder.collect_payment_method)
                }
                "confirm_payment_intent" => {
                    Deserialize::begin(&mut self.builder.confirm_payment_intent)
                }
                "failure_code" => Deserialize::begin(&mut self.builder.failure_code),
                "failure_message" => Deserialize::begin(&mut self.builder.failure_message),
                "process_payment_intent" => {
                    Deserialize::begin(&mut self.builder.process_payment_intent)
                }
                "process_setup_intent" => {
                    Deserialize::begin(&mut self.builder.process_setup_intent)
                }
                "refund_payment" => Deserialize::begin(&mut self.builder.refund_payment),
                "set_reader_display" => Deserialize::begin(&mut self.builder.set_reader_display),
                "status" => Deserialize::begin(&mut self.builder.status),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(collect_inputs),
                Some(collect_payment_method),
                Some(confirm_payment_intent),
                Some(failure_code),
                Some(failure_message),
                Some(process_payment_intent),
                Some(process_setup_intent),
                Some(refund_payment),
                Some(set_reader_display),
                Some(status),
                Some(type_),
            ) = (
                self.builder.collect_inputs.take(),
                self.builder.collect_payment_method.take(),
                self.builder.confirm_payment_intent.take(),
                self.builder.failure_code.take(),
                self.builder.failure_message.take(),
                self.builder.process_payment_intent.take(),
                self.builder.process_setup_intent.take(),
                self.builder.refund_payment.take(),
                self.builder.set_reader_display.take(),
                self.builder.status.take(),
                self.builder.type_.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(TerminalReaderReaderResourceReaderAction {
                collect_inputs,
                collect_payment_method,
                confirm_payment_intent,
                failure_code,
                failure_message,
                process_payment_intent,
                process_setup_intent,
                refund_payment,
                set_reader_display,
                status,
                type_,
            });
            Ok(())
        }
    }
};
/// Status of the action performed by the reader.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalReaderReaderResourceReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalReaderReaderResourceReaderActionStatus {
    pub fn as_str(&self) -> &str {
        use TerminalReaderReaderResourceReaderActionStatus::*;
        match self {
            Failed => "failed",
            InProgress => "in_progress",
            Succeeded => "succeeded",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceReaderActionStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceReaderActionStatus::*;
        match s {
            "failed" => Ok(Failed),
            "in_progress" => Ok(InProgress),
            "succeeded" => Ok(Succeeded),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TerminalReaderReaderResourceReaderActionStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TerminalReaderReaderResourceReaderActionStatus))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalReaderReaderResourceReaderActionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TerminalReaderReaderResourceReaderActionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceReaderActionStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TerminalReaderReaderResourceReaderActionStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceReaderActionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Type of action performed by the reader.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TerminalReaderReaderResourceReaderActionType {
    CollectInputs,
    CollectPaymentMethod,
    ConfirmPaymentIntent,
    ProcessPaymentIntent,
    ProcessSetupIntent,
    RefundPayment,
    SetReaderDisplay,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TerminalReaderReaderResourceReaderActionType {
    pub fn as_str(&self) -> &str {
        use TerminalReaderReaderResourceReaderActionType::*;
        match self {
            CollectInputs => "collect_inputs",
            CollectPaymentMethod => "collect_payment_method",
            ConfirmPaymentIntent => "confirm_payment_intent",
            ProcessPaymentIntent => "process_payment_intent",
            ProcessSetupIntent => "process_setup_intent",
            RefundPayment => "refund_payment",
            SetReaderDisplay => "set_reader_display",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceReaderActionType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceReaderActionType::*;
        match s {
            "collect_inputs" => Ok(CollectInputs),
            "collect_payment_method" => Ok(CollectPaymentMethod),
            "confirm_payment_intent" => Ok(ConfirmPaymentIntent),
            "process_payment_intent" => Ok(ProcessPaymentIntent),
            "process_setup_intent" => Ok(ProcessSetupIntent),
            "refund_payment" => Ok(RefundPayment),
            "set_reader_display" => Ok(SetReaderDisplay),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TerminalReaderReaderResourceReaderActionType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TerminalReaderReaderResourceReaderActionType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TerminalReaderReaderResourceReaderActionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for TerminalReaderReaderResourceReaderActionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceReaderActionType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(TerminalReaderReaderResourceReaderActionType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceReaderActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
