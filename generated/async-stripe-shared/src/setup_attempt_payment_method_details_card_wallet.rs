#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SetupAttemptPaymentMethodDetailsCardWallet {
    pub apple_pay: Option<stripe_shared::PaymentMethodDetailsCardWalletApplePay>,
    pub google_pay: Option<stripe_shared::PaymentMethodDetailsCardWalletGooglePay>,
    /// The type of the card wallet, one of `apple_pay`, `google_pay`, or `link`.
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: SetupAttemptPaymentMethodDetailsCardWalletType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupAttemptPaymentMethodDetailsCardWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SetupAttemptPaymentMethodDetailsCardWallet").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SetupAttemptPaymentMethodDetailsCardWalletBuilder {
    apple_pay: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletApplePay>>,
    google_pay: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletGooglePay>>,
    type_: Option<SetupAttemptPaymentMethodDetailsCardWalletType>,
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

    impl Deserialize for SetupAttemptPaymentMethodDetailsCardWallet {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SetupAttemptPaymentMethodDetailsCardWallet>,
        builder: SetupAttemptPaymentMethodDetailsCardWalletBuilder,
    }

    impl Visitor for Place<SetupAttemptPaymentMethodDetailsCardWallet> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SetupAttemptPaymentMethodDetailsCardWalletBuilder {
                    apple_pay: Deserialize::default(),
                    google_pay: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "apple_pay" => Deserialize::begin(&mut self.builder.apple_pay),
                "google_pay" => Deserialize::begin(&mut self.builder.google_pay),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(apple_pay), Some(google_pay), Some(type_)) =
                (self.builder.apple_pay, self.builder.google_pay, self.builder.type_.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(SetupAttemptPaymentMethodDetailsCardWallet { apple_pay, google_pay, type_ });
            Ok(())
        }
    }
};
/// The type of the card wallet, one of `apple_pay`, `google_pay`, or `link`.
/// An additional hash is included on the Wallet subhash with a name matching this value.
/// It contains additional information specific to the card wallet type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SetupAttemptPaymentMethodDetailsCardWalletType {
    ApplePay,
    GooglePay,
    Link,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SetupAttemptPaymentMethodDetailsCardWalletType {
    pub fn as_str(&self) -> &str {
        use SetupAttemptPaymentMethodDetailsCardWalletType::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            Link => "link",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SetupAttemptPaymentMethodDetailsCardWalletType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupAttemptPaymentMethodDetailsCardWalletType::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "link" => Ok(Link),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "SetupAttemptPaymentMethodDetailsCardWalletType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(SetupAttemptPaymentMethodDetailsCardWalletType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<SetupAttemptPaymentMethodDetailsCardWalletType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(SetupAttemptPaymentMethodDetailsCardWalletType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
