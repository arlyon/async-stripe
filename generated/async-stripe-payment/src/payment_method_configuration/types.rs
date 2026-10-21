/// PaymentMethodConfigurations control which payment methods are displayed to your customers when you don't explicitly specify payment method types.
/// You can have multiple configurations with different sets of payment methods for different scenarios.
///
/// There are two types of PaymentMethodConfigurations.
/// Which is used depends on the [charge type](https://docs.stripe.com/connect/charges):.
///
/// **Direct** configurations apply to payments created on your account, including Connect destination charges, Connect separate charges and transfers, and payments not involving Connect.
///
/// **Child** configurations apply to payments created on your connected accounts using direct charges, and charges with the on_behalf_of parameter.
///
/// Child configurations have a `parent` that sets default values and controls which settings connected accounts may override.
/// You can specify a parent ID at payment time, and Stripe will automatically resolve the connected account’s associated child configuration.
/// Parent configurations are [managed in the dashboard](https://dashboard.stripe.com/settings/payment_methods/connected_accounts) and are not available in this API.
///
/// Related guides:
/// - [Payment Method Configurations API](https://docs.stripe.com/connect/payment-method-configurations).
/// - [Multiple configurations on dynamic payment methods](https://docs.stripe.com/payments/multiple-payment-method-configs).
/// - [Multiple configurations for your Connect accounts](https://docs.stripe.com/connect/multiple-payment-method-configurations).
///
/// For more details see <<https://stripe.com/docs/api/payment_method_configurations/object>>.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodConfiguration {
    pub acss_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Whether the configuration can be used for new payments.
    pub active: bool,
    pub affirm: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub afterpay_clearpay:
        Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub alipay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub alma: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub amazon_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub apple_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// For child configs, the Connect application associated with the configuration.
    pub application: Option<String>,
    pub au_becs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub bacs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub bancontact: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub billie: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub bizum: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub blik: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub boleto: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub card: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub cartes_bancaires:
        Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub cashapp: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub crypto: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub customer_balance:
        Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub eps: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub fpx: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub giropay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub google_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub grabpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Unique identifier for the object.
    pub id: stripe_payment::PaymentMethodConfigurationId,
    pub ideal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// The default configuration is used whenever a payment method configuration is not specified.
    pub is_default: bool,
    pub jcb: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub kakao_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub klarna: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub konbini: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub kr_card: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub link: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub mb_way: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub mobilepay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub multibanco: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// The configuration's name.
    pub name: String,
    pub naver_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub nz_bank_account: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub oxxo: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub p24: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// For child configs, the configuration's parent configuration.
    pub parent: Option<String>,
    pub pay_by_bank: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub payco: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub paynow: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub paypal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub payto: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub pix: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub promptpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub revolut_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub samsung_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub satispay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub scalapay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sepa_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sofort: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sunbit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub swish: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub twint: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub upi: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub us_bank_account: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub wechat_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub zip: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodConfiguration").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodConfigurationBuilder {
    acss_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    active: Option<bool>,
    affirm: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    afterpay_clearpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    alipay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    alma: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    amazon_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    apple_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    application: Option<Option<String>>,
    au_becs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    bacs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    bancontact: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    billie: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    bizum: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    blik: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    boleto: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    card: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    cartes_bancaires: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    cashapp: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    crypto: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    customer_balance: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    eps: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    fpx: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    giropay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    google_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    grabpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    id: Option<stripe_payment::PaymentMethodConfigurationId>,
    ideal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    is_default: Option<bool>,
    jcb: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    kakao_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    klarna: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    konbini: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    kr_card: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    link: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    livemode: Option<bool>,
    mb_way: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    mobilepay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    multibanco: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    name: Option<String>,
    naver_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    nz_bank_account: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    oxxo: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    p24: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    parent: Option<Option<String>>,
    pay_by_bank: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    payco: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    paynow: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    paypal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    payto: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pix: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    promptpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    revolut_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    samsung_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    satispay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    scalapay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    sepa_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    sofort: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    sunbit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    swish: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    twint: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    upi: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    us_bank_account: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    wechat_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    zip: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
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

    impl Deserialize for PaymentMethodConfiguration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodConfiguration>,
        builder: PaymentMethodConfigurationBuilder,
    }

    impl Visitor for Place<PaymentMethodConfiguration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodConfigurationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodConfigurationBuilder {
        type Out = PaymentMethodConfiguration;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.acss_debit)
                }
                "active" => Deserialize::begin(&mut self.active),
                "affirm" => stripe_types::miniserde_helpers::begin_nullable(&mut self.affirm),
                "afterpay_clearpay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.afterpay_clearpay)
                }
                "alipay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.alipay),
                "alma" => stripe_types::miniserde_helpers::begin_nullable(&mut self.alma),
                "amazon_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.amazon_pay)
                }
                "apple_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.apple_pay),
                "application" => Deserialize::begin(&mut self.application),
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
                "bizum" => stripe_types::miniserde_helpers::begin_nullable(&mut self.bizum),
                "blik" => stripe_types::miniserde_helpers::begin_nullable(&mut self.blik),
                "boleto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.boleto),
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "cartes_bancaires" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.cartes_bancaires)
                }
                "cashapp" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp),
                "crypto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.crypto),
                "customer_balance" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.customer_balance)
                }
                "eps" => stripe_types::miniserde_helpers::begin_nullable(&mut self.eps),
                "fpx" => stripe_types::miniserde_helpers::begin_nullable(&mut self.fpx),
                "giropay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.giropay),
                "google_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.google_pay)
                }
                "grabpay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.grabpay),
                "id" => Deserialize::begin(&mut self.id),
                "ideal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.ideal),
                "is_default" => Deserialize::begin(&mut self.is_default),
                "jcb" => stripe_types::miniserde_helpers::begin_nullable(&mut self.jcb),
                "kakao_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.kakao_pay),
                "klarna" => stripe_types::miniserde_helpers::begin_nullable(&mut self.klarna),
                "konbini" => stripe_types::miniserde_helpers::begin_nullable(&mut self.konbini),
                "kr_card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.kr_card),
                "link" => stripe_types::miniserde_helpers::begin_nullable(&mut self.link),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "mb_way" => stripe_types::miniserde_helpers::begin_nullable(&mut self.mb_way),
                "mobilepay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.mobilepay),
                "multibanco" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.multibanco)
                }
                "name" => Deserialize::begin(&mut self.name),
                "naver_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.naver_pay),
                "nz_bank_account" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.nz_bank_account)
                }
                "oxxo" => stripe_types::miniserde_helpers::begin_nullable(&mut self.oxxo),
                "p24" => stripe_types::miniserde_helpers::begin_nullable(&mut self.p24),
                "parent" => Deserialize::begin(&mut self.parent),
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
                active: None,
                affirm: None,
                afterpay_clearpay: None,
                alipay: None,
                alma: None,
                amazon_pay: None,
                apple_pay: None,
                application: Some(None),
                au_becs_debit: None,
                bacs_debit: None,
                bancontact: None,
                billie: None,
                bizum: None,
                blik: None,
                boleto: None,
                card: None,
                cartes_bancaires: None,
                cashapp: None,
                crypto: None,
                customer_balance: None,
                eps: None,
                fpx: None,
                giropay: None,
                google_pay: None,
                grabpay: None,
                id: None,
                ideal: None,
                is_default: None,
                jcb: None,
                kakao_pay: None,
                klarna: None,
                konbini: None,
                kr_card: None,
                link: None,
                livemode: None,
                mb_way: None,
                mobilepay: None,
                multibanco: None,
                name: None,
                naver_pay: None,
                nz_bank_account: None,
                oxxo: None,
                p24: None,
                parent: Some(None),
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
                    active: Some(active),
                    affirm,
                    afterpay_clearpay,
                    alipay,
                    alma,
                    amazon_pay,
                    apple_pay,
                    application: Some(application),
                    au_becs_debit,
                    bacs_debit,
                    bancontact,
                    billie,
                    bizum,
                    blik,
                    boleto,
                    card,
                    cartes_bancaires,
                    cashapp,
                    crypto,
                    customer_balance,
                    eps,
                    fpx,
                    giropay,
                    google_pay,
                    grabpay,
                    id: Some(id),
                    ideal,
                    is_default: Some(is_default),
                    jcb,
                    kakao_pay,
                    klarna,
                    konbini,
                    kr_card,
                    link,
                    livemode: Some(livemode),
                    mb_way,
                    mobilepay,
                    multibanco,
                    name: Some(name),
                    naver_pay,
                    nz_bank_account,
                    oxxo,
                    p24,
                    parent: Some(parent),
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
                    upi,
                    us_bank_account,
                    wechat_pay,
                    zip,
                } => Some(Self::Out {
                    acss_debit,
                    active,
                    affirm,
                    afterpay_clearpay,
                    alipay,
                    alma,
                    amazon_pay,
                    apple_pay,
                    application,
                    au_becs_debit,
                    bacs_debit,
                    bancontact,
                    billie,
                    bizum,
                    blik,
                    boleto,
                    card,
                    cartes_bancaires,
                    cashapp,
                    crypto,
                    customer_balance,
                    eps,
                    fpx,
                    giropay,
                    google_pay,
                    grabpay,
                    id,
                    ideal,
                    is_default,
                    jcb,
                    kakao_pay,
                    klarna,
                    konbini,
                    kr_card,
                    link,
                    livemode,
                    mb_way,
                    mobilepay,
                    multibanco,
                    name,
                    naver_pay,
                    nz_bank_account,
                    oxxo,
                    p24,
                    parent,
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

    impl ObjectDeser for PaymentMethodConfiguration {
        type Builder = PaymentMethodConfigurationBuilder;
    }

    impl FromValueOpt for PaymentMethodConfiguration {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodConfiguration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentMethodConfiguration", 66)?;
        s.serialize_field("acss_debit", &self.acss_debit)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("affirm", &self.affirm)?;
        s.serialize_field("afterpay_clearpay", &self.afterpay_clearpay)?;
        s.serialize_field("alipay", &self.alipay)?;
        s.serialize_field("alma", &self.alma)?;
        s.serialize_field("amazon_pay", &self.amazon_pay)?;
        s.serialize_field("apple_pay", &self.apple_pay)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("au_becs_debit", &self.au_becs_debit)?;
        s.serialize_field("bacs_debit", &self.bacs_debit)?;
        s.serialize_field("bancontact", &self.bancontact)?;
        s.serialize_field("billie", &self.billie)?;
        s.serialize_field("bizum", &self.bizum)?;
        s.serialize_field("blik", &self.blik)?;
        s.serialize_field("boleto", &self.boleto)?;
        s.serialize_field("card", &self.card)?;
        s.serialize_field("cartes_bancaires", &self.cartes_bancaires)?;
        s.serialize_field("cashapp", &self.cashapp)?;
        s.serialize_field("crypto", &self.crypto)?;
        s.serialize_field("customer_balance", &self.customer_balance)?;
        s.serialize_field("eps", &self.eps)?;
        s.serialize_field("fpx", &self.fpx)?;
        s.serialize_field("giropay", &self.giropay)?;
        s.serialize_field("google_pay", &self.google_pay)?;
        s.serialize_field("grabpay", &self.grabpay)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("ideal", &self.ideal)?;
        s.serialize_field("is_default", &self.is_default)?;
        s.serialize_field("jcb", &self.jcb)?;
        s.serialize_field("kakao_pay", &self.kakao_pay)?;
        s.serialize_field("klarna", &self.klarna)?;
        s.serialize_field("konbini", &self.konbini)?;
        s.serialize_field("kr_card", &self.kr_card)?;
        s.serialize_field("link", &self.link)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("mb_way", &self.mb_way)?;
        s.serialize_field("mobilepay", &self.mobilepay)?;
        s.serialize_field("multibanco", &self.multibanco)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("naver_pay", &self.naver_pay)?;
        s.serialize_field("nz_bank_account", &self.nz_bank_account)?;
        s.serialize_field("oxxo", &self.oxxo)?;
        s.serialize_field("p24", &self.p24)?;
        s.serialize_field("parent", &self.parent)?;
        s.serialize_field("pay_by_bank", &self.pay_by_bank)?;
        s.serialize_field("payco", &self.payco)?;
        s.serialize_field("paynow", &self.paynow)?;
        s.serialize_field("paypal", &self.paypal)?;
        s.serialize_field("payto", &self.payto)?;
        s.serialize_field("pix", &self.pix)?;
        s.serialize_field("promptpay", &self.promptpay)?;
        s.serialize_field("revolut_pay", &self.revolut_pay)?;
        s.serialize_field("samsung_pay", &self.samsung_pay)?;
        s.serialize_field("satispay", &self.satispay)?;
        s.serialize_field("scalapay", &self.scalapay)?;
        s.serialize_field("sepa_debit", &self.sepa_debit)?;
        s.serialize_field("sofort", &self.sofort)?;
        s.serialize_field("sunbit", &self.sunbit)?;
        s.serialize_field("swish", &self.swish)?;
        s.serialize_field("twint", &self.twint)?;
        s.serialize_field("upi", &self.upi)?;
        s.serialize_field("us_bank_account", &self.us_bank_account)?;
        s.serialize_field("wechat_pay", &self.wechat_pay)?;
        s.serialize_field("zip", &self.zip)?;

        s.serialize_field("object", "payment_method_configuration")?;
        s.end()
    }
}
impl stripe_types::Object for PaymentMethodConfiguration {
    type Id = stripe_payment::PaymentMethodConfigurationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PaymentMethodConfigurationId);
