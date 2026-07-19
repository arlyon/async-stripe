#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourcePaymentIntentData {
    /// Indicates when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentLinksResourcePaymentIntentDataCaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that will set metadata on [Payment Intents](https://docs.stripe.com/api/payment_intents) generated from this payment link.
    pub metadata: std::collections::HashMap<String, String>,
    /// Indicates that you intend to make future payments with the payment method collected during checkout.
    pub setup_future_usage: Option<PaymentLinksResourcePaymentIntentDataSetupFutureUsage>,
    /// For a non-card payment, information about the charge that appears on the customer's statement when this payment succeeds in creating a charge.
    pub statement_descriptor: Option<String>,
    /// For a card payment, information about the charge that appears on the customer's statement when this payment succeeds in creating a charge.
    /// Concatenated with the account's statement descriptor prefix to form the complete statement descriptor.
    pub statement_descriptor_suffix: Option<String>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://docs.stripe.com/connect/separate-charges-and-transfers) for details.
    pub transfer_group: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourcePaymentIntentData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentLinksResourcePaymentIntentData").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentLinksResourcePaymentIntentDataBuilder {
    capture_method: Option<Option<PaymentLinksResourcePaymentIntentDataCaptureMethod>>,
    description: Option<Option<String>>,
    metadata: Option<std::collections::HashMap<String, String>>,
    setup_future_usage: Option<Option<PaymentLinksResourcePaymentIntentDataSetupFutureUsage>>,
    statement_descriptor: Option<Option<String>>,
    statement_descriptor_suffix: Option<Option<String>>,
    transfer_group: Option<Option<String>>,
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

    impl Deserialize for PaymentLinksResourcePaymentIntentData {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLinksResourcePaymentIntentData>,
        builder: PaymentLinksResourcePaymentIntentDataBuilder,
    }

    impl Visitor for Place<PaymentLinksResourcePaymentIntentData> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinksResourcePaymentIntentDataBuilder {
                    capture_method: Deserialize::default(),
                    description: Deserialize::default(),
                    metadata: Deserialize::default(),
                    setup_future_usage: Deserialize::default(),
                    statement_descriptor: Deserialize::default(),
                    statement_descriptor_suffix: Deserialize::default(),
                    transfer_group: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_method" => Deserialize::begin(&mut self.builder.capture_method),
                "description" => Deserialize::begin(&mut self.builder.description),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "setup_future_usage" => Deserialize::begin(&mut self.builder.setup_future_usage),
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                "statement_descriptor_suffix" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor_suffix)
                }
                "transfer_group" => Deserialize::begin(&mut self.builder.transfer_group),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(capture_method),
                Some(description),
                Some(metadata),
                Some(setup_future_usage),
                Some(statement_descriptor),
                Some(statement_descriptor_suffix),
                Some(transfer_group),
            ) = (
                self.builder.capture_method.take(),
                self.builder.description.take(),
                self.builder.metadata.take(),
                self.builder.setup_future_usage.take(),
                self.builder.statement_descriptor.take(),
                self.builder.statement_descriptor_suffix.take(),
                self.builder.transfer_group.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentLinksResourcePaymentIntentData {
                capture_method,
                description,
                metadata,
                setup_future_usage,
                statement_descriptor,
                statement_descriptor_suffix,
                transfer_group,
            });
            Ok(())
        }
    }
};
/// Indicates when the funds will be captured from the customer's account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinksResourcePaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinksResourcePaymentIntentDataCaptureMethod {
    pub fn as_str(&self) -> &str {
        use PaymentLinksResourcePaymentIntentDataCaptureMethod::*;
        match self {
            Automatic => "automatic",
            AutomaticAsync => "automatic_async",
            Manual => "manual",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourcePaymentIntentDataCaptureMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "automatic_async" => Ok(AutomaticAsync),
            "manual" => Ok(Manual),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinksResourcePaymentIntentDataCaptureMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentLinksResourcePaymentIntentDataCaptureMethod))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentLinksResourcePaymentIntentDataCaptureMethod> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentLinksResourcePaymentIntentDataCaptureMethod::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Indicates that you intend to make future payments with the payment method collected during checkout.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    pub fn as_str(&self) -> &str {
        use PaymentLinksResourcePaymentIntentDataSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourcePaymentIntentDataSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinksResourcePaymentIntentDataSetupFutureUsage"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentLinksResourcePaymentIntentDataSetupFutureUsage))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentLinksResourcePaymentIntentDataSetupFutureUsage>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentLinksResourcePaymentIntentDataSetupFutureUsage::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
