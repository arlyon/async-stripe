#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions {
    /// Controls when the funds will be captured from the customer's account.
    pub capture_method:
        Option<PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage:
        Option<PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage>,
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsBuilder {
    capture_method:
        Option<Option<PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod>>,
    setup_future_usage: Option<
        Option<PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage>,
    >,
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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions>,
        builder: PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsBuilder {
        type Out = PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions;
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
                (self.capture_method, self.setup_future_usage)
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

    impl ObjectDeser for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions {
        type Builder = PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsBuilder::deser_default(
                );
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod {
    Manual,
}
impl PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod {
    pub fn as_str(self) -> &'static str {
        use PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod::*;
        match self {
            Manual => "manual",
        }
    }
}

impl std::str::FromStr
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod::*;
        match s {
            "manual" => Ok(Manual),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsCaptureMethod"))
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
}
impl PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
        }
    }
}

impl std::str::FromStr
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage
{
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage
{
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de>
    for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptionsSetupFutureUsage"))
    }
}
