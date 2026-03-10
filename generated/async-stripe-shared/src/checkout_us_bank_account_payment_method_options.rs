#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutUsBankAccountPaymentMethodOptions {
    pub financial_connections: Option<stripe_shared::LinkedAccountOptionsCommon>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage>,
    /// Controls when Stripe will attempt to debit the funds from the customer's account.
    /// The date must be a string in YYYY-MM-DD format.
    /// The date must be in the future and between 3 and 15 calendar days from now.
    pub target_date: Option<String>,
    /// Bank account verification method.
    pub verification_method: Option<CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod>,
}
#[doc(hidden)]
pub struct CheckoutUsBankAccountPaymentMethodOptionsBuilder {
    financial_connections: Option<Option<stripe_shared::LinkedAccountOptionsCommon>>,
    setup_future_usage: Option<Option<CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage>>,
    target_date: Option<Option<String>>,
    verification_method:
        Option<Option<CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod>>,
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

    impl Deserialize for CheckoutUsBankAccountPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutUsBankAccountPaymentMethodOptions>,
        builder: CheckoutUsBankAccountPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutUsBankAccountPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutUsBankAccountPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutUsBankAccountPaymentMethodOptionsBuilder {
        type Out = CheckoutUsBankAccountPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "financial_connections" => Deserialize::begin(&mut self.financial_connections),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                "target_date" => Deserialize::begin(&mut self.target_date),
                "verification_method" => Deserialize::begin(&mut self.verification_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                financial_connections: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
                target_date: Deserialize::default(),
                verification_method: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(financial_connections),
                Some(setup_future_usage),
                Some(target_date),
                Some(verification_method),
            ) = (
                self.financial_connections.take(),
                self.setup_future_usage.take(),
                self.target_date.take(),
                self.verification_method.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                financial_connections,
                setup_future_usage,
                target_date,
                verification_method,
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

    impl ObjectDeser for CheckoutUsBankAccountPaymentMethodOptions {
        type Builder = CheckoutUsBankAccountPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutUsBankAccountPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutUsBankAccountPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "financial_connections" => {
                        b.financial_connections = FromValueOpt::from_value(v)
                    }
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
pub enum CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    None,
    OffSession,
    OnSession,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(&self) -> &str {
        use CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            OnSession => "on_session",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            "on_session" => Ok(OnSession),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutUsBankAccountPaymentMethodOptionsSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Bank account verification method.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    Automatic,
    Instant,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    pub fn as_str(&self) -> &str {
        use CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::*;
        match self {
            Automatic => "automatic",
            Instant => "instant",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::*;
        match s {
            "automatic" => Ok(Automatic),
            "instant" => Ok(Instant),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CheckoutUsBankAccountPaymentMethodOptionsVerificationMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
