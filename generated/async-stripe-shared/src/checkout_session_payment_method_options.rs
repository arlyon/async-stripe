#[derive(Clone, Debug)]
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
    pub pix: Option<stripe_shared::CheckoutPixPaymentMethodOptions>,
    pub revolut_pay: Option<stripe_shared::CheckoutRevolutPayPaymentMethodOptions>,
    pub samsung_pay: Option<stripe_shared::CheckoutSamsungPayPaymentMethodOptions>,
    pub satispay: Option<stripe_shared::CheckoutSatispayPaymentMethodOptions>,
    pub sepa_debit: Option<stripe_shared::CheckoutSepaDebitPaymentMethodOptions>,
    pub sofort: Option<stripe_shared::CheckoutSofortPaymentMethodOptions>,
    pub swish: Option<stripe_shared::CheckoutSwishPaymentMethodOptions>,
    pub us_bank_account: Option<stripe_shared::CheckoutUsBankAccountPaymentMethodOptions>,
}
#[doc(hidden)]
pub struct CheckoutSessionPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::CheckoutAcssDebitPaymentMethodOptions>>,
    affirm: Option<Option<stripe_shared::CheckoutAffirmPaymentMethodOptions>>,
    afterpay_clearpay: Option<Option<stripe_shared::CheckoutAfterpayClearpayPaymentMethodOptions>>,
    alipay: Option<Option<stripe_shared::CheckoutAlipayPaymentMethodOptions>>,
    alma: Option<Option<stripe_shared::CheckoutAlmaPaymentMethodOptions>>,
    amazon_pay: Option<Option<stripe_shared::CheckoutAmazonPayPaymentMethodOptions>>,
    au_becs_debit: Option<Option<stripe_shared::CheckoutAuBecsDebitPaymentMethodOptions>>,
    bacs_debit: Option<Option<stripe_shared::CheckoutBacsDebitPaymentMethodOptions>>,
    bancontact: Option<Option<stripe_shared::CheckoutBancontactPaymentMethodOptions>>,
    billie: Option<Option<stripe_shared::CheckoutBilliePaymentMethodOptions>>,
    boleto: Option<Option<stripe_shared::CheckoutBoletoPaymentMethodOptions>>,
    card: Option<Option<stripe_shared::CheckoutCardPaymentMethodOptions>>,
    cashapp: Option<Option<stripe_shared::CheckoutCashappPaymentMethodOptions>>,
    customer_balance: Option<Option<stripe_shared::CheckoutCustomerBalancePaymentMethodOptions>>,
    eps: Option<Option<stripe_shared::CheckoutEpsPaymentMethodOptions>>,
    fpx: Option<Option<stripe_shared::CheckoutFpxPaymentMethodOptions>>,
    giropay: Option<Option<stripe_shared::CheckoutGiropayPaymentMethodOptions>>,
    grabpay: Option<Option<stripe_shared::CheckoutGrabPayPaymentMethodOptions>>,
    ideal: Option<Option<stripe_shared::CheckoutIdealPaymentMethodOptions>>,
    kakao_pay: Option<Option<stripe_shared::CheckoutKakaoPayPaymentMethodOptions>>,
    klarna: Option<Option<stripe_shared::CheckoutKlarnaPaymentMethodOptions>>,
    konbini: Option<Option<stripe_shared::CheckoutKonbiniPaymentMethodOptions>>,
    kr_card: Option<Option<stripe_shared::CheckoutKrCardPaymentMethodOptions>>,
    link: Option<Option<stripe_shared::CheckoutLinkPaymentMethodOptions>>,
    mobilepay: Option<Option<stripe_shared::CheckoutMobilepayPaymentMethodOptions>>,
    multibanco: Option<Option<stripe_shared::CheckoutMultibancoPaymentMethodOptions>>,
    naver_pay: Option<Option<stripe_shared::CheckoutNaverPayPaymentMethodOptions>>,
    oxxo: Option<Option<stripe_shared::CheckoutOxxoPaymentMethodOptions>>,
    p24: Option<Option<stripe_shared::CheckoutP24PaymentMethodOptions>>,
    payco: Option<Option<stripe_shared::CheckoutPaycoPaymentMethodOptions>>,
    paynow: Option<Option<stripe_shared::CheckoutPaynowPaymentMethodOptions>>,
    paypal: Option<Option<stripe_shared::CheckoutPaypalPaymentMethodOptions>>,
    pix: Option<Option<stripe_shared::CheckoutPixPaymentMethodOptions>>,
    revolut_pay: Option<Option<stripe_shared::CheckoutRevolutPayPaymentMethodOptions>>,
    samsung_pay: Option<Option<stripe_shared::CheckoutSamsungPayPaymentMethodOptions>>,
    satispay: Option<Option<stripe_shared::CheckoutSatispayPaymentMethodOptions>>,
    sepa_debit: Option<Option<stripe_shared::CheckoutSepaDebitPaymentMethodOptions>>,
    sofort: Option<Option<stripe_shared::CheckoutSofortPaymentMethodOptions>>,
    swish: Option<Option<stripe_shared::CheckoutSwishPaymentMethodOptions>>,
    us_bank_account: Option<Option<stripe_shared::CheckoutUsBankAccountPaymentMethodOptions>>,
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
                "boleto" => Deserialize::begin(&mut self.boleto),
                "card" => Deserialize::begin(&mut self.card),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "customer_balance" => Deserialize::begin(&mut self.customer_balance),
                "eps" => Deserialize::begin(&mut self.eps),
                "fpx" => Deserialize::begin(&mut self.fpx),
                "giropay" => Deserialize::begin(&mut self.giropay),
                "grabpay" => Deserialize::begin(&mut self.grabpay),
                "ideal" => Deserialize::begin(&mut self.ideal),
                "kakao_pay" => Deserialize::begin(&mut self.kakao_pay),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "konbini" => Deserialize::begin(&mut self.konbini),
                "kr_card" => Deserialize::begin(&mut self.kr_card),
                "link" => Deserialize::begin(&mut self.link),
                "mobilepay" => Deserialize::begin(&mut self.mobilepay),
                "multibanco" => Deserialize::begin(&mut self.multibanco),
                "naver_pay" => Deserialize::begin(&mut self.naver_pay),
                "oxxo" => Deserialize::begin(&mut self.oxxo),
                "p24" => Deserialize::begin(&mut self.p24),
                "payco" => Deserialize::begin(&mut self.payco),
                "paynow" => Deserialize::begin(&mut self.paynow),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "pix" => Deserialize::begin(&mut self.pix),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "samsung_pay" => Deserialize::begin(&mut self.samsung_pay),
                "satispay" => Deserialize::begin(&mut self.satispay),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
                "swish" => Deserialize::begin(&mut self.swish),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                affirm: Deserialize::default(),
                afterpay_clearpay: Deserialize::default(),
                alipay: Deserialize::default(),
                alma: Deserialize::default(),
                amazon_pay: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                billie: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                cashapp: Deserialize::default(),
                customer_balance: Deserialize::default(),
                eps: Deserialize::default(),
                fpx: Deserialize::default(),
                giropay: Deserialize::default(),
                grabpay: Deserialize::default(),
                ideal: Deserialize::default(),
                kakao_pay: Deserialize::default(),
                klarna: Deserialize::default(),
                konbini: Deserialize::default(),
                kr_card: Deserialize::default(),
                link: Deserialize::default(),
                mobilepay: Deserialize::default(),
                multibanco: Deserialize::default(),
                naver_pay: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                payco: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                samsung_pay: Deserialize::default(),
                satispay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                swish: Deserialize::default(),
                us_bank_account: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(acss_debit),
                Some(affirm),
                Some(afterpay_clearpay),
                Some(alipay),
                Some(alma),
                Some(amazon_pay),
                Some(au_becs_debit),
                Some(bacs_debit),
                Some(bancontact),
                Some(billie),
                Some(boleto),
                Some(card),
                Some(cashapp),
                Some(customer_balance),
                Some(eps),
                Some(fpx),
                Some(giropay),
                Some(grabpay),
                Some(ideal),
                Some(kakao_pay),
                Some(klarna),
                Some(konbini),
                Some(kr_card),
                Some(link),
                Some(mobilepay),
                Some(multibanco),
                Some(naver_pay),
                Some(oxxo),
                Some(p24),
                Some(payco),
                Some(paynow),
                Some(paypal),
                Some(pix),
                Some(revolut_pay),
                Some(samsung_pay),
                Some(satispay),
                Some(sepa_debit),
                Some(sofort),
                Some(swish),
                Some(us_bank_account),
            ) = (
                self.acss_debit.take(),
                self.affirm,
                self.afterpay_clearpay,
                self.alipay,
                self.alma,
                self.amazon_pay,
                self.au_becs_debit.take(),
                self.bacs_debit.take(),
                self.bancontact,
                self.billie,
                self.boleto,
                self.card.take(),
                self.cashapp,
                self.customer_balance.take(),
                self.eps,
                self.fpx,
                self.giropay,
                self.grabpay,
                self.ideal,
                self.kakao_pay,
                self.klarna,
                self.konbini,
                self.kr_card,
                self.link,
                self.mobilepay,
                self.multibanco,
                self.naver_pay,
                self.oxxo,
                self.p24,
                self.payco,
                self.paynow,
                self.paypal.take(),
                self.pix,
                self.revolut_pay,
                self.samsung_pay,
                self.satispay,
                self.sepa_debit.take(),
                self.sofort,
                self.swish.take(),
                self.us_bank_account.take(),
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
                pix,
                revolut_pay,
                samsung_pay,
                satispay,
                sepa_debit,
                sofort,
                swish,
                us_bank_account,
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

    impl ObjectDeser for CheckoutSessionPaymentMethodOptions {
        type Builder = CheckoutSessionPaymentMethodOptionsBuilder;
    }

    impl FromValueOpt for CheckoutSessionPaymentMethodOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CheckoutSessionPaymentMethodOptionsBuilder::deser_default();
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
                    "boleto" => b.boleto = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "cashapp" => b.cashapp = FromValueOpt::from_value(v),
                    "customer_balance" => b.customer_balance = FromValueOpt::from_value(v),
                    "eps" => b.eps = FromValueOpt::from_value(v),
                    "fpx" => b.fpx = FromValueOpt::from_value(v),
                    "giropay" => b.giropay = FromValueOpt::from_value(v),
                    "grabpay" => b.grabpay = FromValueOpt::from_value(v),
                    "ideal" => b.ideal = FromValueOpt::from_value(v),
                    "kakao_pay" => b.kakao_pay = FromValueOpt::from_value(v),
                    "klarna" => b.klarna = FromValueOpt::from_value(v),
                    "konbini" => b.konbini = FromValueOpt::from_value(v),
                    "kr_card" => b.kr_card = FromValueOpt::from_value(v),
                    "link" => b.link = FromValueOpt::from_value(v),
                    "mobilepay" => b.mobilepay = FromValueOpt::from_value(v),
                    "multibanco" => b.multibanco = FromValueOpt::from_value(v),
                    "naver_pay" => b.naver_pay = FromValueOpt::from_value(v),
                    "oxxo" => b.oxxo = FromValueOpt::from_value(v),
                    "p24" => b.p24 = FromValueOpt::from_value(v),
                    "payco" => b.payco = FromValueOpt::from_value(v),
                    "paynow" => b.paynow = FromValueOpt::from_value(v),
                    "paypal" => b.paypal = FromValueOpt::from_value(v),
                    "pix" => b.pix = FromValueOpt::from_value(v),
                    "revolut_pay" => b.revolut_pay = FromValueOpt::from_value(v),
                    "samsung_pay" => b.samsung_pay = FromValueOpt::from_value(v),
                    "satispay" => b.satispay = FromValueOpt::from_value(v),
                    "sepa_debit" => b.sepa_debit = FromValueOpt::from_value(v),
                    "sofort" => b.sofort = FromValueOpt::from_value(v),
                    "swish" => b.swish = FromValueOpt::from_value(v),
                    "us_bank_account" => b.us_bank_account = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
