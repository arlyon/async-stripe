#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLinksResourcePaymentIntentData {
    /// Indicates when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentLinksResourcePaymentIntentDataCaptureMethod>,
    /// An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    pub metadata: std::collections::HashMap<String, String>,
    /// Indicates that you intend to make future payments with the payment method collected during checkout.
    pub setup_future_usage: Option<PaymentLinksResourcePaymentIntentDataSetupFutureUsage>,
    /// For a non-card payment, information about the charge that appears on the customer's statement when this payment succeeds in creating a charge.
    pub statement_descriptor: Option<String>,
    /// For a card payment, information about the charge that appears on the customer's statement when this payment succeeds in creating a charge.
    /// Concatenated with the account's statement descriptor prefix to form the complete statement descriptor.
    pub statement_descriptor_suffix: Option<String>,
    /// A string that identifies the resulting payment as part of a group.
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    pub transfer_group: Option<String>,
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
                builder: PaymentLinksResourcePaymentIntentDataBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinksResourcePaymentIntentDataBuilder {
        type Out = PaymentLinksResourcePaymentIntentData;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_method" => Deserialize::begin(&mut self.capture_method),
                "description" => Deserialize::begin(&mut self.description),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "statement_descriptor_suffix" => {
                    Deserialize::begin(&mut self.statement_descriptor_suffix)
                }
                "transfer_group" => Deserialize::begin(&mut self.transfer_group),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                capture_method: Deserialize::default(),
                description: Deserialize::default(),
                metadata: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                statement_descriptor_suffix: Deserialize::default(),
                transfer_group: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(capture_method),
                Some(description),
                Some(metadata),
                Some(setup_future_usage),
                Some(statement_descriptor),
                Some(statement_descriptor_suffix),
                Some(transfer_group),
            ) = (
                self.capture_method,
                self.description.take(),
                self.metadata.take(),
                self.setup_future_usage,
                self.statement_descriptor.take(),
                self.statement_descriptor_suffix.take(),
                self.transfer_group.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                capture_method,
                description,
                metadata,
                setup_future_usage,
                statement_descriptor,
                statement_descriptor_suffix,
                transfer_group,
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

    impl ObjectDeser for PaymentLinksResourcePaymentIntentData {
        type Builder = PaymentLinksResourcePaymentIntentDataBuilder;
    }

    impl FromValueOpt for PaymentLinksResourcePaymentIntentData {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinksResourcePaymentIntentDataBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "capture_method" => b.capture_method = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),
                    "statement_descriptor_suffix" => {
                        b.statement_descriptor_suffix = FromValueOpt::from_value(v)
                    }
                    "transfer_group" => b.transfer_group = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates when the funds will be captured from the customer's account.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourcePaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}
impl PaymentLinksResourcePaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourcePaymentIntentDataCaptureMethod::*;
        match self {
            Automatic => "automatic",
            AutomaticAsync => "automatic_async",
            Manual => "manual",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourcePaymentIntentDataCaptureMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "automatic_async" => Ok(AutomaticAsync),
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentLinksResourcePaymentIntentDataCaptureMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentLinksResourcePaymentIntentDataCaptureMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinksResourcePaymentIntentDataCaptureMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentLinksResourcePaymentIntentDataCaptureMethod",
            )
        })
    }
}
/// Indicates that you intend to make future payments with the payment method collected during checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}
impl PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentLinksResourcePaymentIntentDataSetupFutureUsage::*;
        match self {
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinksResourcePaymentIntentDataSetupFutureUsage::*;
        match s {
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentLinksResourcePaymentIntentDataSetupFutureUsage>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentLinksResourcePaymentIntentDataSetupFutureUsage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinksResourcePaymentIntentDataSetupFutureUsage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for PaymentLinksResourcePaymentIntentDataSetupFutureUsage",
            )
        })
    }
}
