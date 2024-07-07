#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetails {
    pub ach_credit_transfer: Option<stripe_shared::PaymentMethodDetailsAchCreditTransfer>,
    pub ach_debit: Option<stripe_shared::PaymentMethodDetailsAchDebit>,
    pub acss_debit: Option<stripe_shared::PaymentMethodDetailsAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodDetailsAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodDetailsAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipayDetails>,
    pub amazon_pay: Option<stripe_shared::PaymentMethodDetailsAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::PaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodDetailsBancontact>,
    pub blik: Option<stripe_shared::PaymentMethodDetailsBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodDetailsBoleto>,
    pub card: Option<stripe_shared::PaymentMethodDetailsCard>,
    pub card_present: Option<stripe_shared::PaymentMethodDetailsCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodDetailsCashapp>,
    pub customer_balance: Option<stripe_shared::PaymentMethodDetailsCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentMethodDetailsEps>,
    pub fpx: Option<stripe_shared::PaymentMethodDetailsFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodDetailsGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodDetailsGrabpay>,
    pub ideal: Option<stripe_shared::PaymentMethodDetailsIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodDetailsInteracPresent>,
    pub klarna: Option<stripe_shared::PaymentMethodDetailsKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodDetailsKonbini>,
    pub link: Option<stripe_shared::PaymentMethodDetailsLink>,
    pub mobilepay: Option<stripe_shared::PaymentMethodDetailsMobilepay>,
    pub multibanco: Option<stripe_shared::PaymentMethodDetailsMultibanco>,
    pub oxxo: Option<stripe_shared::PaymentMethodDetailsOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodDetailsP24>,
    pub paynow: Option<stripe_shared::PaymentMethodDetailsPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodDetailsPaypal>,
    pub pix: Option<stripe_shared::PaymentMethodDetailsPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodDetailsPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodDetailsRevolutPay>,
    pub sepa_credit_transfer: Option<stripe_shared::PaymentMethodDetailsSepaCreditTransfer>,
    pub sepa_debit: Option<stripe_shared::PaymentMethodDetailsSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodDetailsSofort>,
    pub stripe_account: Option<stripe_shared::PaymentMethodDetailsStripeAccount>,
    pub swish: Option<stripe_shared::PaymentMethodDetailsSwish>,
    /// The type of transaction-specific details of the payment method used in the payment, one of `ach_credit_transfer`, `ach_debit`, `acss_debit`, `alipay`, `au_becs_debit`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `klarna`, `multibanco`, `p24`, `sepa_debit`, `sofort`, `stripe_account`, or `wechat`.
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains information specific to the payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub us_bank_account: Option<stripe_shared::PaymentMethodDetailsUsBankAccount>,
    pub wechat: Option<stripe_shared::PaymentMethodDetailsWechat>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodDetailsWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodDetailsZip>,
}
#[doc(hidden)]
pub struct PaymentMethodDetailsBuilder {
    ach_credit_transfer: Option<Option<stripe_shared::PaymentMethodDetailsAchCreditTransfer>>,
    ach_debit: Option<Option<stripe_shared::PaymentMethodDetailsAchDebit>>,
    acss_debit: Option<Option<stripe_shared::PaymentMethodDetailsAcssDebit>>,
    affirm: Option<Option<stripe_shared::PaymentMethodDetailsAffirm>>,
    afterpay_clearpay: Option<Option<stripe_shared::PaymentMethodDetailsAfterpayClearpay>>,
    alipay: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipayDetails>>,
    amazon_pay: Option<Option<stripe_shared::PaymentMethodDetailsAmazonPay>>,
    au_becs_debit: Option<Option<stripe_shared::PaymentMethodDetailsAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::PaymentMethodDetailsBacsDebit>>,
    bancontact: Option<Option<stripe_shared::PaymentMethodDetailsBancontact>>,
    blik: Option<Option<stripe_shared::PaymentMethodDetailsBlik>>,
    boleto: Option<Option<stripe_shared::PaymentMethodDetailsBoleto>>,
    card: Option<Option<stripe_shared::PaymentMethodDetailsCard>>,
    card_present: Option<Option<stripe_shared::PaymentMethodDetailsCardPresent>>,
    cashapp: Option<Option<stripe_shared::PaymentMethodDetailsCashapp>>,
    customer_balance: Option<Option<stripe_shared::PaymentMethodDetailsCustomerBalance>>,
    eps: Option<Option<stripe_shared::PaymentMethodDetailsEps>>,
    fpx: Option<Option<stripe_shared::PaymentMethodDetailsFpx>>,
    giropay: Option<Option<stripe_shared::PaymentMethodDetailsGiropay>>,
    grabpay: Option<Option<stripe_shared::PaymentMethodDetailsGrabpay>>,
    ideal: Option<Option<stripe_shared::PaymentMethodDetailsIdeal>>,
    interac_present: Option<Option<stripe_shared::PaymentMethodDetailsInteracPresent>>,
    klarna: Option<Option<stripe_shared::PaymentMethodDetailsKlarna>>,
    konbini: Option<Option<stripe_shared::PaymentMethodDetailsKonbini>>,
    link: Option<Option<stripe_shared::PaymentMethodDetailsLink>>,
    mobilepay: Option<Option<stripe_shared::PaymentMethodDetailsMobilepay>>,
    multibanco: Option<Option<stripe_shared::PaymentMethodDetailsMultibanco>>,
    oxxo: Option<Option<stripe_shared::PaymentMethodDetailsOxxo>>,
    p24: Option<Option<stripe_shared::PaymentMethodDetailsP24>>,
    paynow: Option<Option<stripe_shared::PaymentMethodDetailsPaynow>>,
    paypal: Option<Option<stripe_shared::PaymentMethodDetailsPaypal>>,
    pix: Option<Option<stripe_shared::PaymentMethodDetailsPix>>,
    promptpay: Option<Option<stripe_shared::PaymentMethodDetailsPromptpay>>,
    revolut_pay: Option<Option<stripe_shared::PaymentMethodDetailsRevolutPay>>,
    sepa_credit_transfer: Option<Option<stripe_shared::PaymentMethodDetailsSepaCreditTransfer>>,
    sepa_debit: Option<Option<stripe_shared::PaymentMethodDetailsSepaDebit>>,
    sofort: Option<Option<stripe_shared::PaymentMethodDetailsSofort>>,
    stripe_account: Option<Option<stripe_shared::PaymentMethodDetailsStripeAccount>>,
    swish: Option<Option<stripe_shared::PaymentMethodDetailsSwish>>,
    type_: Option<String>,
    us_bank_account: Option<Option<stripe_shared::PaymentMethodDetailsUsBankAccount>>,
    wechat: Option<Option<stripe_shared::PaymentMethodDetailsWechat>>,
    wechat_pay: Option<Option<stripe_shared::PaymentMethodDetailsWechatPay>>,
    zip: Option<Option<stripe_shared::PaymentMethodDetailsZip>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodDetails>,
        builder: PaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<PaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodDetailsBuilder {
        type Out = PaymentMethodDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ach_credit_transfer" => Deserialize::begin(&mut self.ach_credit_transfer),
                "ach_debit" => Deserialize::begin(&mut self.ach_debit),
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
                "multibanco" => Deserialize::begin(&mut self.multibanco),
                "oxxo" => Deserialize::begin(&mut self.oxxo),
                "p24" => Deserialize::begin(&mut self.p24),
                "paynow" => Deserialize::begin(&mut self.paynow),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "pix" => Deserialize::begin(&mut self.pix),
                "promptpay" => Deserialize::begin(&mut self.promptpay),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "sepa_credit_transfer" => Deserialize::begin(&mut self.sepa_credit_transfer),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
                "stripe_account" => Deserialize::begin(&mut self.stripe_account),
                "swish" => Deserialize::begin(&mut self.swish),
                "type" => Deserialize::begin(&mut self.type_),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),
                "wechat" => Deserialize::begin(&mut self.wechat),
                "wechat_pay" => Deserialize::begin(&mut self.wechat_pay),
                "zip" => Deserialize::begin(&mut self.zip),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                ach_credit_transfer: Deserialize::default(),
                ach_debit: Deserialize::default(),
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
                multibanco: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                promptpay: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                sepa_credit_transfer: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                stripe_account: Deserialize::default(),
                swish: Deserialize::default(),
                type_: Deserialize::default(),
                us_bank_account: Deserialize::default(),
                wechat: Deserialize::default(),
                wechat_pay: Deserialize::default(),
                zip: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                ach_credit_transfer: self.ach_credit_transfer.take()?,
                ach_debit: self.ach_debit.take()?,
                acss_debit: self.acss_debit.take()?,
                affirm: self.affirm?,
                afterpay_clearpay: self.afterpay_clearpay.take()?,
                alipay: self.alipay.take()?,
                amazon_pay: self.amazon_pay?,
                au_becs_debit: self.au_becs_debit.take()?,
                bacs_debit: self.bacs_debit.take()?,
                bancontact: self.bancontact.take()?,
                blik: self.blik?,
                boleto: self.boleto.take()?,
                card: self.card.take()?,
                card_present: self.card_present.take()?,
                cashapp: self.cashapp.take()?,
                customer_balance: self.customer_balance?,
                eps: self.eps.take()?,
                fpx: self.fpx.take()?,
                giropay: self.giropay.take()?,
                grabpay: self.grabpay.take()?,
                ideal: self.ideal.take()?,
                interac_present: self.interac_present.take()?,
                klarna: self.klarna.take()?,
                konbini: self.konbini?,
                link: self.link.take()?,
                mobilepay: self.mobilepay.take()?,
                multibanco: self.multibanco.take()?,
                oxxo: self.oxxo.take()?,
                p24: self.p24.take()?,
                paynow: self.paynow.take()?,
                paypal: self.paypal.take()?,
                pix: self.pix.take()?,
                promptpay: self.promptpay.take()?,
                revolut_pay: self.revolut_pay?,
                sepa_credit_transfer: self.sepa_credit_transfer.take()?,
                sepa_debit: self.sepa_debit.take()?,
                sofort: self.sofort.take()?,
                stripe_account: self.stripe_account?,
                swish: self.swish.take()?,
                type_: self.type_.take()?,
                us_bank_account: self.us_bank_account.take()?,
                wechat: self.wechat?,
                wechat_pay: self.wechat_pay.take()?,
                zip: self.zip?,
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

    impl ObjectDeser for PaymentMethodDetails {
        type Builder = PaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for PaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ach_credit_transfer" => {
                        b.ach_credit_transfer = Some(FromValueOpt::from_value(v)?)
                    }
                    "ach_debit" => b.ach_debit = Some(FromValueOpt::from_value(v)?),
                    "acss_debit" => b.acss_debit = Some(FromValueOpt::from_value(v)?),
                    "affirm" => b.affirm = Some(FromValueOpt::from_value(v)?),
                    "afterpay_clearpay" => b.afterpay_clearpay = Some(FromValueOpt::from_value(v)?),
                    "alipay" => b.alipay = Some(FromValueOpt::from_value(v)?),
                    "amazon_pay" => b.amazon_pay = Some(FromValueOpt::from_value(v)?),
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
                    "mobilepay" => b.mobilepay = Some(FromValueOpt::from_value(v)?),
                    "multibanco" => b.multibanco = Some(FromValueOpt::from_value(v)?),
                    "oxxo" => b.oxxo = Some(FromValueOpt::from_value(v)?),
                    "p24" => b.p24 = Some(FromValueOpt::from_value(v)?),
                    "paynow" => b.paynow = Some(FromValueOpt::from_value(v)?),
                    "paypal" => b.paypal = Some(FromValueOpt::from_value(v)?),
                    "pix" => b.pix = Some(FromValueOpt::from_value(v)?),
                    "promptpay" => b.promptpay = Some(FromValueOpt::from_value(v)?),
                    "revolut_pay" => b.revolut_pay = Some(FromValueOpt::from_value(v)?),
                    "sepa_credit_transfer" => {
                        b.sepa_credit_transfer = Some(FromValueOpt::from_value(v)?)
                    }
                    "sepa_debit" => b.sepa_debit = Some(FromValueOpt::from_value(v)?),
                    "sofort" => b.sofort = Some(FromValueOpt::from_value(v)?),
                    "stripe_account" => b.stripe_account = Some(FromValueOpt::from_value(v)?),
                    "swish" => b.swish = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "us_bank_account" => b.us_bank_account = Some(FromValueOpt::from_value(v)?),
                    "wechat" => b.wechat = Some(FromValueOpt::from_value(v)?),
                    "wechat_pay" => b.wechat_pay = Some(FromValueOpt::from_value(v)?),
                    "zip" => b.zip = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
