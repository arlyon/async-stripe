#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokenPaymentMethodPreviewCardWallet {
    pub amex_express_checkout: Option<stripe_shared::PaymentMethodCardWalletAmexExpressCheckout>,
    pub apple_pay: Option<stripe_shared::PaymentMethodCardWalletApplePay>,
    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,
    pub google_pay: Option<stripe_shared::PaymentMethodCardWalletGooglePay>,
    pub link: Option<stripe_payment::ConfirmationTokenPaymentMethodPreviewCardWalletLink>,
    pub masterpass: Option<stripe_shared::PaymentMethodCardWalletMasterpass>,
    pub samsung_pay: Option<stripe_shared::PaymentMethodCardWalletSamsungPay>,
    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, `visa_checkout`, or `link`.
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ConfirmationTokenPaymentMethodPreviewCardWalletType,
    pub visa_checkout: Option<stripe_shared::PaymentMethodCardWalletVisaCheckout>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokenPaymentMethodPreviewCardWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmationTokenPaymentMethodPreviewCardWallet").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConfirmationTokenPaymentMethodPreviewCardWalletBuilder {
    amex_express_checkout:
        Option<Option<stripe_shared::PaymentMethodCardWalletAmexExpressCheckout>>,
    apple_pay: Option<Option<stripe_shared::PaymentMethodCardWalletApplePay>>,
    dynamic_last4: Option<Option<String>>,
    google_pay: Option<Option<stripe_shared::PaymentMethodCardWalletGooglePay>>,
    link: Option<Option<stripe_payment::ConfirmationTokenPaymentMethodPreviewCardWalletLink>>,
    masterpass: Option<Option<stripe_shared::PaymentMethodCardWalletMasterpass>>,
    samsung_pay: Option<Option<stripe_shared::PaymentMethodCardWalletSamsungPay>>,
    type_: Option<ConfirmationTokenPaymentMethodPreviewCardWalletType>,
    visa_checkout: Option<Option<stripe_shared::PaymentMethodCardWalletVisaCheckout>>,
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

    impl Deserialize for ConfirmationTokenPaymentMethodPreviewCardWallet {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConfirmationTokenPaymentMethodPreviewCardWallet>,
        builder: ConfirmationTokenPaymentMethodPreviewCardWalletBuilder,
    }

    impl Visitor for Place<ConfirmationTokenPaymentMethodPreviewCardWallet> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConfirmationTokenPaymentMethodPreviewCardWalletBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConfirmationTokenPaymentMethodPreviewCardWalletBuilder {
        type Out = ConfirmationTokenPaymentMethodPreviewCardWallet;
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
                amex_express_checkout: Some(None),
                apple_pay: Some(None),
                dynamic_last4: Some(None),
                google_pay: Some(None),
                link: Some(None),
                masterpass: Some(None),
                samsung_pay: Some(None),
                type_: None,
                visa_checkout: Some(None),
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
                self.type_.take(),
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

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for ConfirmationTokenPaymentMethodPreviewCardWallet {
        type Builder = ConfirmationTokenPaymentMethodPreviewCardWalletBuilder;
    }

    impl FromValueOpt for ConfirmationTokenPaymentMethodPreviewCardWallet {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConfirmationTokenPaymentMethodPreviewCardWalletBuilder::deser_default();
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmationTokenPaymentMethodPreviewCardWalletType {
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
impl ConfirmationTokenPaymentMethodPreviewCardWalletType {
    pub fn as_str(&self) -> &str {
        use ConfirmationTokenPaymentMethodPreviewCardWalletType::*;
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

impl std::str::FromStr for ConfirmationTokenPaymentMethodPreviewCardWalletType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmationTokenPaymentMethodPreviewCardWalletType::*;
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
                    "ConfirmationTokenPaymentMethodPreviewCardWalletType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ConfirmationTokenPaymentMethodPreviewCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ConfirmationTokenPaymentMethodPreviewCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokenPaymentMethodPreviewCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ConfirmationTokenPaymentMethodPreviewCardWalletType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ConfirmationTokenPaymentMethodPreviewCardWalletType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ConfirmationTokenPaymentMethodPreviewCardWalletType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ConfirmationTokenPaymentMethodPreviewCardWalletType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ConfirmationTokenPaymentMethodPreviewCardWalletType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ConfirmationTokenPaymentMethodPreviewCardWalletType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmationTokenPaymentMethodPreviewCardWalletType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
