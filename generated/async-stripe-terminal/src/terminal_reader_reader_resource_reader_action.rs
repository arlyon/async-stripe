/// Represents an action performed by the reader
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TerminalReaderReaderResourceReaderAction {
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
#[doc(hidden)]
pub struct TerminalReaderReaderResourceReaderActionBuilder {
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

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: TerminalReaderReaderResourceReaderActionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TerminalReaderReaderResourceReaderActionBuilder {
        type Out = TerminalReaderReaderResourceReaderAction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "failure_code" => Deserialize::begin(&mut self.failure_code),
                "failure_message" => Deserialize::begin(&mut self.failure_message),
                "process_payment_intent" => Deserialize::begin(&mut self.process_payment_intent),
                "process_setup_intent" => Deserialize::begin(&mut self.process_setup_intent),
                "refund_payment" => Deserialize::begin(&mut self.refund_payment),
                "set_reader_display" => Deserialize::begin(&mut self.set_reader_display),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                failure_code: Deserialize::default(),
                failure_message: Deserialize::default(),
                process_payment_intent: Deserialize::default(),
                process_setup_intent: Deserialize::default(),
                refund_payment: Deserialize::default(),
                set_reader_display: Deserialize::default(),
                status: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                failure_code: self.failure_code.take()?,
                failure_message: self.failure_message.take()?,
                process_payment_intent: self.process_payment_intent.take()?,
                process_setup_intent: self.process_setup_intent.take()?,
                refund_payment: self.refund_payment.take()?,
                set_reader_display: self.set_reader_display.take()?,
                status: self.status?,
                type_: self.type_?,
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

    impl ObjectDeser for TerminalReaderReaderResourceReaderAction {
        type Builder = TerminalReaderReaderResourceReaderActionBuilder;
    }

    impl FromValueOpt for TerminalReaderReaderResourceReaderAction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TerminalReaderReaderResourceReaderActionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "failure_code" => b.failure_code = Some(FromValueOpt::from_value(v)?),
                    "failure_message" => b.failure_message = Some(FromValueOpt::from_value(v)?),
                    "process_payment_intent" => {
                        b.process_payment_intent = Some(FromValueOpt::from_value(v)?)
                    }
                    "process_setup_intent" => {
                        b.process_setup_intent = Some(FromValueOpt::from_value(v)?)
                    }
                    "refund_payment" => b.refund_payment = Some(FromValueOpt::from_value(v)?),
                    "set_reader_display" => {
                        b.set_reader_display = Some(FromValueOpt::from_value(v)?)
                    }
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Status of the action performed by the reader.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
}
impl TerminalReaderReaderResourceReaderActionStatus {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceReaderActionStatus::*;
        match self {
            Failed => "failed",
            InProgress => "in_progress",
            Succeeded => "succeeded",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceReaderActionStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceReaderActionStatus::*;
        match s {
            "failed" => Ok(Failed),
            "in_progress" => Ok(InProgress),
            "succeeded" => Ok(Succeeded),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TerminalReaderReaderResourceReaderActionStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceReaderActionStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TerminalReaderReaderResourceReaderActionStatus::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TerminalReaderReaderResourceReaderActionStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceReaderActionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TerminalReaderReaderResourceReaderActionStatus",
            )
        })
    }
}
/// Type of action performed by the reader.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TerminalReaderReaderResourceReaderActionType {
    ProcessPaymentIntent,
    ProcessSetupIntent,
    RefundPayment,
    SetReaderDisplay,
}
impl TerminalReaderReaderResourceReaderActionType {
    pub fn as_str(self) -> &'static str {
        use TerminalReaderReaderResourceReaderActionType::*;
        match self {
            ProcessPaymentIntent => "process_payment_intent",
            ProcessSetupIntent => "process_setup_intent",
            RefundPayment => "refund_payment",
            SetReaderDisplay => "set_reader_display",
        }
    }
}

impl std::str::FromStr for TerminalReaderReaderResourceReaderActionType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TerminalReaderReaderResourceReaderActionType::*;
        match s {
            "process_payment_intent" => Ok(ProcessPaymentIntent),
            "process_setup_intent" => Ok(ProcessSetupIntent),
            "refund_payment" => Ok(RefundPayment),
            "set_reader_display" => Ok(SetReaderDisplay),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for TerminalReaderReaderResourceReaderActionType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TerminalReaderReaderResourceReaderActionType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TerminalReaderReaderResourceReaderActionType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TerminalReaderReaderResourceReaderActionType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TerminalReaderReaderResourceReaderActionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TerminalReaderReaderResourceReaderActionType",
            )
        })
    }
}
