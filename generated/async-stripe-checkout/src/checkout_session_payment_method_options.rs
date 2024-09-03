#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CheckoutSessionPaymentMethodOptions {
    pub acss_debit: Option<stripe_checkout::CheckoutAcssDebitPaymentMethodOptions>,
    pub affirm: Option<stripe_checkout::CheckoutAffirmPaymentMethodOptions>,
    pub afterpay_clearpay: Option<stripe_checkout::CheckoutAfterpayClearpayPaymentMethodOptions>,
    pub alipay: Option<stripe_checkout::CheckoutAlipayPaymentMethodOptions>,
    pub amazon_pay: Option<stripe_checkout::CheckoutAmazonPayPaymentMethodOptions>,
    pub au_becs_debit: Option<stripe_checkout::CheckoutAuBecsDebitPaymentMethodOptions>,
    pub bacs_debit: Option<stripe_checkout::CheckoutBacsDebitPaymentMethodOptions>,
    pub bancontact: Option<stripe_checkout::CheckoutBancontactPaymentMethodOptions>,
    pub boleto: Option<stripe_checkout::CheckoutBoletoPaymentMethodOptions>,
    pub card: Option<stripe_checkout::CheckoutCardPaymentMethodOptions>,
    pub cashapp: Option<stripe_checkout::CheckoutCashappPaymentMethodOptions>,
    pub customer_balance: Option<stripe_checkout::CheckoutCustomerBalancePaymentMethodOptions>,
    pub eps: Option<stripe_checkout::CheckoutEpsPaymentMethodOptions>,
    pub fpx: Option<stripe_checkout::CheckoutFpxPaymentMethodOptions>,
    pub giropay: Option<stripe_checkout::CheckoutGiropayPaymentMethodOptions>,
    pub grabpay: Option<stripe_checkout::CheckoutGrabPayPaymentMethodOptions>,
    pub ideal: Option<stripe_checkout::CheckoutIdealPaymentMethodOptions>,
    pub klarna: Option<stripe_checkout::CheckoutKlarnaPaymentMethodOptions>,
    pub konbini: Option<stripe_checkout::CheckoutKonbiniPaymentMethodOptions>,
    pub link: Option<stripe_checkout::CheckoutLinkPaymentMethodOptions>,
    pub mobilepay: Option<stripe_checkout::CheckoutMobilepayPaymentMethodOptions>,
    pub oxxo: Option<stripe_checkout::CheckoutOxxoPaymentMethodOptions>,
    pub p24: Option<stripe_checkout::CheckoutP24PaymentMethodOptions>,
    pub paynow: Option<stripe_checkout::CheckoutPaynowPaymentMethodOptions>,
    pub paypal: Option<stripe_checkout::CheckoutPaypalPaymentMethodOptions>,
    pub pix: Option<stripe_checkout::CheckoutPixPaymentMethodOptions>,
    pub revolut_pay: Option<stripe_checkout::CheckoutRevolutPayPaymentMethodOptions>,
    pub sepa_debit: Option<stripe_checkout::CheckoutSepaDebitPaymentMethodOptions>,
    pub sofort: Option<stripe_checkout::CheckoutSofortPaymentMethodOptions>,
    pub swish: Option<stripe_checkout::CheckoutSwishPaymentMethodOptions>,
    pub us_bank_account: Option<stripe_checkout::CheckoutUsBankAccountPaymentMethodOptions>,
}
#[doc(hidden)]
pub struct CheckoutSessionPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_checkout::CheckoutAcssDebitPaymentMethodOptions>>,
    affirm: Option<Option<stripe_checkout::CheckoutAffirmPaymentMethodOptions>>,
    afterpay_clearpay:
        Option<Option<stripe_checkout::CheckoutAfterpayClearpayPaymentMethodOptions>>,
    alipay: Option<Option<stripe_checkout::CheckoutAlipayPaymentMethodOptions>>,
    amazon_pay: Option<Option<stripe_checkout::CheckoutAmazonPayPaymentMethodOptions>>,
    au_becs_debit: Option<Option<stripe_checkout::CheckoutAuBecsDebitPaymentMethodOptions>>,
    bacs_debit: Option<Option<stripe_checkout::CheckoutBacsDebitPaymentMethodOptions>>,
    bancontact: Option<Option<stripe_checkout::CheckoutBancontactPaymentMethodOptions>>,
    boleto: Option<Option<stripe_checkout::CheckoutBoletoPaymentMethodOptions>>,
    card: Option<Option<stripe_checkout::CheckoutCardPaymentMethodOptions>>,
    cashapp: Option<Option<stripe_checkout::CheckoutCashappPaymentMethodOptions>>,
    customer_balance: Option<Option<stripe_checkout::CheckoutCustomerBalancePaymentMethodOptions>>,
    eps: Option<Option<stripe_checkout::CheckoutEpsPaymentMethodOptions>>,
    fpx: Option<Option<stripe_checkout::CheckoutFpxPaymentMethodOptions>>,
    giropay: Option<Option<stripe_checkout::CheckoutGiropayPaymentMethodOptions>>,
    grabpay: Option<Option<stripe_checkout::CheckoutGrabPayPaymentMethodOptions>>,
    ideal: Option<Option<stripe_checkout::CheckoutIdealPaymentMethodOptions>>,
    klarna: Option<Option<stripe_checkout::CheckoutKlarnaPaymentMethodOptions>>,
    konbini: Option<Option<stripe_checkout::CheckoutKonbiniPaymentMethodOptions>>,
    link: Option<Option<stripe_checkout::CheckoutLinkPaymentMethodOptions>>,
    mobilepay: Option<Option<stripe_checkout::CheckoutMobilepayPaymentMethodOptions>>,
    oxxo: Option<Option<stripe_checkout::CheckoutOxxoPaymentMethodOptions>>,
    p24: Option<Option<stripe_checkout::CheckoutP24PaymentMethodOptions>>,
    paynow: Option<Option<stripe_checkout::CheckoutPaynowPaymentMethodOptions>>,
    paypal: Option<Option<stripe_checkout::CheckoutPaypalPaymentMethodOptions>>,
    pix: Option<Option<stripe_checkout::CheckoutPixPaymentMethodOptions>>,
    revolut_pay: Option<Option<stripe_checkout::CheckoutRevolutPayPaymentMethodOptions>>,
    sepa_debit: Option<Option<stripe_checkout::CheckoutSepaDebitPaymentMethodOptions>>,
    sofort: Option<Option<stripe_checkout::CheckoutSofortPaymentMethodOptions>>,
    swish: Option<Option<stripe_checkout::CheckoutSwishPaymentMethodOptions>>,
    us_bank_account: Option<Option<stripe_checkout::CheckoutUsBankAccountPaymentMethodOptions>>,
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
    use miniserde::{make_place, Deserialize, Result};
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
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.bancontact),
                "boleto" => Deserialize::begin(&mut self.boleto),
                "card" => Deserialize::begin(&mut self.card),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "customer_balance" => Deserialize::begin(&mut self.customer_balance),
                "eps" => Deserialize::begin(&mut self.eps),
                "fpx" => Deserialize::begin(&mut self.fpx),
                "giropay" => Deserialize::begin(&mut self.giropay),
                "grabpay" => Deserialize::begin(&mut self.grabpay),
                "ideal" => Deserialize::begin(&mut self.ideal),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "konbini" => Deserialize::begin(&mut self.konbini),
                "link" => Deserialize::begin(&mut self.link),
                "mobilepay" => Deserialize::begin(&mut self.mobilepay),
                "oxxo" => Deserialize::begin(&mut self.oxxo),
                "p24" => Deserialize::begin(&mut self.p24),
                "paynow" => Deserialize::begin(&mut self.paynow),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "pix" => Deserialize::begin(&mut self.pix),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
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
                amazon_pay: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                cashapp: Deserialize::default(),
                customer_balance: Deserialize::default(),
                eps: Deserialize::default(),
                fpx: Deserialize::default(),
                giropay: Deserialize::default(),
                grabpay: Deserialize::default(),
                ideal: Deserialize::default(),
                klarna: Deserialize::default(),
                konbini: Deserialize::default(),
                link: Deserialize::default(),
                mobilepay: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                revolut_pay: Deserialize::default(),
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
                Some(amazon_pay),
                Some(au_becs_debit),
                Some(bacs_debit),
                Some(bancontact),
                Some(boleto),
                Some(card),
                Some(cashapp),
                Some(customer_balance),
                Some(eps),
                Some(fpx),
                Some(giropay),
                Some(grabpay),
                Some(ideal),
                Some(klarna),
                Some(konbini),
                Some(link),
                Some(mobilepay),
                Some(oxxo),
                Some(p24),
                Some(paynow),
                Some(paypal),
                Some(pix),
                Some(revolut_pay),
                Some(sepa_debit),
                Some(sofort),
                Some(swish),
                Some(us_bank_account),
            ) = (
                self.acss_debit.take(),
                self.affirm,
                self.afterpay_clearpay,
                self.alipay,
                self.amazon_pay,
                self.au_becs_debit,
                self.bacs_debit,
                self.bancontact,
                self.boleto,
                self.card.take(),
                self.cashapp,
                self.customer_balance.take(),
                self.eps,
                self.fpx,
                self.giropay,
                self.grabpay,
                self.ideal,
                self.klarna,
                self.konbini,
                self.link,
                self.mobilepay,
                self.oxxo,
                self.p24,
                self.paynow,
                self.paypal.take(),
                self.pix,
                self.revolut_pay,
                self.sepa_debit,
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
                amazon_pay,
                au_becs_debit,
                bacs_debit,
                bancontact,
                boleto,
                card,
                cashapp,
                customer_balance,
                eps,
                fpx,
                giropay,
                grabpay,
                ideal,
                klarna,
                konbini,
                link,
                mobilepay,
                oxxo,
                p24,
                paynow,
                paypal,
                pix,
                revolut_pay,
                sepa_debit,
                sofort,
                swish,
                us_bank_account,
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
                    "amazon_pay" => b.amazon_pay = FromValueOpt::from_value(v),
                    "au_becs_debit" => b.au_becs_debit = FromValueOpt::from_value(v),
                    "bacs_debit" => b.bacs_debit = FromValueOpt::from_value(v),
                    "bancontact" => b.bancontact = FromValueOpt::from_value(v),
                    "boleto" => b.boleto = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "cashapp" => b.cashapp = FromValueOpt::from_value(v),
                    "customer_balance" => b.customer_balance = FromValueOpt::from_value(v),
                    "eps" => b.eps = FromValueOpt::from_value(v),
                    "fpx" => b.fpx = FromValueOpt::from_value(v),
                    "giropay" => b.giropay = FromValueOpt::from_value(v),
                    "grabpay" => b.grabpay = FromValueOpt::from_value(v),
                    "ideal" => b.ideal = FromValueOpt::from_value(v),
                    "klarna" => b.klarna = FromValueOpt::from_value(v),
                    "konbini" => b.konbini = FromValueOpt::from_value(v),
                    "link" => b.link = FromValueOpt::from_value(v),
                    "mobilepay" => b.mobilepay = FromValueOpt::from_value(v),
                    "oxxo" => b.oxxo = FromValueOpt::from_value(v),
                    "p24" => b.p24 = FromValueOpt::from_value(v),
                    "paynow" => b.paynow = FromValueOpt::from_value(v),
                    "paypal" => b.paypal = FromValueOpt::from_value(v),
                    "pix" => b.pix = FromValueOpt::from_value(v),
                    "revolut_pay" => b.revolut_pay = FromValueOpt::from_value(v),
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
