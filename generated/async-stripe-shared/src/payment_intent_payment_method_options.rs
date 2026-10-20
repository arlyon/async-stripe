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
    pub bizum: Option<stripe_shared::PaymentMethodOptionsBizum>,
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
    pub scalapay: Option<stripe_shared::PaymentMethodOptionsScalapay>,
    pub sepa_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodOptionsSofort>,
    pub sunbit: Option<stripe_shared::PaymentMethodOptionsSunbit>,
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
    acss_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsAcssDebit>,
    affirm: Option<stripe_shared::PaymentMethodOptionsAffirm>,
    afterpay_clearpay: Option<stripe_shared::PaymentMethodOptionsAfterpayClearpay>,
    alipay: Option<stripe_shared::PaymentMethodOptionsAlipay>,
    alma: Option<stripe_shared::PaymentMethodOptionsAlma>,
    amazon_pay: Option<stripe_shared::PaymentMethodOptionsAmazonPay>,
    au_becs_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsAuBecsDebit>,
    bacs_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsBacsDebit>,
    bancontact: Option<stripe_shared::PaymentMethodOptionsBancontact>,
    billie: Option<stripe_shared::PaymentMethodOptionsBillie>,
    bizum: Option<stripe_shared::PaymentMethodOptionsBizum>,
    blik: Option<stripe_shared::PaymentIntentPaymentMethodOptionsBlik>,
    boleto: Option<stripe_shared::PaymentMethodOptionsBoleto>,
    card: Option<stripe_shared::PaymentIntentPaymentMethodOptionsCard>,
    card_present: Option<stripe_shared::PaymentMethodOptionsCardPresent>,
    cashapp: Option<stripe_shared::PaymentMethodOptionsCashapp>,
    crypto: Option<stripe_shared::PaymentMethodOptionsCrypto>,
    customer_balance: Option<stripe_shared::PaymentMethodOptionsCustomerBalance>,
    eps: Option<stripe_shared::PaymentIntentPaymentMethodOptionsEps>,
    fpx: Option<stripe_shared::PaymentMethodOptionsFpx>,
    giropay: Option<stripe_shared::PaymentMethodOptionsGiropay>,
    grabpay: Option<stripe_shared::PaymentMethodOptionsGrabpay>,
    ideal: Option<stripe_shared::PaymentMethodOptionsIdeal>,
    interac_present: Option<stripe_shared::PaymentMethodOptionsInteracPresent>,
    kakao_pay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsKakaoPayPaymentMethodOptions>,
    klarna: Option<stripe_shared::PaymentMethodOptionsKlarna>,
    konbini: Option<stripe_shared::PaymentMethodOptionsKonbini>,
    kr_card: Option<stripe_shared::PaymentMethodOptionsKrCard>,
    link: Option<stripe_shared::PaymentIntentPaymentMethodOptionsLink>,
    mb_way: Option<stripe_shared::PaymentMethodOptionsMbWay>,
    mobilepay: Option<stripe_shared::PaymentIntentPaymentMethodOptionsMobilepay>,
    multibanco: Option<stripe_shared::PaymentMethodOptionsMultibanco>,
    naver_pay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsNaverPayPaymentMethodOptions>,
    nz_bank_account: Option<stripe_shared::PaymentIntentPaymentMethodOptionsNzBankAccount>,
    oxxo: Option<stripe_shared::PaymentMethodOptionsOxxo>,
    p24: Option<stripe_shared::PaymentMethodOptionsP24>,
    pay_by_bank: Option<stripe_shared::PaymentMethodOptionsPayByBank>,
    payco: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsPaycoPaymentMethodOptions>,
    paynow: Option<stripe_shared::PaymentMethodOptionsPaynow>,
    paypal: Option<stripe_shared::PaymentMethodOptionsPaypal>,
    payto: Option<stripe_shared::PaymentIntentPaymentMethodOptionsPayto>,
    pix: Option<stripe_shared::PaymentMethodOptionsPix>,
    promptpay: Option<stripe_shared::PaymentMethodOptionsPromptpay>,
    revolut_pay: Option<stripe_shared::PaymentMethodOptionsRevolutPay>,
    samsung_pay:
        Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsSamsungPayPaymentMethodOptions>,
    satispay: Option<stripe_shared::PaymentMethodOptionsSatispay>,
    scalapay: Option<stripe_shared::PaymentMethodOptionsScalapay>,
    sepa_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>,
    sofort: Option<stripe_shared::PaymentMethodOptionsSofort>,
    sunbit: Option<stripe_shared::PaymentMethodOptionsSunbit>,
    swish: Option<stripe_shared::PaymentIntentPaymentMethodOptionsSwish>,
    twint: Option<stripe_shared::PaymentMethodOptionsTwint>,
    upi: Option<stripe_shared::PaymentMethodOptionsUpi>,
    us_bank_account: Option<stripe_shared::PaymentIntentPaymentMethodOptionsUsBankAccount>,
    wechat_pay: Option<stripe_shared::PaymentMethodOptionsWechatPay>,
    zip: Option<stripe_shared::PaymentMethodOptionsZip>,
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
                "acss_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.acss_debit)
                }
                "affirm" => stripe_types::miniserde_helpers::begin_nullable(&mut self.affirm),
                "afterpay_clearpay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.afterpay_clearpay)
                }
                "alipay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.alipay),
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
                "bizum" => stripe_types::miniserde_helpers::begin_nullable(&mut self.bizum),
                "blik" => stripe_types::miniserde_helpers::begin_nullable(&mut self.blik),
                "boleto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.boleto),
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "card_present" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_present)
                }
                "cashapp" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp),
                "crypto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.crypto),
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
                alma: None,
                amazon_pay: None,
                au_becs_debit: None,
                bacs_debit: None,
                bancontact: None,
                billie: None,
                bizum: None,
                blik: None,
                boleto: None,
                card: None,
                card_present: None,
                cashapp: None,
                crypto: None,
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
                upi: None,
                us_bank_account: None,
                wechat_pay: None,
                zip: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                acss_debit,
                affirm,
                afterpay_clearpay,
                alipay,
                alma,
                amazon_pay,
                au_becs_debit,
                bacs_debit,
                bancontact,
                billie,
                bizum,
                blik,
                boleto,
                card,
                card_present,
                cashapp,
                crypto,
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
                upi,
                us_bank_account,
                wechat_pay,
                zip,
            ) = (
                self.acss_debit.take(),
                self.affirm.take(),
                self.afterpay_clearpay.take(),
                self.alipay.take(),
                self.alma.take(),
                self.amazon_pay.take(),
                self.au_becs_debit.take(),
                self.bacs_debit.take(),
                self.bancontact.take(),
                self.billie.take(),
                self.bizum,
                self.blik.take(),
                self.boleto.take(),
                self.card.take(),
                self.card_present.take(),
                self.cashapp.take(),
                self.crypto.take(),
                self.customer_balance.take(),
                self.eps.take(),
                self.fpx.take(),
                self.giropay.take(),
                self.grabpay.take(),
                self.ideal.take(),
                self.interac_present,
                self.kakao_pay.take(),
                self.klarna.take(),
                self.konbini.take(),
                self.kr_card.take(),
                self.link.take(),
                self.mb_way.take(),
                self.mobilepay.take(),
                self.multibanco.take(),
                self.naver_pay.take(),
                self.nz_bank_account.take(),
                self.oxxo.take(),
                self.p24.take(),
                self.pay_by_bank,
                self.payco.take(),
                self.paynow.take(),
                self.paypal.take(),
                self.payto.take(),
                self.pix.take(),
                self.promptpay.take(),
                self.revolut_pay.take(),
                self.samsung_pay.take(),
                self.satispay.take(),
                self.scalapay.take(),
                self.sepa_debit.take(),
                self.sofort.take(),
                self.sunbit.take(),
                self.swish.take(),
                self.twint.take(),
                self.upi.take(),
                self.us_bank_account.take(),
                self.wechat_pay.take(),
                self.zip.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                acss_debit,
                affirm,
                afterpay_clearpay,
                alipay,
                alma,
                amazon_pay,
                au_becs_debit,
                bacs_debit,
                bancontact,
                billie,
                bizum,
                blik,
                boleto,
                card,
                card_present,
                cashapp,
                crypto,
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
                upi,
                us_bank_account,
                wechat_pay,
                zip,
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

    impl ObjectDeser for PaymentIntentPaymentMethodOptions {
        type Builder = PaymentIntentPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for PaymentIntentPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
