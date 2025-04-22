#[derive(Copy, Clone, Debug)]
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
#[doc(hidden)]
pub struct SetupAttemptPaymentMethodDetailsCardWalletBuilder {
    apple_pay: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletApplePay>>,
    google_pay: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletGooglePay>>,
    type_: Option<SetupAttemptPaymentMethodDetailsCardWalletType>,
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
                builder: SetupAttemptPaymentMethodDetailsCardWalletBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SetupAttemptPaymentMethodDetailsCardWalletBuilder {
        type Out = SetupAttemptPaymentMethodDetailsCardWallet;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "apple_pay" => Deserialize::begin(&mut self.apple_pay),
                "google_pay" => Deserialize::begin(&mut self.google_pay),
                "type" => Deserialize::begin(&mut self.type_),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                apple_pay: Deserialize::default(),
                google_pay: Deserialize::default(),
                type_: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(apple_pay), Some(google_pay), Some(type_)) =
                (self.apple_pay, self.google_pay, self.type_)
            else {
                return None;
            };
            Some(Self::Out { apple_pay, google_pay, type_ })
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

    impl ObjectDeser for SetupAttemptPaymentMethodDetailsCardWallet {
        type Builder = SetupAttemptPaymentMethodDetailsCardWalletBuilder;
    }

    impl FromValueOpt for SetupAttemptPaymentMethodDetailsCardWallet {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SetupAttemptPaymentMethodDetailsCardWalletBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "apple_pay" => b.apple_pay = FromValueOpt::from_value(v),
                    "google_pay" => b.google_pay = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the card wallet, one of `apple_pay`, `google_pay`, or `link`.
/// An additional hash is included on the Wallet subhash with a name matching this value.
/// It contains additional information specific to the card wallet type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SetupAttemptPaymentMethodDetailsCardWalletType {
    ApplePay,
    GooglePay,
    Link,
}
impl SetupAttemptPaymentMethodDetailsCardWalletType {
    pub fn as_str(self) -> &'static str {
        use SetupAttemptPaymentMethodDetailsCardWalletType::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            Link => "link",
        }
    }
}

impl std::str::FromStr for SetupAttemptPaymentMethodDetailsCardWalletType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SetupAttemptPaymentMethodDetailsCardWalletType::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "link" => Ok(Link),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SetupAttemptPaymentMethodDetailsCardWalletType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SetupAttemptPaymentMethodDetailsCardWalletType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SetupAttemptPaymentMethodDetailsCardWalletType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SetupAttemptPaymentMethodDetailsCardWalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SetupAttemptPaymentMethodDetailsCardWalletType",
            )
        })
    }
}
