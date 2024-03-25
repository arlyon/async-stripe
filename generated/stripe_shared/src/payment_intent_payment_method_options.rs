#[derive(Clone, Debug, Default)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentIntentPaymentMethodOptions {
    pub acss_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodOptionsAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodOptionsAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentMethodOptionsAlipay>,
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
    pub oxxo: Option<stripe_shared::PaymentMethodOptionsOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodOptionsP24>,
    pub paynow: Option<stripe_shared::PaymentMethodOptionsPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodOptionsPaypal>,
    pub pix: Option<stripe_shared::PaymentMethodOptionsPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodOptionsPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodOptionsRevolutPay>,
    pub sepa_debit: Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodOptionsSofort>,
    pub us_bank_account: Option<stripe_shared::PaymentIntentPaymentMethodOptionsUsBankAccount>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodOptionsWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodOptionsZip>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentIntentPaymentMethodOptionsBuilder {
    acss_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsAcssDebit>>,
    affirm: Option<Option<stripe_shared::PaymentMethodOptionsAffirm>>,
    afterpay_clearpay: Option<Option<stripe_shared::PaymentMethodOptionsAfterpayClearpay>>,
    alipay: Option<Option<stripe_shared::PaymentMethodOptionsAlipay>>,
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
    oxxo: Option<Option<stripe_shared::PaymentMethodOptionsOxxo>>,
    p24: Option<Option<stripe_shared::PaymentMethodOptionsP24>>,
    paynow: Option<Option<stripe_shared::PaymentMethodOptionsPaynow>>,
    paypal: Option<Option<stripe_shared::PaymentMethodOptionsPaypal>>,
    pix: Option<Option<stripe_shared::PaymentMethodOptionsPix>>,
    promptpay: Option<Option<stripe_shared::PaymentMethodOptionsPromptpay>>,
    revolut_pay: Option<Option<stripe_shared::PaymentMethodOptionsRevolutPay>>,
    sepa_debit: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsSepaDebit>>,
    sofort: Option<Option<stripe_shared::PaymentMethodOptionsSofort>>,
    us_bank_account: Option<Option<stripe_shared::PaymentIntentPaymentMethodOptionsUsBankAccount>>,
    wechat_pay: Option<Option<stripe_shared::PaymentMethodOptionsWechatPay>>,
    zip: Option<Option<stripe_shared::PaymentMethodOptionsZip>>,
}

#[cfg(feature = "min-ser")]
#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
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
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentIntentPaymentMethodOptionsBuilder::deser_default() }))
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
                "oxxo" => Deserialize::begin(&mut self.oxxo),
                "p24" => Deserialize::begin(&mut self.p24),
                "paynow" => Deserialize::begin(&mut self.paynow),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "pix" => Deserialize::begin(&mut self.pix),
                "promptpay" => Deserialize::begin(&mut self.promptpay),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
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
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                promptpay: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                us_bank_account: Deserialize::default(),
                wechat_pay: Deserialize::default(),
                zip: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let acss_debit = self.acss_debit.take()?;
            let affirm = self.affirm.take()?;
            let afterpay_clearpay = self.afterpay_clearpay.take()?;
            let alipay = self.alipay.take()?;
            let au_becs_debit = self.au_becs_debit.take()?;
            let bacs_debit = self.bacs_debit.take()?;
            let bancontact = self.bancontact.take()?;
            let blik = self.blik.take()?;
            let boleto = self.boleto.take()?;
            let card = self.card.take()?;
            let card_present = self.card_present.take()?;
            let cashapp = self.cashapp.take()?;
            let customer_balance = self.customer_balance.take()?;
            let eps = self.eps.take()?;
            let fpx = self.fpx.take()?;
            let giropay = self.giropay.take()?;
            let grabpay = self.grabpay.take()?;
            let ideal = self.ideal.take()?;
            let interac_present = self.interac_present.take()?;
            let klarna = self.klarna.take()?;
            let konbini = self.konbini.take()?;
            let link = self.link.take()?;
            let oxxo = self.oxxo.take()?;
            let p24 = self.p24.take()?;
            let paynow = self.paynow.take()?;
            let paypal = self.paypal.take()?;
            let pix = self.pix.take()?;
            let promptpay = self.promptpay.take()?;
            let revolut_pay = self.revolut_pay.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let sofort = self.sofort.take()?;
            let us_bank_account = self.us_bank_account.take()?;
            let wechat_pay = self.wechat_pay.take()?;
            let zip = self.zip.take()?;

            Some(Self::Out {
                acss_debit,
                affirm,
                afterpay_clearpay,
                alipay,
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
                oxxo,
                p24,
                paynow,
                paypal,
                pix,
                promptpay,
                revolut_pay,
                sepa_debit,
                sofort,
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
                    "acss_debit" => b.acss_debit = Some(FromValueOpt::from_value(v)?),
                    "affirm" => b.affirm = Some(FromValueOpt::from_value(v)?),
                    "afterpay_clearpay" => b.afterpay_clearpay = Some(FromValueOpt::from_value(v)?),
                    "alipay" => b.alipay = Some(FromValueOpt::from_value(v)?),
                    "au_becs_debit" => b.au_becs_debit = Some(FromValueOpt::from_value(v)?),
                    "bacs_debit" => b.bacs_debit = Some(FromValueOpt::from_value(v)?),
                    "bancontact" => b.bancontact = Some(FromValueOpt::from_value(v)?),
                    "blik" => b.blik = Some(FromValueOpt::from_value(v)?),
                    "boleto" => b.boleto = Some(FromValueOpt::from_value(v)?),
                    "card" => b.card = Some(FromValueOpt::from_value(v)?),
                    "card_present" => b.card_present = Some(FromValueOpt::from_value(v)?),
                    "cashapp" => b.cashapp = Some(FromValueOpt::from_value(v)?),
                    "customer_balance" => b.customer_balance = Some(FromValueOpt::from_value(v)?),
                    "eps" => b.eps = Some(FromValueOpt::from_value(v)?),
                    "fpx" => b.fpx = Some(FromValueOpt::from_value(v)?),
                    "giropay" => b.giropay = Some(FromValueOpt::from_value(v)?),
                    "grabpay" => b.grabpay = Some(FromValueOpt::from_value(v)?),
                    "ideal" => b.ideal = Some(FromValueOpt::from_value(v)?),
                    "interac_present" => b.interac_present = Some(FromValueOpt::from_value(v)?),
                    "klarna" => b.klarna = Some(FromValueOpt::from_value(v)?),
                    "konbini" => b.konbini = Some(FromValueOpt::from_value(v)?),
                    "link" => b.link = Some(FromValueOpt::from_value(v)?),
                    "oxxo" => b.oxxo = Some(FromValueOpt::from_value(v)?),
                    "p24" => b.p24 = Some(FromValueOpt::from_value(v)?),
                    "paynow" => b.paynow = Some(FromValueOpt::from_value(v)?),
                    "paypal" => b.paypal = Some(FromValueOpt::from_value(v)?),
                    "pix" => b.pix = Some(FromValueOpt::from_value(v)?),
                    "promptpay" => b.promptpay = Some(FromValueOpt::from_value(v)?),
                    "revolut_pay" => b.revolut_pay = Some(FromValueOpt::from_value(v)?),
                    "sepa_debit" => b.sepa_debit = Some(FromValueOpt::from_value(v)?),
                    "sofort" => b.sofort = Some(FromValueOpt::from_value(v)?),
                    "us_bank_account" => b.us_bank_account = Some(FromValueOpt::from_value(v)?),
                    "wechat_pay" => b.wechat_pay = Some(FromValueOpt::from_value(v)?),
                    "zip" => b.zip = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
