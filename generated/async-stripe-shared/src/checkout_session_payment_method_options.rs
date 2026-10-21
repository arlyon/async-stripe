#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutSessionPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::CheckoutAcssDebitPaymentMethodOptions>,
    pub affirm: Option<stripe_shared::CheckoutAffirmPaymentMethodOptions>,
    pub afterpay_clearpay: Option<stripe_shared::CheckoutAfterpayClearpayPaymentMethodOptions>,
    pub alipay: Option<stripe_shared::CheckoutAlipayPaymentMethodOptions>,
    pub alma: Option<stripe_shared::CheckoutAlmaPaymentMethodOptions>,
    pub amazon_pay: Option<stripe_shared::CheckoutAmazonPayPaymentMethodOptions>,
    pub au_becs_debit: Option<stripe_shared::CheckoutAuBecsDebitPaymentMethodOptions>,
    pub bacs_debit: Option<stripe_shared::CheckoutBacsDebitPaymentMethodOptions>,
    pub bancontact: Option<stripe_shared::CheckoutBancontactPaymentMethodOptions>,
    pub billie: Option<stripe_shared::CheckoutBilliePaymentMethodOptions>,
    pub boleto: Option<stripe_shared::CheckoutBoletoPaymentMethodOptions>,
    pub card: Option<stripe_shared::CheckoutCardPaymentMethodOptions>,
    pub cashapp: Option<stripe_shared::CheckoutCashappPaymentMethodOptions>,
    pub customer_balance: Option<stripe_shared::CheckoutCustomerBalancePaymentMethodOptions>,
    pub eps: Option<stripe_shared::CheckoutEpsPaymentMethodOptions>,
    pub fpx: Option<stripe_shared::CheckoutFpxPaymentMethodOptions>,
    pub giropay: Option<stripe_shared::CheckoutGiropayPaymentMethodOptions>,
    pub grabpay: Option<stripe_shared::CheckoutGrabPayPaymentMethodOptions>,
    pub ideal: Option<stripe_shared::CheckoutIdealPaymentMethodOptions>,
    pub kakao_pay: Option<stripe_shared::CheckoutKakaoPayPaymentMethodOptions>,
    pub klarna: Option<stripe_shared::CheckoutKlarnaPaymentMethodOptions>,
    pub konbini: Option<stripe_shared::CheckoutKonbiniPaymentMethodOptions>,
    pub kr_card: Option<stripe_shared::CheckoutKrCardPaymentMethodOptions>,
    pub link: Option<stripe_shared::CheckoutLinkPaymentMethodOptions>,
    pub mobilepay: Option<stripe_shared::CheckoutMobilepayPaymentMethodOptions>,
    pub multibanco: Option<stripe_shared::CheckoutMultibancoPaymentMethodOptions>,
    pub naver_pay: Option<stripe_shared::CheckoutNaverPayPaymentMethodOptions>,
    pub oxxo: Option<stripe_shared::CheckoutOxxoPaymentMethodOptions>,
    pub p24: Option<stripe_shared::CheckoutP24PaymentMethodOptions>,
    pub payco: Option<stripe_shared::CheckoutPaycoPaymentMethodOptions>,
    pub paynow: Option<stripe_shared::CheckoutPaynowPaymentMethodOptions>,
    pub paypal: Option<stripe_shared::CheckoutPaypalPaymentMethodOptions>,
    pub payto: Option<stripe_shared::CheckoutPaytoPaymentMethodOptions>,
    pub pix: Option<stripe_shared::CheckoutPixPaymentMethodOptions>,
    pub revolut_pay: Option<stripe_shared::CheckoutRevolutPayPaymentMethodOptions>,
    pub samsung_pay: Option<stripe_shared::CheckoutSamsungPayPaymentMethodOptions>,
    pub satispay: Option<stripe_shared::CheckoutSatispayPaymentMethodOptions>,
    pub scalapay: Option<stripe_shared::CheckoutScalapayPaymentMethodOptions>,
    pub sepa_debit: Option<stripe_shared::CheckoutSepaDebitPaymentMethodOptions>,
    pub sofort: Option<stripe_shared::CheckoutSofortPaymentMethodOptions>,
    pub sunbit: Option<stripe_shared::CheckoutSunbitPaymentMethodOptions>,
    pub swish: Option<stripe_shared::CheckoutSwishPaymentMethodOptions>,
    pub twint: Option<stripe_shared::CheckoutTwintPaymentMethodOptions>,
    pub upi: Option<stripe_shared::CheckoutUpiPaymentMethodOptions>,
    pub us_bank_account: Option<stripe_shared::CheckoutUsBankAccountPaymentMethodOptions>,
    pub wechat_pay: Option<stripe_shared::CheckoutWechatPayPaymentMethodOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CheckoutSessionPaymentMethodOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CheckoutSessionPaymentMethodOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CheckoutSessionPaymentMethodOptionsBuilder {
    acss_debit: Option<stripe_shared::CheckoutAcssDebitPaymentMethodOptions>,
    affirm: Option<stripe_shared::CheckoutAffirmPaymentMethodOptions>,
    afterpay_clearpay: Option<stripe_shared::CheckoutAfterpayClearpayPaymentMethodOptions>,
    alipay: Option<stripe_shared::CheckoutAlipayPaymentMethodOptions>,
    alma: Option<stripe_shared::CheckoutAlmaPaymentMethodOptions>,
    amazon_pay: Option<stripe_shared::CheckoutAmazonPayPaymentMethodOptions>,
    au_becs_debit: Option<stripe_shared::CheckoutAuBecsDebitPaymentMethodOptions>,
    bacs_debit: Option<stripe_shared::CheckoutBacsDebitPaymentMethodOptions>,
    bancontact: Option<stripe_shared::CheckoutBancontactPaymentMethodOptions>,
    billie: Option<stripe_shared::CheckoutBilliePaymentMethodOptions>,
    boleto: Option<stripe_shared::CheckoutBoletoPaymentMethodOptions>,
    card: Option<stripe_shared::CheckoutCardPaymentMethodOptions>,
    cashapp: Option<stripe_shared::CheckoutCashappPaymentMethodOptions>,
    customer_balance: Option<stripe_shared::CheckoutCustomerBalancePaymentMethodOptions>,
    eps: Option<stripe_shared::CheckoutEpsPaymentMethodOptions>,
    fpx: Option<stripe_shared::CheckoutFpxPaymentMethodOptions>,
    giropay: Option<stripe_shared::CheckoutGiropayPaymentMethodOptions>,
    grabpay: Option<stripe_shared::CheckoutGrabPayPaymentMethodOptions>,
    ideal: Option<stripe_shared::CheckoutIdealPaymentMethodOptions>,
    kakao_pay: Option<stripe_shared::CheckoutKakaoPayPaymentMethodOptions>,
    klarna: Option<stripe_shared::CheckoutKlarnaPaymentMethodOptions>,
    konbini: Option<stripe_shared::CheckoutKonbiniPaymentMethodOptions>,
    kr_card: Option<stripe_shared::CheckoutKrCardPaymentMethodOptions>,
    link: Option<stripe_shared::CheckoutLinkPaymentMethodOptions>,
    mobilepay: Option<stripe_shared::CheckoutMobilepayPaymentMethodOptions>,
    multibanco: Option<stripe_shared::CheckoutMultibancoPaymentMethodOptions>,
    naver_pay: Option<stripe_shared::CheckoutNaverPayPaymentMethodOptions>,
    oxxo: Option<stripe_shared::CheckoutOxxoPaymentMethodOptions>,
    p24: Option<stripe_shared::CheckoutP24PaymentMethodOptions>,
    payco: Option<stripe_shared::CheckoutPaycoPaymentMethodOptions>,
    paynow: Option<stripe_shared::CheckoutPaynowPaymentMethodOptions>,
    paypal: Option<stripe_shared::CheckoutPaypalPaymentMethodOptions>,
    payto: Option<stripe_shared::CheckoutPaytoPaymentMethodOptions>,
    pix: Option<stripe_shared::CheckoutPixPaymentMethodOptions>,
    revolut_pay: Option<stripe_shared::CheckoutRevolutPayPaymentMethodOptions>,
    samsung_pay: Option<stripe_shared::CheckoutSamsungPayPaymentMethodOptions>,
    satispay: Option<stripe_shared::CheckoutSatispayPaymentMethodOptions>,
    scalapay: Option<stripe_shared::CheckoutScalapayPaymentMethodOptions>,
    sepa_debit: Option<stripe_shared::CheckoutSepaDebitPaymentMethodOptions>,
    sofort: Option<stripe_shared::CheckoutSofortPaymentMethodOptions>,
    sunbit: Option<stripe_shared::CheckoutSunbitPaymentMethodOptions>,
    swish: Option<stripe_shared::CheckoutSwishPaymentMethodOptions>,
    twint: Option<stripe_shared::CheckoutTwintPaymentMethodOptions>,
    upi: Option<stripe_shared::CheckoutUpiPaymentMethodOptions>,
    us_bank_account: Option<stripe_shared::CheckoutUsBankAccountPaymentMethodOptions>,
    wechat_pay: Option<stripe_shared::CheckoutWechatPayPaymentMethodOptions>,
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

    impl Deserialize for CheckoutSessionPaymentMethodOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CheckoutSessionPaymentMethodOptions>,
        builder: CheckoutSessionPaymentMethodOptionsBuilder,
    }

    impl Visitor for Place<CheckoutSessionPaymentMethodOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CheckoutSessionPaymentMethodOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CheckoutSessionPaymentMethodOptionsBuilder {
        type Out = CheckoutSessionPaymentMethodOptions;
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
                "boleto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.boleto),
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "cashapp" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp),
                "customer_balance" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.customer_balance)
                }
                "eps" => stripe_types::miniserde_helpers::begin_nullable(&mut self.eps),
                "fpx" => stripe_types::miniserde_helpers::begin_nullable(&mut self.fpx),
                "giropay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.giropay),
                "grabpay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.grabpay),
                "ideal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.ideal),
                "kakao_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.kakao_pay),
                "klarna" => stripe_types::miniserde_helpers::begin_nullable(&mut self.klarna),
                "konbini" => stripe_types::miniserde_helpers::begin_nullable(&mut self.konbini),
                "kr_card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.kr_card),
                "link" => stripe_types::miniserde_helpers::begin_nullable(&mut self.link),
                "mobilepay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.mobilepay),
                "multibanco" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.multibanco)
                }
                "naver_pay" => stripe_types::miniserde_helpers::begin_nullable(&mut self.naver_pay),
                "oxxo" => stripe_types::miniserde_helpers::begin_nullable(&mut self.oxxo),
                "p24" => stripe_types::miniserde_helpers::begin_nullable(&mut self.p24),
                "payco" => stripe_types::miniserde_helpers::begin_nullable(&mut self.payco),
                "paynow" => stripe_types::miniserde_helpers::begin_nullable(&mut self.paynow),
                "paypal" => stripe_types::miniserde_helpers::begin_nullable(&mut self.paypal),
                "payto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.payto),
                "pix" => stripe_types::miniserde_helpers::begin_nullable(&mut self.pix),
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
                boleto: None,
                card: None,
                cashapp: None,
                customer_balance: None,
                eps: None,
                fpx: None,
                giropay: None,
                grabpay: None,
                ideal: None,
                kakao_pay: None,
                klarna: None,
                konbini: None,
                kr_card: None,
                link: None,
                mobilepay: None,
                multibanco: None,
                naver_pay: None,
                oxxo: None,
                p24: None,
                payco: None,
                paynow: None,
                paypal: None,
                payto: None,
                pix: None,
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
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let Self {
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
                boleto,
                card,
                cashapp,
                customer_balance,
                eps,
                fpx,
                giropay,
                grabpay,
                ideal,
                kakao_pay,
                klarna,
                konbini,
                kr_card,
                link,
                mobilepay,
                multibanco,
                naver_pay,
                oxxo,
                p24,
                payco,
                paynow,
                paypal,
                payto,
                pix,
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
            } = core::mem::replace(self, Self::deser_default());
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
                boleto,
                card,
                cashapp,
                customer_balance,
                eps,
                fpx,
                giropay,
                grabpay,
                ideal,
                kakao_pay,
                klarna,
                konbini,
                kr_card,
                link,
                mobilepay,
                multibanco,
                naver_pay,
                oxxo,
                p24,
                payco,
                paynow,
                paypal,
                payto,
                pix,
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

    impl ObjectDeser for CheckoutSessionPaymentMethodOptions {
        type Builder = CheckoutSessionPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutSessionPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
