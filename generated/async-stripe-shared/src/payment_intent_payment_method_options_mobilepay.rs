#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptionsMobilepay {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage>,
}
#[doc(hidden)]
pub struct PaymentIntentPaymentMethodOptionsMobilepayBuilder {
    capture_method: Option<Option<PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod>>,
    setup_future_usage: Option<Option<PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage>>,
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

    impl Deserialize for PaymentIntentPaymentMethodOptionsMobilepay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentPaymentMethodOptionsMobilepay>,
        builder: PaymentIntentPaymentMethodOptionsMobilepayBuilder,
    }

    impl Visitor for Place<PaymentIntentPaymentMethodOptionsMobilepay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentPaymentMethodOptionsMobilepayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentPaymentMethodOptionsMobilepayBuilder {
        type Out = PaymentIntentPaymentMethodOptionsMobilepay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "capture_method" => Deserialize::begin(&mut self.capture_method),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                capture_method: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(capture_method), Some(setup_future_usage)) =
                (self.capture_method.take(), self.setup_future_usage.take())
            else {
                return None;
            };
            Some(Self::Out { capture_method, setup_future_usage })
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

    impl ObjectDeser for PaymentIntentPaymentMethodOptionsMobilepay {
        type Builder = PaymentIntentPaymentMethodOptionsMobilepayBuilder;
    }

    impl FromValueOpt for PaymentIntentPaymentMethodOptionsMobilepay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentPaymentMethodOptionsMobilepayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "capture_method" => b.capture_method = FromValueOpt::from_value(v),
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Controls when the funds will be captured from the customer's account.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod {
    Manual,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod {
    pub fn as_str(&self) -> &str {
        use PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod::*;
        match self {
            Manual => "manual",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsMobilepayCaptureMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Indicates that you intend to make future payments with this PaymentIntent's payment method.
///
/// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
/// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
///
/// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
///
/// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage {
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage {
    pub fn as_str(&self) -> &str {
        use PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage::*;
        match self {
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentIntentPaymentMethodOptionsMobilepaySetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
