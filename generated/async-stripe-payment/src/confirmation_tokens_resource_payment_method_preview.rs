/// Details of the PaymentMethod collected by Payment Element
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourcePaymentMethodPreview {
    pub acss_debit: Option<stripe_shared::PaymentMethodAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipay>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to “unspecified”.
    pub allow_redisplay: Option<ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay>,
    pub alma: Option<stripe_shared::PaymentMethodAlma>,
    pub amazon_pay: Option<stripe_shared::PaymentMethodAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::PaymentMethodAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentMethodBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodBancontact>,
    pub billie: Option<stripe_shared::PaymentMethodBillie>,
    pub billing_details: stripe_shared::BillingDetails,
    pub bizum: Option<stripe_shared::PaymentMethodBizum>,
    pub blik: Option<stripe_shared::PaymentMethodBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodBoleto>,
    pub card: Option<stripe_shared::PaymentMethodCard>,
    pub card_present: Option<stripe_shared::PaymentMethodCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodCashapp>,
    pub crypto: Option<stripe_shared::PaymentMethodCrypto>,
    /// The ID of the Customer to which this PaymentMethod is saved.
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    pub customer_account: Option<String>,
    pub customer_balance: Option<stripe_shared::PaymentMethodCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentMethodEps>,
    pub fpx: Option<stripe_shared::PaymentMethodFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodGrabpay>,
    pub ideal: Option<stripe_shared::PaymentMethodIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodInteracPresent>,
    pub kakao_pay: Option<stripe_shared::PaymentMethodKakaoPay>,
    pub klarna: Option<stripe_shared::PaymentMethodKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodKonbini>,
    pub kr_card: Option<stripe_shared::PaymentMethodKrCard>,
    pub link: Option<stripe_shared::PaymentMethodLink>,
    pub mb_way: Option<stripe_shared::PaymentMethodMbWay>,
    pub mobilepay: Option<stripe_shared::PaymentMethodMobilepay>,
    pub multibanco: Option<stripe_shared::PaymentMethodMultibanco>,
    pub naver_pay: Option<stripe_shared::PaymentMethodNaverPay>,
    pub nz_bank_account: Option<stripe_shared::PaymentMethodNzBankAccount>,
    pub oxxo: Option<stripe_shared::PaymentMethodOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodP24>,
    pub pay_by_bank: Option<stripe_shared::PaymentMethodPayByBank>,
    pub payco: Option<stripe_shared::PaymentMethodPayco>,
    pub paynow: Option<stripe_shared::PaymentMethodPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodPaypal>,
    pub payto: Option<stripe_shared::PaymentMethodPayto>,
    pub pix: Option<stripe_shared::PaymentMethodPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodRevolutPay>,
    pub samsung_pay: Option<stripe_shared::PaymentMethodSamsungPay>,
    pub satispay: Option<stripe_shared::PaymentMethodSatispay>,
    pub scalapay: Option<stripe_shared::PaymentMethodScalapay>,
    pub sepa_debit: Option<stripe_shared::PaymentMethodSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodSofort>,
    pub sunbit: Option<stripe_shared::PaymentMethodSunbit>,
    pub swish: Option<stripe_shared::PaymentMethodSwish>,
    pub twint: Option<stripe_shared::PaymentMethodTwint>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ConfirmationTokensResourcePaymentMethodPreviewType,
    pub upi: Option<stripe_shared::PaymentMethodUpi>,
    pub us_bank_account: Option<stripe_shared::PaymentMethodUsBankAccount>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodZip>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokensResourcePaymentMethodPreview {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ConfirmationTokensResourcePaymentMethodPreview").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ConfirmationTokensResourcePaymentMethodPreviewBuilder {
    acss_debit: Option<stripe_shared::PaymentMethodAcssDebit>,
    affirm: Option<stripe_shared::PaymentMethodAffirm>,
    afterpay_clearpay: Option<stripe_shared::PaymentMethodAfterpayClearpay>,
    alipay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipay>,
    allow_redisplay: Option<ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay>,
    alma: Option<stripe_shared::PaymentMethodAlma>,
    amazon_pay: Option<stripe_shared::PaymentMethodAmazonPay>,
    au_becs_debit: Option<stripe_shared::PaymentMethodAuBecsDebit>,
    bacs_debit: Option<stripe_shared::PaymentMethodBacsDebit>,
    bancontact: Option<stripe_shared::PaymentMethodBancontact>,
    billie: Option<stripe_shared::PaymentMethodBillie>,
    billing_details: Option<stripe_shared::BillingDetails>,
    bizum: Option<stripe_shared::PaymentMethodBizum>,
    blik: Option<stripe_shared::PaymentMethodBlik>,
    boleto: Option<stripe_shared::PaymentMethodBoleto>,
    card: Option<stripe_shared::PaymentMethodCard>,
    card_present: Option<stripe_shared::PaymentMethodCardPresent>,
    cashapp: Option<stripe_shared::PaymentMethodCashapp>,
    crypto: Option<stripe_shared::PaymentMethodCrypto>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    customer_account: Option<Option<String>>,
    customer_balance: Option<stripe_shared::PaymentMethodCustomerBalance>,
    eps: Option<stripe_shared::PaymentMethodEps>,
    fpx: Option<stripe_shared::PaymentMethodFpx>,
    giropay: Option<stripe_shared::PaymentMethodGiropay>,
    grabpay: Option<stripe_shared::PaymentMethodGrabpay>,
    ideal: Option<stripe_shared::PaymentMethodIdeal>,
    interac_present: Option<stripe_shared::PaymentMethodInteracPresent>,
    kakao_pay: Option<stripe_shared::PaymentMethodKakaoPay>,
    klarna: Option<stripe_shared::PaymentMethodKlarna>,
    konbini: Option<stripe_shared::PaymentMethodKonbini>,
    kr_card: Option<stripe_shared::PaymentMethodKrCard>,
    link: Option<stripe_shared::PaymentMethodLink>,
    mb_way: Option<stripe_shared::PaymentMethodMbWay>,
    mobilepay: Option<stripe_shared::PaymentMethodMobilepay>,
    multibanco: Option<stripe_shared::PaymentMethodMultibanco>,
    naver_pay: Option<stripe_shared::PaymentMethodNaverPay>,
    nz_bank_account: Option<stripe_shared::PaymentMethodNzBankAccount>,
    oxxo: Option<stripe_shared::PaymentMethodOxxo>,
    p24: Option<stripe_shared::PaymentMethodP24>,
    pay_by_bank: Option<stripe_shared::PaymentMethodPayByBank>,
    payco: Option<stripe_shared::PaymentMethodPayco>,
    paynow: Option<stripe_shared::PaymentMethodPaynow>,
    paypal: Option<stripe_shared::PaymentMethodPaypal>,
    payto: Option<stripe_shared::PaymentMethodPayto>,
    pix: Option<stripe_shared::PaymentMethodPix>,
    promptpay: Option<stripe_shared::PaymentMethodPromptpay>,
    revolut_pay: Option<stripe_shared::PaymentMethodRevolutPay>,
    samsung_pay: Option<stripe_shared::PaymentMethodSamsungPay>,
    satispay: Option<stripe_shared::PaymentMethodSatispay>,
    scalapay: Option<stripe_shared::PaymentMethodScalapay>,
    sepa_debit: Option<stripe_shared::PaymentMethodSepaDebit>,
    sofort: Option<stripe_shared::PaymentMethodSofort>,
    sunbit: Option<stripe_shared::PaymentMethodSunbit>,
    swish: Option<stripe_shared::PaymentMethodSwish>,
    twint: Option<stripe_shared::PaymentMethodTwint>,
    type_: Option<ConfirmationTokensResourcePaymentMethodPreviewType>,
    upi: Option<stripe_shared::PaymentMethodUpi>,
    us_bank_account: Option<stripe_shared::PaymentMethodUsBankAccount>,
    wechat_pay: Option<stripe_shared::PaymentMethodWechatPay>,
    zip: Option<stripe_shared::PaymentMethodZip>,
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

    impl Deserialize for ConfirmationTokensResourcePaymentMethodPreview {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConfirmationTokensResourcePaymentMethodPreview>,
        builder: ConfirmationTokensResourcePaymentMethodPreviewBuilder,
    }

    impl Visitor for Place<ConfirmationTokensResourcePaymentMethodPreview> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConfirmationTokensResourcePaymentMethodPreviewBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConfirmationTokensResourcePaymentMethodPreviewBuilder {
        type Out = ConfirmationTokensResourcePaymentMethodPreview;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.acss_debit)
                }
                "affirm" => stripe_types::miniserde_helpers::begin_nullable(&mut self.affirm),
                "afterpay_clearpay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.afterpay_clearpay)
                }
                "alipay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.alipay),
                "allow_redisplay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.allow_redisplay)
                }
                "alma" => stripe_types::miniserde_helpers::begin_nullable(&mut self.alma),
                "amazon_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.amazon_pay)
                }
                "au_becs_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.au_becs_debit)
                }
                "bacs_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bacs_debit)
                }
                "bancontact" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.bancontact)
                }
                "billie" => stripe_types::miniserde_helpers::begin_nullable(&mut self.billie),
                "billing_details" => Deserialize::begin(&mut self.billing_details),
                "bizum" => stripe_types::miniserde_helpers::begin_nullable(&mut self.bizum),
                "blik" => stripe_types::miniserde_helpers::begin_nullable(&mut self.blik),
                "boleto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.boleto),
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "card_present" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_present)
                }
                "cashapp" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp),
                "crypto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.crypto),
                "customer" => stripe_types::miniserde_helpers::begin_nullable(&mut self.customer),
                "customer_account" => Deserialize::begin(&mut self.customer_account),
                "customer_balance" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.customer_balance)
                }
                "eps" => stripe_types::miniserde_helpers::begin_nullable(&mut self.eps),
                "fpx" => stripe_types::miniserde_helpers::begin_nullable(&mut self.fpx),
                "giropay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.giropay),
                "grabpay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.grabpay),
                "ideal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.ideal),
                "interac_present" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.interac_present)
                }
                "kakao_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.kakao_pay),
                "klarna" => stripe_types::miniserde_helpers::begin_nullable(&mut self.klarna),
                "konbini" => stripe_types::miniserde_helpers::begin_nullable(&mut self.konbini),
                "kr_card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.kr_card),
                "link" => stripe_types::miniserde_helpers::begin_nullable(&mut self.link),
                "mb_way" => stripe_types::miniserde_helpers::begin_nullable(&mut self.mb_way),
                "mobilepay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.mobilepay),
                "multibanco" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.multibanco)
                }
                "naver_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.naver_pay),
                "nz_bank_account" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.nz_bank_account)
                }
                "oxxo" => stripe_types::miniserde_helpers::begin_nullable(&mut self.oxxo),
                "p24" => stripe_types::miniserde_helpers::begin_nullable(&mut self.p24),
                "pay_by_bank" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.pay_by_bank)
                }
                "payco" => stripe_types::miniserde_helpers::begin_nullable(&mut self.payco),
                "paynow" => stripe_types::miniserde_helpers::begin_nullable(&mut self.paynow),
                "paypal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.paypal),
                "payto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.payto),
                "pix" => stripe_types::miniserde_helpers::begin_nullable(&mut self.pix),
                "promptpay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.promptpay),
                "revolut_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.revolut_pay)
                }
                "samsung_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.samsung_pay)
                }
                "satispay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.satispay),
                "scalapay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.scalapay),
                "sepa_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sepa_debit)
                }
                "sofort" => stripe_types::miniserde_helpers::begin_nullable(&mut self.sofort),
                "sunbit" => stripe_types::miniserde_helpers::begin_nullable(&mut self.sunbit),
                "swish" => stripe_types::miniserde_helpers::begin_nullable(&mut self.swish),
                "twint" => stripe_types::miniserde_helpers::begin_nullable(&mut self.twint),
                "type" => Deserialize::begin(&mut self.type_),
                "upi" => stripe_types::miniserde_helpers::begin_nullable(&mut self.upi),
                "us_bank_account" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.us_bank_account)
                }
                "wechat_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.wechat_pay)
                }
                "zip" => stripe_types::miniserde_helpers::begin_nullable(&mut self.zip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: None,
                affirm: None,
                afterpay_clearpay: None,
                alipay: None,
                allow_redisplay: None,
                alma: None,
                amazon_pay: None,
                au_becs_debit: None,
                bacs_debit: None,
                bancontact: None,
                billie: None,
                billing_details: None,
                bizum: None,
                blik: None,
                boleto: None,
                card: None,
                card_present: None,
                cashapp: None,
                crypto: None,
                customer: None,
                customer_account: Some(None),
                customer_balance: None,
                eps: None,
                fpx: None,
                giropay: None,
                grabpay: None,
                ideal: None,
                interac_present: None,
                kakao_pay: None,
                klarna: None,
                konbini: None,
                kr_card: None,
                link: None,
                mb_way: None,
                mobilepay: None,
                multibanco: None,
                naver_pay: None,
                nz_bank_account: None,
                oxxo: None,
                p24: None,
                pay_by_bank: None,
                payco: None,
                paynow: None,
                paypal: None,
                payto: None,
                pix: None,
                promptpay: None,
                revolut_pay: None,
                samsung_pay: None,
                satispay: None,
                scalapay: None,
                sepa_debit: None,
                sofort: None,
                sunbit: None,
                swish: None,
                twint: None,
                type_: None,
                upi: None,
                us_bank_account: None,
                wechat_pay: None,
                zip: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self {
                    acss_debit,
                    affirm,
                    afterpay_clearpay,
                    alipay,
                    allow_redisplay,
                    alma,
                    amazon_pay,
                    au_becs_debit,
                    bacs_debit,
                    bancontact,
                    billie,
                    billing_details: Some(billing_details),
                    bizum,
                    blik,
                    boleto,
                    card,
                    card_present,
                    cashapp,
                    crypto,
                    customer,
                    customer_account: Some(customer_account),
                    customer_balance,
                    eps,
                    fpx,
                    giropay,
                    grabpay,
                    ideal,
                    interac_present,
                    kakao_pay,
                    klarna,
                    konbini,
                    kr_card,
                    link,
                    mb_way,
                    mobilepay,
                    multibanco,
                    naver_pay,
                    nz_bank_account,
                    oxxo,
                    p24,
                    pay_by_bank,
                    payco,
                    paynow,
                    paypal,
                    payto,
                    pix,
                    promptpay,
                    revolut_pay,
                    samsung_pay,
                    satispay,
                    scalapay,
                    sepa_debit,
                    sofort,
                    sunbit,
                    swish,
                    twint,
                    type_: Some(type_),
                    upi,
                    us_bank_account,
                    wechat_pay,
                    zip,
                } => Some(Self::Out {
                    acss_debit,
                    affirm,
                    afterpay_clearpay,
                    alipay,
                    allow_redisplay,
                    alma,
                    amazon_pay,
                    au_becs_debit,
                    bacs_debit,
                    bancontact,
                    billie,
                    billing_details,
                    bizum,
                    blik,
                    boleto,
                    card,
                    card_present,
                    cashapp,
                    crypto,
                    customer,
                    customer_account,
                    customer_balance,
                    eps,
                    fpx,
                    giropay,
                    grabpay,
                    ideal,
                    interac_present,
                    kakao_pay,
                    klarna,
                    konbini,
                    kr_card,
                    link,
                    mb_way,
                    mobilepay,
                    multibanco,
                    naver_pay,
                    nz_bank_account,
                    oxxo,
                    p24,
                    pay_by_bank,
                    payco,
                    paynow,
                    paypal,
                    payto,
                    pix,
                    promptpay,
                    revolut_pay,
                    samsung_pay,
                    satispay,
                    scalapay,
                    sepa_debit,
                    sofort,
                    sunbit,
                    swish,
                    twint,
                    type_,
                    upi,
                    us_bank_account,
                    wechat_pay,
                    zip,
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for ConfirmationTokensResourcePaymentMethodPreview {
        type Builder = ConfirmationTokensResourcePaymentMethodPreviewBuilder;
    }

    impl FromValueOpt for ConfirmationTokensResourcePaymentMethodPreview {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to “unspecified”.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    Always,
    Limited,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    pub fn as_str(&self) -> &str {
        use ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmationTokensResourcePaymentMethodPreviewType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Bizum,
    Blik,
    Boleto,
    Card,
    CardPresent,
    Cashapp,
    Crypto,
    Custom,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    Link,
    MbWay,
    Mobilepay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    Oxxo,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Payto,
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    Scalapay,
    SepaDebit,
    Sofort,
    Sunbit,
    Swish,
    Twint,
    Upi,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ConfirmationTokensResourcePaymentMethodPreviewType {
    pub fn as_str(&self) -> &str {
        use ConfirmationTokensResourcePaymentMethodPreviewType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            Alma => "alma",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Billie => "billie",
            Bizum => "bizum",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            CardPresent => "card_present",
            Cashapp => "cashapp",
            Crypto => "crypto",
            Custom => "custom",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            InteracPresent => "interac_present",
            KakaoPay => "kakao_pay",
            Klarna => "klarna",
            Konbini => "konbini",
            KrCard => "kr_card",
            Link => "link",
            MbWay => "mb_way",
            Mobilepay => "mobilepay",
            Multibanco => "multibanco",
            NaverPay => "naver_pay",
            NzBankAccount => "nz_bank_account",
            Oxxo => "oxxo",
            P24 => "p24",
            PayByBank => "pay_by_bank",
            Payco => "payco",
            Paynow => "paynow",
            Paypal => "paypal",
            Payto => "payto",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SamsungPay => "samsung_pay",
            Satispay => "satispay",
            Scalapay => "scalapay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Sunbit => "sunbit",
            Swish => "swish",
            Twint => "twint",
            Upi => "upi",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ConfirmationTokensResourcePaymentMethodPreviewType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmationTokensResourcePaymentMethodPreviewType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "alma" => Ok(Alma),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "billie" => Ok(Billie),
            "bizum" => Ok(Bizum),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "card_present" => Ok(CardPresent),
            "cashapp" => Ok(Cashapp),
            "crypto" => Ok(Crypto),
            "custom" => Ok(Custom),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "interac_present" => Ok(InteracPresent),
            "kakao_pay" => Ok(KakaoPay),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "kr_card" => Ok(KrCard),
            "link" => Ok(Link),
            "mb_way" => Ok(MbWay),
            "mobilepay" => Ok(Mobilepay),
            "multibanco" => Ok(Multibanco),
            "naver_pay" => Ok(NaverPay),
            "nz_bank_account" => Ok(NzBankAccount),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "pay_by_bank" => Ok(PayByBank),
            "payco" => Ok(Payco),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "payto" => Ok(Payto),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "samsung_pay" => Ok(SamsungPay),
            "satispay" => Ok(Satispay),
            "scalapay" => Ok(Scalapay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "sunbit" => Ok(Sunbit),
            "swish" => Ok(Swish),
            "twint" => Ok(Twint),
            "upi" => Ok(Upi),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ConfirmationTokensResourcePaymentMethodPreviewType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ConfirmationTokensResourcePaymentMethodPreviewType))
            .finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ConfirmationTokensResourcePaymentMethodPreviewType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ConfirmationTokensResourcePaymentMethodPreviewType::from_str(s).expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ConfirmationTokensResourcePaymentMethodPreviewType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
