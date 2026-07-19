#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodOptionsAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodOptionsAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentMethodOptionsAlipay>,
    pub alma: Option<stripe_shared::PaymentMethodOptionsAlma>,
    pub amazon_pay: Option<stripe_shared::PaymentMethodOptionsAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodOptionsBancontact>,
    pub billie: Option<stripe_shared::PaymentMethodOptionsBillie>,
    pub blik: Option<stripe_shared::PaymentIntentPaymentMethodOptionsBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodOptionsBoleto>,
    pub card: Option<stripe_shared::PaymentIntentPaymentMethodOptionsCard>,
    pub card_present: Option<stripe_shared::PaymentMethodOptionsCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodOptionsCashapp>,
    pub crypto: Option<stripe_shared::PaymentMethodOptionsCrypto>,
    pub customer_balance: Option<stripe_shared::PaymentMethodOptionsCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentIntentPaymentMethodOptionsEps>,
    pub fpx: Option<stripe_shared::PaymentMethodOptionsFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodOptionsGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodOptionsGrabpay>,
    pub ideal: Option<stripe_shared::PaymentMethodOptionsIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodOptionsInteracPresent>,
    pub kakao_pay:
        Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptions>,
    pub klarna: Option<stripe_shared::PaymentMethodOptionsKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodOptionsKonbini>,
    pub kr_card: Option<stripe_shared::PaymentMethodOptionsKrCard>,
    pub link: Option<stripe_shared::PaymentIntentPaymentMethodOptionsLink>,
    pub mb_way: Option<stripe_shared::PaymentMethodOptionsMbWay>,
    pub mobilepay: Option<stripe_shared::PaymentIntentPaymentMethodOptionsMobilepay>,
    pub multibanco: Option<stripe_shared::PaymentMethodOptionsMultibanco>,
    pub naver_pay:
        Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions>,
    pub nz_bank_account: Option<stripe_shared::PaymentIntentPaymentMethodOptionsNzBankAccount>,
    pub oxxo: Option<stripe_shared::PaymentMethodOptionsOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodOptionsP24>,
    pub pay_by_bank: Option<stripe_shared::PaymentMethodOptionsPayByBank>,
    pub payco: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptions>,
    pub paynow: Option<stripe_shared::PaymentMethodOptionsPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodOptionsPaypal>,
    pub payto: Option<stripe_shared::PaymentIntentPaymentMethodOptionsPayto>,
    pub pix: Option<stripe_shared::PaymentMethodOptionsPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodOptionsPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodOptionsRevolutPay>,
    pub samsung_pay:
        Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsSamsungPayPaymentMethodOptions>,
    pub satispay: Option<stripe_shared::PaymentMethodOptionsSatispay>,
    pub sepa_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodOptionsSofort>,
    pub swish: Option<stripe_shared::PaymentIntentPaymentMethodOptionsSwish>,
    pub twint: Option<stripe_shared::PaymentMethodOptionsTwint>,
    pub upi: Option<stripe_shared::PaymentMethodOptionsUpi>,
    pub us_bank_account: Option<stripe_shared::PaymentIntentPaymentMethodOptionsUsBankAccount>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodOptionsWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodOptionsZip>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentIntentPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentIntentPaymentMethodOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentIntentPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsAcssDebit>>,
    affirm: Option<Option<stripe_shared::PaymentMethodOptionsAffirm>>,
    afterpay_clearpay: Option<Option<stripe_shared::PaymentMethodOptionsAfterpayClearpay>>,
    alipay: Option<Option<stripe_shared::PaymentMethodOptionsAlipay>>,
    alma: Option<Option<stripe_shared::PaymentMethodOptionsAlma>>,
    amazon_pay: Option<Option<stripe_shared::PaymentMethodOptionsAmazonPay>>,
    au_becs_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsBacsDebit>>,
    bancontact: Option<Option<stripe_shared::PaymentMethodOptionsBancontact>>,
    billie: Option<Option<stripe_shared::PaymentMethodOptionsBillie>>,
    blik: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsBlik>>,
    boleto: Option<Option<stripe_shared::PaymentMethodOptionsBoleto>>,
    card: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsCard>>,
    card_present: Option<Option<stripe_shared::PaymentMethodOptionsCardPresent>>,
    cashapp: Option<Option<stripe_shared::PaymentMethodOptionsCashapp>>,
    crypto: Option<Option<stripe_shared::PaymentMethodOptionsCrypto>>,
    customer_balance: Option<Option<stripe_shared::PaymentMethodOptionsCustomerBalance>>,
    eps: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsEps>>,
    fpx: Option<Option<stripe_shared::PaymentMethodOptionsFpx>>,
    giropay: Option<Option<stripe_shared::PaymentMethodOptionsGiropay>>,
    grabpay: Option<Option<stripe_shared::PaymentMethodOptionsGrabpay>>,
    ideal: Option<Option<stripe_shared::PaymentMethodOptionsIdeal>>,
    interac_present: Option<Option<stripe_shared::PaymentMethodOptionsInteracPresent>>,
    kakao_pay: Option<
        Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptions>,
    >,
    klarna: Option<Option<stripe_shared::PaymentMethodOptionsKlarna>>,
    konbini: Option<Option<stripe_shared::PaymentMethodOptionsKonbini>>,
    kr_card: Option<Option<stripe_shared::PaymentMethodOptionsKrCard>>,
    link: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsLink>>,
    mb_way: Option<Option<stripe_shared::PaymentMethodOptionsMbWay>>,
    mobilepay: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsMobilepay>>,
    multibanco: Option<Option<stripe_shared::PaymentMethodOptionsMultibanco>>,
    naver_pay: Option<
        Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions>,
    >,
    nz_bank_account: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsNzBankAccount>>,
    oxxo: Option<Option<stripe_shared::PaymentMethodOptionsOxxo>>,
    p24: Option<Option<stripe_shared::PaymentMethodOptionsP24>>,
    pay_by_bank: Option<Option<stripe_shared::PaymentMethodOptionsPayByBank>>,
    payco:
        Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptions>>,
    paynow: Option<Option<stripe_shared::PaymentMethodOptionsPaynow>>,
    paypal: Option<Option<stripe_shared::PaymentMethodOptionsPaypal>>,
    payto: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsPayto>>,
    pix: Option<Option<stripe_shared::PaymentMethodOptionsPix>>,
    promptpay: Option<Option<stripe_shared::PaymentMethodOptionsPromptpay>>,
    revolut_pay: Option<Option<stripe_shared::PaymentMethodOptionsRevolutPay>>,
    samsung_pay: Option<
        Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsSamsungPayPaymentMethodOptions>,
    >,
    satispay: Option<Option<stripe_shared::PaymentMethodOptionsSatispay>>,
    sepa_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>>,
    sofort: Option<Option<stripe_shared::PaymentMethodOptionsSofort>>,
    swish: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsSwish>>,
    twint: Option<Option<stripe_shared::PaymentMethodOptionsTwint>>,
    upi: Option<Option<stripe_shared::PaymentMethodOptionsUpi>>,
    us_bank_account: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsUsBankAccount>>,
    wechat_pay: Option<Option<stripe_shared::PaymentMethodOptionsWechatPay>>,
    zip: Option<Option<stripe_shared::PaymentMethodOptionsZip>>,
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

    impl Deserialize for PaymentIntentPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentIntentPaymentMethodOptions>,
        builder: PaymentIntentPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<PaymentIntentPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentIntentPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentIntentPaymentMethodOptionsBuilder {
        type Out = PaymentIntentPaymentMethodOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "affirm" => Deserialize::begin(&mut self.affirm),
                "afterpay_clearpay" => Deserialize::begin(&mut self.afterpay_clearpay),
                "alipay" => Deserialize::begin(&mut self.alipay),
                "alma" => Deserialize::begin(&mut self.alma),
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.bancontact),
                "billie" => Deserialize::begin(&mut self.billie),
                "blik" => Deserialize::begin(&mut self.blik),
                "boleto" => Deserialize::begin(&mut self.boleto),
                "card" => Deserialize::begin(&mut self.card),
                "card_present" => Deserialize::begin(&mut self.card_present),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "crypto" => Deserialize::begin(&mut self.crypto),
                "customer_balance" => Deserialize::begin(&mut self.customer_balance),
                "eps" => Deserialize::begin(&mut self.eps),
                "fpx" => Deserialize::begin(&mut self.fpx),
                "giropay" => Deserialize::begin(&mut self.giropay),
                "grabpay" => Deserialize::begin(&mut self.grabpay),
                "ideal" => Deserialize::begin(&mut self.ideal),
                "interac_present" => Deserialize::begin(&mut self.interac_present),
                "kakao_pay" => Deserialize::begin(&mut self.kakao_pay),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "konbini" => Deserialize::begin(&mut self.konbini),
                "kr_card" => Deserialize::begin(&mut self.kr_card),
                "link" => Deserialize::begin(&mut self.link),
                "mb_way" => Deserialize::begin(&mut self.mb_way),
                "mobilepay" => Deserialize::begin(&mut self.mobilepay),
                "multibanco" => Deserialize::begin(&mut self.multibanco),
                "naver_pay" => Deserialize::begin(&mut self.naver_pay),
                "nz_bank_account" => Deserialize::begin(&mut self.nz_bank_account),
                "oxxo" => Deserialize::begin(&mut self.oxxo),
                "p24" => Deserialize::begin(&mut self.p24),
                "pay_by_bank" => Deserialize::begin(&mut self.pay_by_bank),
                "payco" => Deserialize::begin(&mut self.payco),
                "paynow" => Deserialize::begin(&mut self.paynow),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "payto" => Deserialize::begin(&mut self.payto),
                "pix" => Deserialize::begin(&mut self.pix),
                "promptpay" => Deserialize::begin(&mut self.promptpay),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "samsung_pay" => Deserialize::begin(&mut self.samsung_pay),
                "satispay" => Deserialize::begin(&mut self.satispay),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
                "swish" => Deserialize::begin(&mut self.swish),
                "twint" => Deserialize::begin(&mut self.twint),
                "upi" => Deserialize::begin(&mut self.upi),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),
                "wechat_pay" => Deserialize::begin(&mut self.wechat_pay),
                "zip" => Deserialize::begin(&mut self.zip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Some(None),
                affirm: Some(None),
                afterpay_clearpay: Some(None),
                alipay: Some(None),
                alma: Some(None),
                amazon_pay: Some(None),
                au_becs_debit: Some(None),
                bacs_debit: Some(None),
                bancontact: Some(None),
                billie: Some(None),
                blik: Some(None),
                boleto: Some(None),
                card: Some(None),
                card_present: Some(None),
                cashapp: Some(None),
                crypto: Some(None),
                customer_balance: Some(None),
                eps: Some(None),
                fpx: Some(None),
                giropay: Some(None),
                grabpay: Some(None),
                ideal: Some(None),
                interac_present: Some(None),
                kakao_pay: Some(None),
                klarna: Some(None),
                konbini: Some(None),
                kr_card: Some(None),
                link: Some(None),
                mb_way: Some(None),
                mobilepay: Some(None),
                multibanco: Some(None),
                naver_pay: Some(None),
                nz_bank_account: Some(None),
                oxxo: Some(None),
                p24: Some(None),
                pay_by_bank: Some(None),
                payco: Some(None),
                paynow: Some(None),
                paypal: Some(None),
                payto: Some(None),
                pix: Some(None),
                promptpay: Some(None),
                revolut_pay: Some(None),
                samsung_pay: Some(None),
                satispay: Some(None),
                sepa_debit: Some(None),
                sofort: Some(None),
                swish: Some(None),
                twint: Some(None),
                upi: Some(None),
                us_bank_account: Some(None),
                wechat_pay: Some(None),
                zip: Some(None),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                acss_debit: self.acss_debit.take().flatten(),
                affirm: self.affirm.take().flatten(),
                afterpay_clearpay: self.afterpay_clearpay.take().flatten(),
                alipay: self.alipay.take().flatten(),
                alma: self.alma.take().flatten(),
                amazon_pay: self.amazon_pay.take().flatten(),
                au_becs_debit: self.au_becs_debit.take().flatten(),
                bacs_debit: self.bacs_debit.take().flatten(),
                bancontact: self.bancontact.take().flatten(),
                billie: self.billie.take().flatten(),
                blik: self.blik.take().flatten(),
                boleto: self.boleto.take().flatten(),
                card: self.card.take().flatten(),
                card_present: self.card_present.take().flatten(),
                cashapp: self.cashapp.take().flatten(),
                crypto: self.crypto.take().flatten(),
                customer_balance: self.customer_balance.take().flatten(),
                eps: self.eps.take().flatten(),
                fpx: self.fpx.take().flatten(),
                giropay: self.giropay.take().flatten(),
                grabpay: self.grabpay.take().flatten(),
                ideal: self.ideal.take().flatten(),
                interac_present: self.interac_present.flatten(),
                kakao_pay: self.kakao_pay.take().flatten(),
                klarna: self.klarna.take().flatten(),
                konbini: self.konbini.take().flatten(),
                kr_card: self.kr_card.take().flatten(),
                link: self.link.take().flatten(),
                mb_way: self.mb_way.take().flatten(),
                mobilepay: self.mobilepay.take().flatten(),
                multibanco: self.multibanco.take().flatten(),
                naver_pay: self.naver_pay.take().flatten(),
                nz_bank_account: self.nz_bank_account.take().flatten(),
                oxxo: self.oxxo.take().flatten(),
                p24: self.p24.take().flatten(),
                pay_by_bank: self.pay_by_bank.flatten(),
                payco: self.payco.take().flatten(),
                paynow: self.paynow.take().flatten(),
                paypal: self.paypal.take().flatten(),
                payto: self.payto.take().flatten(),
                pix: self.pix.take().flatten(),
                promptpay: self.promptpay.take().flatten(),
                revolut_pay: self.revolut_pay.take().flatten(),
                samsung_pay: self.samsung_pay.take().flatten(),
                satispay: self.satispay.take().flatten(),
                sepa_debit: self.sepa_debit.take().flatten(),
                sofort: self.sofort.take().flatten(),
                swish: self.swish.take().flatten(),
                twint: self.twint.take().flatten(),
                upi: self.upi.take().flatten(),
                us_bank_account: self.us_bank_account.take().flatten(),
                wechat_pay: self.wechat_pay.take().flatten(),
                zip: self.zip.take().flatten(),
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

    impl ObjectDeser for PaymentIntentPaymentMethodOptions {
        type Builder = PaymentIntentPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for PaymentIntentPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentIntentPaymentMethodOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit" => b.acss_debit = FromValueOpt::from_value(v),
                    "affirm" => b.affirm = FromValueOpt::from_value(v),
                    "afterpay_clearpay" => b.afterpay_clearpay = FromValueOpt::from_value(v),
                    "alipay" => b.alipay = FromValueOpt::from_value(v),
                    "alma" => b.alma = FromValueOpt::from_value(v),
                    "amazon_pay" => b.amazon_pay = FromValueOpt::from_value(v),
                    "au_becs_debit" => b.au_becs_debit = FromValueOpt::from_value(v),
                    "bacs_debit" => b.bacs_debit = FromValueOpt::from_value(v),
                    "bancontact" => b.bancontact = FromValueOpt::from_value(v),
                    "billie" => b.billie = FromValueOpt::from_value(v),
                    "blik" => b.blik = FromValueOpt::from_value(v),
                    "boleto" => b.boleto = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "card_present" => b.card_present = FromValueOpt::from_value(v),
                    "cashapp" => b.cashapp = FromValueOpt::from_value(v),
                    "crypto" => b.crypto = FromValueOpt::from_value(v),
                    "customer_balance" => b.customer_balance = FromValueOpt::from_value(v),
                    "eps" => b.eps = FromValueOpt::from_value(v),
                    "fpx" => b.fpx = FromValueOpt::from_value(v),
                    "giropay" => b.giropay = FromValueOpt::from_value(v),
                    "grabpay" => b.grabpay = FromValueOpt::from_value(v),
                    "ideal" => b.ideal = FromValueOpt::from_value(v),
                    "interac_present" => b.interac_present = FromValueOpt::from_value(v),
                    "kakao_pay" => b.kakao_pay = FromValueOpt::from_value(v),
                    "klarna" => b.klarna = FromValueOpt::from_value(v),
                    "konbini" => b.konbini = FromValueOpt::from_value(v),
                    "kr_card" => b.kr_card = FromValueOpt::from_value(v),
                    "link" => b.link = FromValueOpt::from_value(v),
                    "mb_way" => b.mb_way = FromValueOpt::from_value(v),
                    "mobilepay" => b.mobilepay = FromValueOpt::from_value(v),
                    "multibanco" => b.multibanco = FromValueOpt::from_value(v),
                    "naver_pay" => b.naver_pay = FromValueOpt::from_value(v),
                    "nz_bank_account" => b.nz_bank_account = FromValueOpt::from_value(v),
                    "oxxo" => b.oxxo = FromValueOpt::from_value(v),
                    "p24" => b.p24 = FromValueOpt::from_value(v),
                    "pay_by_bank" => b.pay_by_bank = FromValueOpt::from_value(v),
                    "payco" => b.payco = FromValueOpt::from_value(v),
                    "paynow" => b.paynow = FromValueOpt::from_value(v),
                    "paypal" => b.paypal = FromValueOpt::from_value(v),
                    "payto" => b.payto = FromValueOpt::from_value(v),
                    "pix" => b.pix = FromValueOpt::from_value(v),
                    "promptpay" => b.promptpay = FromValueOpt::from_value(v),
                    "revolut_pay" => b.revolut_pay = FromValueOpt::from_value(v),
                    "samsung_pay" => b.samsung_pay = FromValueOpt::from_value(v),
                    "satispay" => b.satispay = FromValueOpt::from_value(v),
                    "sepa_debit" => b.sepa_debit = FromValueOpt::from_value(v),
                    "sofort" => b.sofort = FromValueOpt::from_value(v),
                    "swish" => b.swish = FromValueOpt::from_value(v),
                    "twint" => b.twint = FromValueOpt::from_value(v),
                    "upi" => b.upi = FromValueOpt::from_value(v),
                    "us_bank_account" => b.us_bank_account = FromValueOpt::from_value(v),
                    "wechat_pay" => b.wechat_pay = FromValueOpt::from_value(v),
                    "zip" => b.zip = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
