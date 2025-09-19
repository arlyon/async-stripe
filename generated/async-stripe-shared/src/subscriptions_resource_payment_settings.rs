#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SubscriptionsResourcePaymentSettings {
    /// Payment-method-specific configuration to provide to invoices created by the subscription.
    pub payment_method_options: Option<stripe_shared::SubscriptionsResourcePaymentMethodOptions>,
    /// The list of payment method types to provide to every invoice created by the subscription.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<SubscriptionsResourcePaymentSettingsPaymentMethodTypes>>,
    /// Configure whether Stripe updates `subscription.default_payment_method` when payment succeeds.
    /// Defaults to `off`.
    pub save_default_payment_method:
        Option<SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod>,
}
#[doc(hidden)]
pub struct SubscriptionsResourcePaymentSettingsBuilder {
    payment_method_options:
        Option<Option<stripe_shared::SubscriptionsResourcePaymentMethodOptions>>,
    payment_method_types:
        Option<Option<Vec<SubscriptionsResourcePaymentSettingsPaymentMethodTypes>>>,
    save_default_payment_method:
        Option<Option<SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod>>,
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

    impl Deserialize for SubscriptionsResourcePaymentSettings {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SubscriptionsResourcePaymentSettings>,
        builder: SubscriptionsResourcePaymentSettingsBuilder,
    }

    impl Visitor for Place<SubscriptionsResourcePaymentSettings> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SubscriptionsResourcePaymentSettingsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SubscriptionsResourcePaymentSettingsBuilder {
        type Out = SubscriptionsResourcePaymentSettings;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "payment_method_options" => Deserialize::begin(&mut self.payment_method_options),
                "payment_method_types" => Deserialize::begin(&mut self.payment_method_types),
                "save_default_payment_method" => {
                    Deserialize::begin(&mut self.save_default_payment_method)
                }

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                payment_method_options: Deserialize::default(),
                payment_method_types: Deserialize::default(),
                save_default_payment_method: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(payment_method_options),
                Some(payment_method_types),
                Some(save_default_payment_method),
            ) = (
                self.payment_method_options.take(),
                self.payment_method_types.take(),
                self.save_default_payment_method,
            )
            else {
                return None;
            };
            Some(Self::Out {
                payment_method_options,
                payment_method_types,
                save_default_payment_method,
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

    impl ObjectDeser for SubscriptionsResourcePaymentSettings {
        type Builder = SubscriptionsResourcePaymentSettingsBuilder;
    }

    impl FromValueOpt for SubscriptionsResourcePaymentSettings {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SubscriptionsResourcePaymentSettingsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "payment_method_options" => {
                        b.payment_method_options = FromValueOpt::from_value(v)
                    }
                    "payment_method_types" => b.payment_method_types = FromValueOpt::from_value(v),
                    "save_default_payment_method" => {
                        b.save_default_payment_method = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The list of payment method types to provide to every invoice created by the subscription.
/// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    Affirm,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    Crypto,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    JpCreditTransfer,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    Link,
    Multibanco,
    NaverPay,
    NzBankAccount,
    P24,
    Payco,
    Paynow,
    Paypal,
    Promptpay,
    RevolutPay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    Swish,
    UsBankAccount,
    WechatPay,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    pub fn as_str(&self) -> &str {
        use SubscriptionsResourcePaymentSettingsPaymentMethodTypes::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            Crypto => "crypto",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            JpCreditTransfer => "jp_credit_transfer",
            KakaoPay => "kakao_pay",
            Klarna => "klarna",
            Konbini => "konbini",
            KrCard => "kr_card",
            Link => "link",
            Multibanco => "multibanco",
            NaverPay => "naver_pay",
            NzBankAccount => "nz_bank_account",
            P24 => "p24",
            Payco => "payco",
            Paynow => "paynow",
            Paypal => "paypal",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SepaCreditTransfer => "sepa_credit_transfer",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourcePaymentSettingsPaymentMethodTypes::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "crypto" => Ok(Crypto),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "jp_credit_transfer" => Ok(JpCreditTransfer),
            "kakao_pay" => Ok(KakaoPay),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "kr_card" => Ok(KrCard),
            "link" => Ok(Link),
            "multibanco" => Ok(Multibanco),
            "naver_pay" => Ok(NaverPay),
            "nz_bank_account" => Ok(NzBankAccount),
            "p24" => Ok(P24),
            "payco" => Ok(Payco),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "sepa_credit_transfer" => Ok(SepaCreditTransfer),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionsResourcePaymentSettingsPaymentMethodTypes>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(SubscriptionsResourcePaymentSettingsPaymentMethodTypes::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SubscriptionsResourcePaymentSettingsPaymentMethodTypes);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionsResourcePaymentSettingsPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Configure whether Stripe updates `subscription.default_payment_method` when payment succeeds.
/// Defaults to `off`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    Off,
    OnSubscription,
}
impl SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    pub fn as_str(self) -> &'static str {
        use SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::*;
        match self {
            Off => "off",
            OnSubscription => "on_subscription",
        }
    }
}

impl std::str::FromStr for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::*;
        match s {
            "off" => Ok(Off),
            "on_subscription" => Ok(OnSubscription),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for SubscriptionsResourcePaymentSettingsSaveDefaultPaymentMethod",
            )
        })
    }
}
