#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetailsCardWallet {
    pub amex_express_checkout:
        Option<stripe_shared::PaymentMethodDetailsCardWalletAmexExpressCheckout>,
    pub apple_pay: Option<stripe_shared::PaymentMethodDetailsCardWalletApplePay>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    pub google_pay: Option<stripe_shared::PaymentMethodDetailsCardWalletGooglePay>,
    pub link: Option<stripe_shared::PaymentMethodDetailsCardWalletLink>,
    pub masterpass: Option<stripe_shared::PaymentMethodDetailsCardWalletMasterpass>,
    pub samsung_pay: Option<stripe_shared::PaymentMethodDetailsCardWalletSamsungPay>,
    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: PaymentMethodDetailsCardWalletType,
    pub visa_checkout: Option<stripe_shared::PaymentMethodDetailsCardWalletVisaCheckout>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCardWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetailsCardWallet").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsCardWalletBuilder {
    amex_express_checkout:
        Option<Option<stripe_shared::PaymentMethodDetailsCardWalletAmexExpressCheckout>>,
    apple_pay: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletApplePay>>,
    dynamic_last4: Option<Option<String>>,
    google_pay: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletGooglePay>>,
    link: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletLink>>,
    masterpass: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletMasterpass>>,
    samsung_pay: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletSamsungPay>>,
    type_: Option<PaymentMethodDetailsCardWalletType>,
    visa_checkout: Option<Option<stripe_shared::PaymentMethodDetailsCardWalletVisaCheckout>>,
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

    impl Deserialize for PaymentMethodDetailsCardWallet {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetailsCardWallet>,
        builder: PaymentMethodDetailsCardWalletBuilder,
    }

    impl Visitor for Place<PaymentMethodDetailsCardWallet> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsCardWalletBuilder {
                    amex_express_checkout: Deserialize::default(),
                    apple_pay: Deserialize::default(),
                    dynamic_last4: Deserialize::default(),
                    google_pay: Deserialize::default(),
                    link: Deserialize::default(),
                    masterpass: Deserialize::default(),
                    samsung_pay: Deserialize::default(),
                    type_: Deserialize::default(),
                    visa_checkout: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amex_express_checkout" => {
                    Deserialize::begin(&mut self.builder.amex_express_checkout)
                }
                "apple_pay" => Deserialize::begin(&mut self.builder.apple_pay),
                "dynamic_last4" => Deserialize::begin(&mut self.builder.dynamic_last4),
                "google_pay" => Deserialize::begin(&mut self.builder.google_pay),
                "link" => Deserialize::begin(&mut self.builder.link),
                "masterpass" => Deserialize::begin(&mut self.builder.masterpass),
                "samsung_pay" => Deserialize::begin(&mut self.builder.samsung_pay),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "visa_checkout" => Deserialize::begin(&mut self.builder.visa_checkout),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amex_express_checkout),
                Some(apple_pay),
                Some(dynamic_last4),
                Some(google_pay),
                Some(link),
                Some(masterpass),
                Some(samsung_pay),
                Some(type_),
                Some(visa_checkout),
            ) = (
                self.builder.amex_express_checkout,
                self.builder.apple_pay,
                self.builder.dynamic_last4.take(),
                self.builder.google_pay,
                self.builder.link,
                self.builder.masterpass.take(),
                self.builder.samsung_pay,
                self.builder.type_.take(),
                self.builder.visa_checkout.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodDetailsCardWallet {
                amex_express_checkout,
                apple_pay,
                dynamic_last4,
                google_pay,
                link,
                masterpass,
                samsung_pay,
                type_,
                visa_checkout,
            });
            Ok(())
        }
    }
};
/// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
/// An additional hash is included on the Wallet subhash with a name matching this value.
/// It contains additional information specific to the card wallet type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodDetailsCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Link,
    Masterpass,
    SamsungPay,
    VisaCheckout,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodDetailsCardWalletType {
    pub fn as_str(&self) -> &str {
        use PaymentMethodDetailsCardWalletType::*;
        match self {
            AmexExpressCheckout => "amex_express_checkout",
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            Link => "link",
            Masterpass => "masterpass",
            SamsungPay => "samsung_pay",
            VisaCheckout => "visa_checkout",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardWalletType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodDetailsCardWalletType::*;
        match s {
            "amex_express_checkout" => Ok(AmexExpressCheckout),
            "apple_pay" => Ok(ApplePay),
            "google_pay" => Ok(GooglePay),
            "link" => Ok(Link),
            "masterpass" => Ok(Masterpass),
            "samsung_pay" => Ok(SamsungPay),
            "visa_checkout" => Ok(VisaCheckout),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodDetailsCardWalletType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodDetailsCardWalletType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodDetailsCardWalletType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodDetailsCardWalletType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCardWalletType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodDetailsCardWalletType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardWalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
