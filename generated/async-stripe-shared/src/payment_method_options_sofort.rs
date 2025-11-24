#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodOptionsSofort {
    /// Preferred language of the SOFORT authorization page that the customer is redirected to.
    pub preferred_language: Option<PaymentMethodOptionsSofortPreferredLanguage>,
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// If you provide a Customer with the PaymentIntent, you can use this parameter to [attach the payment method](/payments/save-during-payment) to the Customer after the PaymentIntent is confirmed and the customer completes any required actions.
    /// If you don't provide a Customer, you can still [attach](/api/payment_methods/attach) the payment method to a Customer after the transaction completes.
    ///
    /// If the payment method is `card_present` and isn't a digital wallet, Stripe creates and attaches a [generated_card](/api/charges/object#charge_object-payment_method_details-card_present-generated_card) payment method representing the card to the Customer instead.
    ///
    /// When processing card payments, Stripe uses `setup_future_usage` to help you comply with regional legislation and network rules, such as [SCA](/strong-customer-authentication).
    pub setup_future_usage: Option<PaymentMethodOptionsSofortSetupFutureUsage>,
}
#[doc(hidden)]
pub struct PaymentMethodOptionsSofortBuilder {
    preferred_language: Option<Option<PaymentMethodOptionsSofortPreferredLanguage>>,
    setup_future_usage: Option<Option<PaymentMethodOptionsSofortSetupFutureUsage>>,
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

    impl Deserialize for PaymentMethodOptionsSofort {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodOptionsSofort>,
        builder: PaymentMethodOptionsSofortBuilder,
    }

    impl Visitor for Place<PaymentMethodOptionsSofort> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodOptionsSofortBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodOptionsSofortBuilder {
        type Out = PaymentMethodOptionsSofort;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "preferred_language" => Deserialize::begin(&mut self.preferred_language),
                "setup_future_usage" => Deserialize::begin(&mut self.setup_future_usage),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                preferred_language: Deserialize::default(),
                setup_future_usage: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(preferred_language), Some(setup_future_usage)) =
                (self.preferred_language.take(), self.setup_future_usage.take())
            else {
                return None;
            };
            Some(Self::Out { preferred_language, setup_future_usage })
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

    impl ObjectDeser for PaymentMethodOptionsSofort {
        type Builder = PaymentMethodOptionsSofortBuilder;
    }

    impl FromValueOpt for PaymentMethodOptionsSofort {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodOptionsSofortBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "preferred_language" => b.preferred_language = FromValueOpt::from_value(v),
                    "setup_future_usage" => b.setup_future_usage = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Preferred language of the SOFORT authorization page that the customer is redirected to.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodOptionsSofortPreferredLanguage {
    pub fn as_str(&self) -> &str {
        use PaymentMethodOptionsSofortPreferredLanguage::*;
        match self {
            De => "de",
            En => "en",
            Es => "es",
            Fr => "fr",
            It => "it",
            Nl => "nl",
            Pl => "pl",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsSofortPreferredLanguage {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsSofortPreferredLanguage::*;
        match s {
            "de" => Ok(De),
            "en" => Ok(En),
            "es" => Ok(Es),
            "fr" => Ok(Fr),
            "it" => Ok(It),
            "nl" => Ok(Nl),
            "pl" => Ok(Pl),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodOptionsSofortPreferredLanguage"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsSofortPreferredLanguage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsSofortPreferredLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsSofortPreferredLanguage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsSofortPreferredLanguage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodOptionsSofortPreferredLanguage::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsSofortPreferredLanguage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsSofortPreferredLanguage {
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
pub enum PaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(&self) -> &str {
        use PaymentMethodOptionsSofortSetupFutureUsage::*;
        match self {
            None => "none",
            OffSession => "off_session",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodOptionsSofortSetupFutureUsage {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodOptionsSofortSetupFutureUsage::*;
        match s {
            "none" => Ok(None),
            "off_session" => Ok(OffSession),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodOptionsSofortSetupFutureUsage"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodOptionsSofortSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodOptionsSofortSetupFutureUsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentMethodOptionsSofortSetupFutureUsage {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodOptionsSofortSetupFutureUsage> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodOptionsSofortSetupFutureUsage::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodOptionsSofortSetupFutureUsage);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodOptionsSofortSetupFutureUsage {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
