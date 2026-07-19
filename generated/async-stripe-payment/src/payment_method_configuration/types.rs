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
    pub sepa_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sofort: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
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
    acss_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    active: Option<bool>,
    affirm: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    afterpay_clearpay:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    alipay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    alma: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    amazon_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    apple_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    application: Option<Option<String>>,
    au_becs_debit:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    bacs_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    bancontact: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    billie: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    blik: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    boleto: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    card: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    cartes_bancaires:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    cashapp: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    crypto: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    customer_balance:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    eps: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    fpx: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    giropay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    google_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    grabpay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    id: Option<stripe_payment::PaymentMethodConfigurationId>,
    ideal: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    is_default: Option<bool>,
    jcb: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    kakao_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    klarna: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    konbini: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    kr_card: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    link: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    livemode: Option<bool>,
    mb_way: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    mobilepay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    multibanco: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    name: Option<String>,
    naver_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    nz_bank_account:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    oxxo: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    p24: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    parent: Option<Option<String>>,
    pay_by_bank: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    payco: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    paynow: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    paypal: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    payto: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    pix: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    promptpay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    revolut_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    samsung_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    satispay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    sepa_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    sofort: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    swish: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    twint: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    upi: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    us_bank_account:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    wechat_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    zip: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
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
                builder: PaymentMethodConfigurationBuilder {
                    acss_debit: Deserialize::default(),
                    active: Deserialize::default(),
                    affirm: Deserialize::default(),
                    afterpay_clearpay: Deserialize::default(),
                    alipay: Deserialize::default(),
                    alma: Deserialize::default(),
                    amazon_pay: Deserialize::default(),
                    apple_pay: Deserialize::default(),
                    application: Deserialize::default(),
                    au_becs_debit: Deserialize::default(),
                    bacs_debit: Deserialize::default(),
                    bancontact: Deserialize::default(),
                    billie: Deserialize::default(),
                    blik: Deserialize::default(),
                    boleto: Deserialize::default(),
                    card: Deserialize::default(),
                    cartes_bancaires: Deserialize::default(),
                    cashapp: Deserialize::default(),
                    crypto: Deserialize::default(),
                    customer_balance: Deserialize::default(),
                    eps: Deserialize::default(),
                    fpx: Deserialize::default(),
                    giropay: Deserialize::default(),
                    google_pay: Deserialize::default(),
                    grabpay: Deserialize::default(),
                    id: Deserialize::default(),
                    ideal: Deserialize::default(),
                    is_default: Deserialize::default(),
                    jcb: Deserialize::default(),
                    kakao_pay: Deserialize::default(),
                    klarna: Deserialize::default(),
                    konbini: Deserialize::default(),
                    kr_card: Deserialize::default(),
                    link: Deserialize::default(),
                    livemode: Deserialize::default(),
                    mb_way: Deserialize::default(),
                    mobilepay: Deserialize::default(),
                    multibanco: Deserialize::default(),
                    name: Deserialize::default(),
                    naver_pay: Deserialize::default(),
                    nz_bank_account: Deserialize::default(),
                    oxxo: Deserialize::default(),
                    p24: Deserialize::default(),
                    parent: Deserialize::default(),
                    pay_by_bank: Deserialize::default(),
                    payco: Deserialize::default(),
                    paynow: Deserialize::default(),
                    paypal: Deserialize::default(),
                    payto: Deserialize::default(),
                    pix: Deserialize::default(),
                    promptpay: Deserialize::default(),
                    revolut_pay: Deserialize::default(),
                    samsung_pay: Deserialize::default(),
                    satispay: Deserialize::default(),
                    sepa_debit: Deserialize::default(),
                    sofort: Deserialize::default(),
                    swish: Deserialize::default(),
                    twint: Deserialize::default(),
                    upi: Deserialize::default(),
                    us_bank_account: Deserialize::default(),
                    wechat_pay: Deserialize::default(),
                    zip: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.builder.acss_debit),
                "active" => Deserialize::begin(&mut self.builder.active),
                "affirm" => Deserialize::begin(&mut self.builder.affirm),
                "afterpay_clearpay" => Deserialize::begin(&mut self.builder.afterpay_clearpay),
                "alipay" => Deserialize::begin(&mut self.builder.alipay),
                "alma" => Deserialize::begin(&mut self.builder.alma),
                "amazon_pay" => Deserialize::begin(&mut self.builder.amazon_pay),
                "apple_pay" => Deserialize::begin(&mut self.builder.apple_pay),
                "application" => Deserialize::begin(&mut self.builder.application),
                "au_becs_debit" => Deserialize::begin(&mut self.builder.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.builder.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.builder.bancontact),
                "billie" => Deserialize::begin(&mut self.builder.billie),
                "blik" => Deserialize::begin(&mut self.builder.blik),
                "boleto" => Deserialize::begin(&mut self.builder.boleto),
                "card" => Deserialize::begin(&mut self.builder.card),
                "cartes_bancaires" => Deserialize::begin(&mut self.builder.cartes_bancaires),
                "cashapp" => Deserialize::begin(&mut self.builder.cashapp),
                "crypto" => Deserialize::begin(&mut self.builder.crypto),
                "customer_balance" => Deserialize::begin(&mut self.builder.customer_balance),
                "eps" => Deserialize::begin(&mut self.builder.eps),
                "fpx" => Deserialize::begin(&mut self.builder.fpx),
                "giropay" => Deserialize::begin(&mut self.builder.giropay),
                "google_pay" => Deserialize::begin(&mut self.builder.google_pay),
                "grabpay" => Deserialize::begin(&mut self.builder.grabpay),
                "id" => Deserialize::begin(&mut self.builder.id),
                "ideal" => Deserialize::begin(&mut self.builder.ideal),
                "is_default" => Deserialize::begin(&mut self.builder.is_default),
                "jcb" => Deserialize::begin(&mut self.builder.jcb),
                "kakao_pay" => Deserialize::begin(&mut self.builder.kakao_pay),
                "klarna" => Deserialize::begin(&mut self.builder.klarna),
                "konbini" => Deserialize::begin(&mut self.builder.konbini),
                "kr_card" => Deserialize::begin(&mut self.builder.kr_card),
                "link" => Deserialize::begin(&mut self.builder.link),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "mb_way" => Deserialize::begin(&mut self.builder.mb_way),
                "mobilepay" => Deserialize::begin(&mut self.builder.mobilepay),
                "multibanco" => Deserialize::begin(&mut self.builder.multibanco),
                "name" => Deserialize::begin(&mut self.builder.name),
                "naver_pay" => Deserialize::begin(&mut self.builder.naver_pay),
                "nz_bank_account" => Deserialize::begin(&mut self.builder.nz_bank_account),
                "oxxo" => Deserialize::begin(&mut self.builder.oxxo),
                "p24" => Deserialize::begin(&mut self.builder.p24),
                "parent" => Deserialize::begin(&mut self.builder.parent),
                "pay_by_bank" => Deserialize::begin(&mut self.builder.pay_by_bank),
                "payco" => Deserialize::begin(&mut self.builder.payco),
                "paynow" => Deserialize::begin(&mut self.builder.paynow),
                "paypal" => Deserialize::begin(&mut self.builder.paypal),
                "payto" => Deserialize::begin(&mut self.builder.payto),
                "pix" => Deserialize::begin(&mut self.builder.pix),
                "promptpay" => Deserialize::begin(&mut self.builder.promptpay),
                "revolut_pay" => Deserialize::begin(&mut self.builder.revolut_pay),
                "samsung_pay" => Deserialize::begin(&mut self.builder.samsung_pay),
                "satispay" => Deserialize::begin(&mut self.builder.satispay),
                "sepa_debit" => Deserialize::begin(&mut self.builder.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.builder.sofort),
                "swish" => Deserialize::begin(&mut self.builder.swish),
                "twint" => Deserialize::begin(&mut self.builder.twint),
                "upi" => Deserialize::begin(&mut self.builder.upi),
                "us_bank_account" => Deserialize::begin(&mut self.builder.us_bank_account),
                "wechat_pay" => Deserialize::begin(&mut self.builder.wechat_pay),
                "zip" => Deserialize::begin(&mut self.builder.zip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(acss_debit),
                Some(active),
                Some(affirm),
                Some(afterpay_clearpay),
                Some(alipay),
                Some(alma),
                Some(amazon_pay),
                Some(apple_pay),
                Some(application),
                Some(au_becs_debit),
                Some(bacs_debit),
                Some(bancontact),
                Some(billie),
                Some(blik),
                Some(boleto),
                Some(card),
                Some(cartes_bancaires),
                Some(cashapp),
                Some(crypto),
                Some(customer_balance),
                Some(eps),
                Some(fpx),
                Some(giropay),
                Some(google_pay),
                Some(grabpay),
                Some(id),
                Some(ideal),
                Some(is_default),
                Some(jcb),
                Some(kakao_pay),
                Some(klarna),
                Some(konbini),
                Some(kr_card),
                Some(link),
                Some(livemode),
                Some(mb_way),
                Some(mobilepay),
                Some(multibanco),
                Some(name),
                Some(naver_pay),
                Some(nz_bank_account),
                Some(oxxo),
                Some(p24),
                Some(parent),
                Some(pay_by_bank),
                Some(payco),
                Some(paynow),
                Some(paypal),
                Some(payto),
                Some(pix),
                Some(promptpay),
                Some(revolut_pay),
                Some(samsung_pay),
                Some(satispay),
                Some(sepa_debit),
                Some(sofort),
                Some(swish),
                Some(twint),
                Some(upi),
                Some(us_bank_account),
                Some(wechat_pay),
                Some(zip),
            ) = (
                self.builder.acss_debit.take(),
                self.builder.active,
                self.builder.affirm.take(),
                self.builder.afterpay_clearpay.take(),
                self.builder.alipay.take(),
                self.builder.alma.take(),
                self.builder.amazon_pay.take(),
                self.builder.apple_pay.take(),
                self.builder.application.take(),
                self.builder.au_becs_debit.take(),
                self.builder.bacs_debit.take(),
                self.builder.bancontact.take(),
                self.builder.billie.take(),
                self.builder.blik.take(),
                self.builder.boleto.take(),
                self.builder.card.take(),
                self.builder.cartes_bancaires.take(),
                self.builder.cashapp.take(),
                self.builder.crypto.take(),
                self.builder.customer_balance.take(),
                self.builder.eps.take(),
                self.builder.fpx.take(),
                self.builder.giropay.take(),
                self.builder.google_pay.take(),
                self.builder.grabpay.take(),
                self.builder.id.take(),
                self.builder.ideal.take(),
                self.builder.is_default,
                self.builder.jcb.take(),
                self.builder.kakao_pay.take(),
                self.builder.klarna.take(),
                self.builder.konbini.take(),
                self.builder.kr_card.take(),
                self.builder.link.take(),
                self.builder.livemode,
                self.builder.mb_way.take(),
                self.builder.mobilepay.take(),
                self.builder.multibanco.take(),
                self.builder.name.take(),
                self.builder.naver_pay.take(),
                self.builder.nz_bank_account.take(),
                self.builder.oxxo.take(),
                self.builder.p24.take(),
                self.builder.parent.take(),
                self.builder.pay_by_bank.take(),
                self.builder.payco.take(),
                self.builder.paynow.take(),
                self.builder.paypal.take(),
                self.builder.payto.take(),
                self.builder.pix.take(),
                self.builder.promptpay.take(),
                self.builder.revolut_pay.take(),
                self.builder.samsung_pay.take(),
                self.builder.satispay.take(),
                self.builder.sepa_debit.take(),
                self.builder.sofort.take(),
                self.builder.swish.take(),
                self.builder.twint.take(),
                self.builder.upi.take(),
                self.builder.us_bank_account.take(),
                self.builder.wechat_pay.take(),
                self.builder.zip.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethodConfiguration {
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
                sepa_debit,
                sofort,
                swish,
                twint,
                upi,
                us_bank_account,
                wechat_pay,
                zip,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodConfiguration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentMethodConfiguration", 63)?;
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
        s.serialize_field("sepa_debit", &self.sepa_debit)?;
        s.serialize_field("sofort", &self.sofort)?;
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
