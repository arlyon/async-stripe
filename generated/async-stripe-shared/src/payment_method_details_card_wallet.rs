#[derive(Clone, Debug)]
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
                builder: PaymentMethodDetailsCardWalletBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsCardWalletBuilder {
        type Out = PaymentMethodDetailsCardWallet;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amex_express_checkout" => Deserialize::begin(&mut self.amex_express_checkout),
                "apple_pay" => Deserialize::begin(&mut self.apple_pay),
                "dynamic_last4" => Deserialize::begin(&mut self.dynamic_last4),
                "google_pay" => Deserialize::begin(&mut self.google_pay),
                "link" => Deserialize::begin(&mut self.link),
                "masterpass" => Deserialize::begin(&mut self.masterpass),
                "samsung_pay" => Deserialize::begin(&mut self.samsung_pay),
                "type" => Deserialize::begin(&mut self.type_),
                "visa_checkout" => Deserialize::begin(&mut self.visa_checkout),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amex_express_checkout: Deserialize::default(),
                apple_pay: Deserialize::default(),
                dynamic_last4: Deserialize::default(),
                google_pay: Deserialize::default(),
                link: Deserialize::default(),
                masterpass: Deserialize::default(),
                samsung_pay: Deserialize::default(),
                type_: Deserialize::default(),
                visa_checkout: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
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
                self.amex_express_checkout,
                self.apple_pay,
                self.dynamic_last4.take(),
                self.google_pay,
                self.link,
                self.masterpass.take(),
                self.samsung_pay,
                self.type_,
                self.visa_checkout.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                amex_express_checkout,
                apple_pay,
                dynamic_last4,
                google_pay,
                link,
                masterpass,
                samsung_pay,
                type_,
                visa_checkout,
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

    impl ObjectDeser for PaymentMethodDetailsCardWallet {
        type Builder = PaymentMethodDetailsCardWalletBuilder;
    }

    impl FromValueOpt for PaymentMethodDetailsCardWallet {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsCardWalletBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amex_express_checkout" => {
                        b.amex_express_checkout = FromValueOpt::from_value(v)
                    }
                    "apple_pay" => b.apple_pay = FromValueOpt::from_value(v),
                    "dynamic_last4" => b.dynamic_last4 = FromValueOpt::from_value(v),
                    "google_pay" => b.google_pay = FromValueOpt::from_value(v),
                    "link" => b.link = FromValueOpt::from_value(v),
                    "masterpass" => b.masterpass = FromValueOpt::from_value(v),
                    "samsung_pay" => b.samsung_pay = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "visa_checkout" => b.visa_checkout = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
/// An additional hash is included on the Wallet subhash with a name matching this value.
/// It contains additional information specific to the card wallet type.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentMethodDetailsCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Link,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}
impl PaymentMethodDetailsCardWalletType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodDetailsCardWalletType::*;
        match self {
            AmexExpressCheckout => "amex_express_checkout",
            ApplePay => "apple_pay",
            GooglePay => "google_pay",
            Link => "link",
            Masterpass => "masterpass",
            SamsungPay => "samsung_pay",
            VisaCheckout => "visa_checkout",
        }
    }
}

impl std::str::FromStr for PaymentMethodDetailsCardWalletType {
    type Err = stripe_types::StripeParseError;
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
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for PaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for PaymentMethodDetailsCardWalletType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentMethodDetailsCardWalletType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentMethodDetailsCardWalletType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentMethodDetailsCardWalletType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodDetailsCardWalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentMethodDetailsCardWalletType")
        })
    }
}
