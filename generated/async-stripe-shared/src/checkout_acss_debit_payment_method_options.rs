#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutAcssDebitPaymentMethodOptions {
    /// Currency supported by the bank account. Returned when the Session is in `setup` mode.
    pub currency: Option<CheckoutAcssDebitPaymentMethodOptionsCurrency>,
    pub mandate_options: Option<stripe_shared::CheckoutAcssDebitMandateOptions>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    pub target_date: Option<String>,
    /// Bank account verification method.
    pub verification_method: Option<CheckoutAcssDebitPaymentMethodOptionsVerificationMethod>,
}
#[doc(hidden)]
pub struct CheckoutAcssDebitPaymentMethodOptionsBuilder {
    currency: Option<Option<CheckoutAcssDebitPaymentMethodOptionsCurrency>>,
    mandate_options: Option<Option<stripe_shared::CheckoutAcssDebitMandateOptions>>,
    setup_future_usage: Option<Option<CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>>,
    target_date: Option<Option<String>>,
    verification_method: Option<Option<CheckoutAcssDebitPaymentMethodOptionsVerificationMethod>>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CheckoutAcssDebitPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutAcssDebitPaymentMethodOptions>,
        builder: CheckoutAcssDebitPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutAcssDebitPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutAcssDebitPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutAcssDebitPaymentMethodOptionsBuilder {
        type Out = CheckoutAcssDebitPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "currency" => Deserialize::begin(&mut self.currency),
                "mandate_options" => Deserialize::begin(&mut self.mandate_options),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                "target_date" => Deserialize::begin(&mut self.target_date),
                "verification_method" => Deserialize::begin(&mut self.verification_method),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                currency: Deserialize::default(),
                mandate_options: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
                target_date: Deserialize::default(),
                verification_method: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(currency),
                Some(mandate_options),
                Some(setup_future_usage),
                Some(target_date),
                Some(verification_method),
            ) = (
                self.currency,
                self.mandate_options.take(),
                self.setup_future_usage,
                self.target_date.take(),
                self.verification_method,
            )
            else {
                return None;
            };
            Some(Self::Out {
                currency,
                mandate_options,
                setup_future_usage,
                target_date,
                verification_method,
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

    impl ObjectDeser for CheckoutAcssDebitPaymentMethodOptions {
        type Builder = CheckoutAcssDebitPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutAcssDebitPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutAcssDebitPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "mandate_options" => b.mandate_options = FromValueOpt::from_value(v),
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    "target_date" => b.target_date = FromValueOpt::from_value(v),
                    "verification_method" => b.verification_method = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Currency supported by the bank account. Returned when the Session is in `setup` mode.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutAcssDebitPaymentMethodOptionsCurrency {
    Cad,
    Usd,
}
impl CheckoutAcssDebitPaymentMethodOptionsCurrency {
    pub fn as_str(self) -> &'static str {
        use CheckoutAcssDebitPaymentMethodOptionsCurrency::*;
        match self {
            Cad => "cad",
            Usd => "usd",
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitPaymentMethodOptionsCurrency::*;
        match s {
            "cad" => Ok(Cad),
            "usd" => Ok(Usd),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CheckoutAcssDebitPaymentMethodOptionsCurrency> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutAcssDebitPaymentMethodOptionsCurrency::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutAcssDebitPaymentMethodOptionsCurrency);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitPaymentMethodOptionsCurrency {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutAcssDebitPaymentMethodOptionsCurrency",
            )
        })
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
pub enum CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}
impl CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        use CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutAcssDebitPaymentMethodOptionsSetupFutureUsage",
            )
        })
    }
}
/// Bank account verification method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}
impl CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    pub fn as_str(self) -> &'static str {
        use CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Microdeposits => "microdeposits",
        }
    }
}

impl std::str::FromStr for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            "microdeposits" => Ok(Microdeposits),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutAcssDebitPaymentMethodOptionsVerificationMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutAcssDebitPaymentMethodOptionsVerificationMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CheckoutAcssDebitPaymentMethodOptionsVerificationMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CheckoutAcssDebitPaymentMethodOptionsVerificationMethod",
            )
        })
    }
}
