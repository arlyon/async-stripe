#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet {
    /// The type of mobile wallet, one of `apple_pay`, `google_pay`, `samsung_pay`, or `unknown`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletBuilder {
    type_: Option<PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType>,
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

    impl Deserialize for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet>,
        builder: PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletBuilder,
    }

    impl Visitor for Place<PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletBuilder {
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(type_),) = (self.builder.type_.take(),) else {
                return Ok(());
            };
            *self.out = Some(PaymentFlowsPrivatePaymentMethodsCardPresentCommonWallet { type_ });
            Ok(())
        }
    }
};
/// The type of mobile wallet, one of `apple_pay`, `google_pay`, `samsung_pay`, or `unknown`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    ApplePay,
    GooglePay,
    SamsungPay,
    Unknown,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    /// This variant is prefixed with an underscore to avoid conflicts with Stripe's 'Unknown' variant.
    _Unknown(String),
}
impl PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    pub fn as_str(&self) -> &str {
        use PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::*;
        match self {
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            SamsungPay => "samsung_pay",
            Unknown => "unknown",
            _Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::*;
        match s {
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "samsung_pay" => Ok(SamsungPay),
            "unknown" => Ok(Unknown),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType"
                );
                Ok(_Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor
    for crate::Place<PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType>
{
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentFlowsPrivatePaymentMethodsCardPresentCommonWalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
