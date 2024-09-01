#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodOptionsAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodOptionsAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentMethodOptionsAlipay>,
    pub amazon_pay: Option<stripe_shared::PaymentMethodOptionsAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentMethodOptionsBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodOptionsBancontact>,
    pub blik: Option<stripe_shared::PaymentIntentPaymentMethodOptionsBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodOptionsBoleto>,
    pub card: Option<stripe_shared::PaymentIntentPaymentMethodOptionsCard>,
    pub card_present: Option<stripe_shared::PaymentMethodOptionsCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodOptionsCashapp>,
    pub customer_balance: Option<stripe_shared::PaymentMethodOptionsCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentIntentPaymentMethodOptionsEps>,
    pub fpx: Option<stripe_shared::PaymentMethodOptionsFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodOptionsGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodOptionsGrabpay>,
    pub ideal: Option<stripe_shared::PaymentMethodOptionsIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodOptionsInteracPresent>,
    pub klarna: Option<stripe_shared::PaymentMethodOptionsKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodOptionsKonbini>,
    pub link: Option<stripe_shared::PaymentIntentPaymentMethodOptionsLink>,
    pub mobilepay: Option<stripe_shared::PaymentIntentPaymentMethodOptionsMobilepay>,
    pub oxxo: Option<stripe_shared::PaymentMethodOptionsOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodOptionsP24>,
    pub paynow: Option<stripe_shared::PaymentMethodOptionsPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodOptionsPaypal>,
    pub pix: Option<stripe_shared::PaymentMethodOptionsPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodOptionsPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodOptionsRevolutPay>,
    pub sepa_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodOptionsSofort>,
    pub swish: Option<stripe_shared::PaymentIntentPaymentMethodOptionsSwish>,
    pub us_bank_account: Option<stripe_shared::PaymentIntentPaymentMethodOptionsUsBankAccount>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodOptionsWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodOptionsZip>,
}
#[doc(hidden)]
pub struct PaymentIntentPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsAcssDebit>>,
    affirm: Option<Option<stripe_shared::PaymentMethodOptionsAffirm>>,
    afterpay_clearpay: Option<Option<stripe_shared::PaymentMethodOptionsAfterpayClearpay>>,
    alipay: Option<Option<stripe_shared::PaymentMethodOptionsAlipay>>,
    amazon_pay: Option<Option<stripe_shared::PaymentMethodOptionsAmazonPay>>,
    au_becs_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::PaymentMethodOptionsBacsDebit>>,
    bancontact: Option<Option<stripe_shared::PaymentMethodOptionsBancontact>>,
    blik: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsBlik>>,
    boleto: Option<Option<stripe_shared::PaymentMethodOptionsBoleto>>,
    card: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsCard>>,
    card_present: Option<Option<stripe_shared::PaymentMethodOptionsCardPresent>>,
    cashapp: Option<Option<stripe_shared::PaymentMethodOptionsCashapp>>,
    customer_balance: Option<Option<stripe_shared::PaymentMethodOptionsCustomerBalance>>,
    eps: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsEps>>,
    fpx: Option<Option<stripe_shared::PaymentMethodOptionsFpx>>,
    giropay: Option<Option<stripe_shared::PaymentMethodOptionsGiropay>>,
    grabpay: Option<Option<stripe_shared::PaymentMethodOptionsGrabpay>>,
    ideal: Option<Option<stripe_shared::PaymentMethodOptionsIdeal>>,
    interac_present: Option<Option<stripe_shared::PaymentMethodOptionsInteracPresent>>,
    klarna: Option<Option<stripe_shared::PaymentMethodOptionsKlarna>>,
    konbini: Option<Option<stripe_shared::PaymentMethodOptionsKonbini>>,
    link: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsLink>>,
    mobilepay: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsMobilepay>>,
    oxxo: Option<Option<stripe_shared::PaymentMethodOptionsOxxo>>,
    p24: Option<Option<stripe_shared::PaymentMethodOptionsP24>>,
    paynow: Option<Option<stripe_shared::PaymentMethodOptionsPaynow>>,
    paypal: Option<Option<stripe_shared::PaymentMethodOptionsPaypal>>,
    pix: Option<Option<stripe_shared::PaymentMethodOptionsPix>>,
    promptpay: Option<Option<stripe_shared::PaymentMethodOptionsPromptpay>>,
    revolut_pay: Option<Option<stripe_shared::PaymentMethodOptionsRevolutPay>>,
    sepa_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>>,
    sofort: Option<Option<stripe_shared::PaymentMethodOptionsSofort>>,
    swish: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsSwish>>,
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
    use miniserde::{make_place, Deserialize, Result};
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
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.bancontact),
                "blik" => Deserialize::begin(&mut self.blik),
                "boleto" => Deserialize::begin(&mut self.boleto),
                "card" => Deserialize::begin(&mut self.card),
                "card_present" => Deserialize::begin(&mut self.card_present),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "customer_balance" => Deserialize::begin(&mut self.customer_balance),
                "eps" => Deserialize::begin(&mut self.eps),
                "fpx" => Deserialize::begin(&mut self.fpx),
                "giropay" => Deserialize::begin(&mut self.giropay),
                "grabpay" => Deserialize::begin(&mut self.grabpay),
                "ideal" => Deserialize::begin(&mut self.ideal),
                "interac_present" => Deserialize::begin(&mut self.interac_present),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "konbini" => Deserialize::begin(&mut self.konbini),
                "link" => Deserialize::begin(&mut self.link),
                "mobilepay" => Deserialize::begin(&mut self.mobilepay),
                "oxxo" => Deserialize::begin(&mut self.oxxo),
                "p24" => Deserialize::begin(&mut self.p24),
                "paynow" => Deserialize::begin(&mut self.paynow),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "pix" => Deserialize::begin(&mut self.pix),
                "promptpay" => Deserialize::begin(&mut self.promptpay),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
                "swish" => Deserialize::begin(&mut self.swish),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),
                "wechat_pay" => Deserialize::begin(&mut self.wechat_pay),
                "zip" => Deserialize::begin(&mut self.zip),

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
                blik: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                cashapp: Deserialize::default(),
                customer_balance: Deserialize::default(),
                eps: Deserialize::default(),
                fpx: Deserialize::default(),
                giropay: Deserialize::default(),
                grabpay: Deserialize::default(),
                ideal: Deserialize::default(),
                interac_present: Deserialize::default(),
                klarna: Deserialize::default(),
                konbini: Deserialize::default(),
                link: Deserialize::default(),
                mobilepay: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                promptpay: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                swish: Deserialize::default(),
                us_bank_account: Deserialize::default(),
                wechat_pay: Deserialize::default(),
                zip: Deserialize::default(),
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
                Some(blik),
                Some(boleto),
                Some(card),
                Some(card_present),
                Some(cashapp),
                Some(customer_balance),
                Some(eps),
                Some(fpx),
                Some(giropay),
                Some(grabpay),
                Some(ideal),
                Some(interac_present),
                Some(klarna),
                Some(konbini),
                Some(link),
                Some(mobilepay),
                Some(oxxo),
                Some(p24),
                Some(paynow),
                Some(paypal),
                Some(pix),
                Some(promptpay),
                Some(revolut_pay),
                Some(sepa_debit),
                Some(sofort),
                Some(swish),
                Some(us_bank_account),
                Some(wechat_pay),
                Some(zip),
            ) = (
                self.acss_debit.take(),
                self.affirm.take(),
                self.afterpay_clearpay.take(),
                self.alipay,
                self.amazon_pay,
                self.au_becs_debit,
                self.bacs_debit,
                self.bancontact,
                self.blik,
                self.boleto,
                self.card.take(),
                self.card_present,
                self.cashapp,
                self.customer_balance.take(),
                self.eps,
                self.fpx,
                self.giropay,
                self.grabpay,
                self.ideal,
                self.interac_present,
                self.klarna.take(),
                self.konbini.take(),
                self.link.take(),
                self.mobilepay,
                self.oxxo,
                self.p24,
                self.paynow,
                self.paypal.take(),
                self.pix,
                self.promptpay,
                self.revolut_pay,
                self.sepa_debit,
                self.sofort,
                self.swish.take(),
                self.us_bank_account.take(),
                self.wechat_pay.take(),
                self.zip,
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
                blik,
                boleto,
                card,
                card_present,
                cashapp,
                customer_balance,
                eps,
                fpx,
                giropay,
                grabpay,
                ideal,
                interac_present,
                klarna,
                konbini,
                link,
                mobilepay,
                oxxo,
                p24,
                paynow,
                paypal,
                pix,
                promptpay,
                revolut_pay,
                sepa_debit,
                sofort,
                swish,
                us_bank_account,
                wechat_pay,
                zip,
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
                    "amazon_pay" => b.amazon_pay = FromValueOpt::from_value(v),
                    "au_becs_debit" => b.au_becs_debit = FromValueOpt::from_value(v),
                    "bacs_debit" => b.bacs_debit = FromValueOpt::from_value(v),
                    "bancontact" => b.bancontact = FromValueOpt::from_value(v),
                    "blik" => b.blik = FromValueOpt::from_value(v),
                    "boleto" => b.boleto = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "card_present" => b.card_present = FromValueOpt::from_value(v),
                    "cashapp" => b.cashapp = FromValueOpt::from_value(v),
                    "customer_balance" => b.customer_balance = FromValueOpt::from_value(v),
                    "eps" => b.eps = FromValueOpt::from_value(v),
                    "fpx" => b.fpx = FromValueOpt::from_value(v),
                    "giropay" => b.giropay = FromValueOpt::from_value(v),
                    "grabpay" => b.grabpay = FromValueOpt::from_value(v),
                    "ideal" => b.ideal = FromValueOpt::from_value(v),
                    "interac_present" => b.interac_present = FromValueOpt::from_value(v),
                    "klarna" => b.klarna = FromValueOpt::from_value(v),
                    "konbini" => b.konbini = FromValueOpt::from_value(v),
                    "link" => b.link = FromValueOpt::from_value(v),
                    "mobilepay" => b.mobilepay = FromValueOpt::from_value(v),
                    "oxxo" => b.oxxo = FromValueOpt::from_value(v),
                    "p24" => b.p24 = FromValueOpt::from_value(v),
                    "paynow" => b.paynow = FromValueOpt::from_value(v),
                    "paypal" => b.paypal = FromValueOpt::from_value(v),
                    "pix" => b.pix = FromValueOpt::from_value(v),
                    "promptpay" => b.promptpay = FromValueOpt::from_value(v),
                    "revolut_pay" => b.revolut_pay = FromValueOpt::from_value(v),
                    "sepa_debit" => b.sepa_debit = FromValueOpt::from_value(v),
                    "sofort" => b.sofort = FromValueOpt::from_value(v),
                    "swish" => b.swish = FromValueOpt::from_value(v),
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
