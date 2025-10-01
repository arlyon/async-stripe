/// Details of the PaymentMethod collected by Payment Element
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ConfirmationTokensResourcePaymentMethodPreview {
    pub acss_debit: Option<stripe_shared::PaymentMethodAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipay>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to “unspecified”.
    pub allow_redisplay: Option<ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay>,
    pub alma: Option<stripe_shared::PaymentMethodAlma>,
    pub amazon_pay: Option<stripe_shared::PaymentMethodAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::PaymentMethodAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentMethodBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodBancontact>,
    pub billie: Option<stripe_shared::PaymentMethodBillie>,
    pub billing_details: stripe_shared::BillingDetails,
    pub blik: Option<stripe_shared::PaymentMethodBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodBoleto>,
    pub card: Option<stripe_shared::PaymentMethodCard>,
    pub card_present: Option<stripe_shared::PaymentMethodCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodCashapp>,
    pub crypto: Option<stripe_shared::PaymentMethodCrypto>,
    /// The ID of the Customer to which this PaymentMethod is saved.
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    pub customer_balance: Option<stripe_shared::PaymentMethodCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentMethodEps>,
    pub fpx: Option<stripe_shared::PaymentMethodFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodGrabpay>,
    pub ideal: Option<stripe_shared::PaymentMethodIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodInteracPresent>,
    pub kakao_pay: Option<stripe_shared::PaymentMethodKakaoPay>,
    pub klarna: Option<stripe_shared::PaymentMethodKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodKonbini>,
    pub kr_card: Option<stripe_shared::PaymentMethodKrCard>,
    pub link: Option<stripe_shared::PaymentMethodLink>,
    pub mb_way: Option<stripe_shared::PaymentMethodMbWay>,
    pub mobilepay: Option<stripe_shared::PaymentMethodMobilepay>,
    pub multibanco: Option<stripe_shared::PaymentMethodMultibanco>,
    pub naver_pay: Option<stripe_shared::PaymentMethodNaverPay>,
    pub nz_bank_account: Option<stripe_shared::PaymentMethodNzBankAccount>,
    pub oxxo: Option<stripe_shared::PaymentMethodOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodP24>,
    pub pay_by_bank: Option<stripe_shared::PaymentMethodPayByBank>,
    pub payco: Option<stripe_shared::PaymentMethodPayco>,
    pub paynow: Option<stripe_shared::PaymentMethodPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodPaypal>,
    pub pix: Option<stripe_shared::PaymentMethodPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodRevolutPay>,
    pub samsung_pay: Option<stripe_shared::PaymentMethodSamsungPay>,
    pub satispay: Option<stripe_shared::PaymentMethodSatispay>,
    pub sepa_debit: Option<stripe_shared::PaymentMethodSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodSofort>,
    pub swish: Option<stripe_shared::PaymentMethodSwish>,
    pub twint: Option<stripe_shared::PaymentMethodTwint>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: ConfirmationTokensResourcePaymentMethodPreviewType,
    pub us_bank_account: Option<stripe_shared::PaymentMethodUsBankAccount>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodZip>,
}
#[doc(hidden)]
pub struct ConfirmationTokensResourcePaymentMethodPreviewBuilder {
    acss_debit: Option<Option<stripe_shared::PaymentMethodAcssDebit>>,
    affirm: Option<Option<stripe_shared::PaymentMethodAffirm>>,
    afterpay_clearpay: Option<Option<stripe_shared::PaymentMethodAfterpayClearpay>>,
    alipay: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipay>>,
    allow_redisplay: Option<Option<ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay>>,
    alma: Option<Option<stripe_shared::PaymentMethodAlma>>,
    amazon_pay: Option<Option<stripe_shared::PaymentMethodAmazonPay>>,
    au_becs_debit: Option<Option<stripe_shared::PaymentMethodAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::PaymentMethodBacsDebit>>,
    bancontact: Option<Option<stripe_shared::PaymentMethodBancontact>>,
    billie: Option<Option<stripe_shared::PaymentMethodBillie>>,
    billing_details: Option<stripe_shared::BillingDetails>,
    blik: Option<Option<stripe_shared::PaymentMethodBlik>>,
    boleto: Option<Option<stripe_shared::PaymentMethodBoleto>>,
    card: Option<Option<stripe_shared::PaymentMethodCard>>,
    card_present: Option<Option<stripe_shared::PaymentMethodCardPresent>>,
    cashapp: Option<Option<stripe_shared::PaymentMethodCashapp>>,
    crypto: Option<Option<stripe_shared::PaymentMethodCrypto>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_balance: Option<Option<stripe_shared::PaymentMethodCustomerBalance>>,
    eps: Option<Option<stripe_shared::PaymentMethodEps>>,
    fpx: Option<Option<stripe_shared::PaymentMethodFpx>>,
    giropay: Option<Option<stripe_shared::PaymentMethodGiropay>>,
    grabpay: Option<Option<stripe_shared::PaymentMethodGrabpay>>,
    ideal: Option<Option<stripe_shared::PaymentMethodIdeal>>,
    interac_present: Option<Option<stripe_shared::PaymentMethodInteracPresent>>,
    kakao_pay: Option<Option<stripe_shared::PaymentMethodKakaoPay>>,
    klarna: Option<Option<stripe_shared::PaymentMethodKlarna>>,
    konbini: Option<Option<stripe_shared::PaymentMethodKonbini>>,
    kr_card: Option<Option<stripe_shared::PaymentMethodKrCard>>,
    link: Option<Option<stripe_shared::PaymentMethodLink>>,
    mb_way: Option<Option<stripe_shared::PaymentMethodMbWay>>,
    mobilepay: Option<Option<stripe_shared::PaymentMethodMobilepay>>,
    multibanco: Option<Option<stripe_shared::PaymentMethodMultibanco>>,
    naver_pay: Option<Option<stripe_shared::PaymentMethodNaverPay>>,
    nz_bank_account: Option<Option<stripe_shared::PaymentMethodNzBankAccount>>,
    oxxo: Option<Option<stripe_shared::PaymentMethodOxxo>>,
    p24: Option<Option<stripe_shared::PaymentMethodP24>>,
    pay_by_bank: Option<Option<stripe_shared::PaymentMethodPayByBank>>,
    payco: Option<Option<stripe_shared::PaymentMethodPayco>>,
    paynow: Option<Option<stripe_shared::PaymentMethodPaynow>>,
    paypal: Option<Option<stripe_shared::PaymentMethodPaypal>>,
    pix: Option<Option<stripe_shared::PaymentMethodPix>>,
    promptpay: Option<Option<stripe_shared::PaymentMethodPromptpay>>,
    revolut_pay: Option<Option<stripe_shared::PaymentMethodRevolutPay>>,
    samsung_pay: Option<Option<stripe_shared::PaymentMethodSamsungPay>>,
    satispay: Option<Option<stripe_shared::PaymentMethodSatispay>>,
    sepa_debit: Option<Option<stripe_shared::PaymentMethodSepaDebit>>,
    sofort: Option<Option<stripe_shared::PaymentMethodSofort>>,
    swish: Option<Option<stripe_shared::PaymentMethodSwish>>,
    twint: Option<Option<stripe_shared::PaymentMethodTwint>>,
    type_: Option<ConfirmationTokensResourcePaymentMethodPreviewType>,
    us_bank_account: Option<Option<stripe_shared::PaymentMethodUsBankAccount>>,
    wechat_pay: Option<Option<stripe_shared::PaymentMethodWechatPay>>,
    zip: Option<Option<stripe_shared::PaymentMethodZip>>,
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

    impl Deserialize for ConfirmationTokensResourcePaymentMethodPreview {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ConfirmationTokensResourcePaymentMethodPreview>,
        builder: ConfirmationTokensResourcePaymentMethodPreviewBuilder,
    }

    impl Visitor for Place<ConfirmationTokensResourcePaymentMethodPreview> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ConfirmationTokensResourcePaymentMethodPreviewBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ConfirmationTokensResourcePaymentMethodPreviewBuilder {
        type Out = ConfirmationTokensResourcePaymentMethodPreview;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "affirm" => Deserialize::begin(&mut self.affirm),
                "afterpay_clearpay" => Deserialize::begin(&mut self.afterpay_clearpay),
                "alipay" => Deserialize::begin(&mut self.alipay),
                "allow_redisplay" => Deserialize::begin(&mut self.allow_redisplay),
                "alma" => Deserialize::begin(&mut self.alma),
                "amazon_pay" => Deserialize::begin(&mut self.amazon_pay),
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.bancontact),
                "billie" => Deserialize::begin(&mut self.billie),
                "billing_details" => Deserialize::begin(&mut self.billing_details),
                "blik" => Deserialize::begin(&mut self.blik),
                "boleto" => Deserialize::begin(&mut self.boleto),
                "card" => Deserialize::begin(&mut self.card),
                "card_present" => Deserialize::begin(&mut self.card_present),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "crypto" => Deserialize::begin(&mut self.crypto),
                "customer" => Deserialize::begin(&mut self.customer),
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
                "pix" => Deserialize::begin(&mut self.pix),
                "promptpay" => Deserialize::begin(&mut self.promptpay),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "samsung_pay" => Deserialize::begin(&mut self.samsung_pay),
                "satispay" => Deserialize::begin(&mut self.satispay),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
                "swish" => Deserialize::begin(&mut self.swish),
                "twint" => Deserialize::begin(&mut self.twint),
                "type" => Deserialize::begin(&mut self.type_),
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
                allow_redisplay: Deserialize::default(),
                alma: Deserialize::default(),
                amazon_pay: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                billie: Deserialize::default(),
                billing_details: Deserialize::default(),
                blik: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                cashapp: Deserialize::default(),
                crypto: Deserialize::default(),
                customer: Deserialize::default(),
                customer_balance: Deserialize::default(),
                eps: Deserialize::default(),
                fpx: Deserialize::default(),
                giropay: Deserialize::default(),
                grabpay: Deserialize::default(),
                ideal: Deserialize::default(),
                interac_present: Deserialize::default(),
                kakao_pay: Deserialize::default(),
                klarna: Deserialize::default(),
                konbini: Deserialize::default(),
                kr_card: Deserialize::default(),
                link: Deserialize::default(),
                mb_way: Deserialize::default(),
                mobilepay: Deserialize::default(),
                multibanco: Deserialize::default(),
                naver_pay: Deserialize::default(),
                nz_bank_account: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                pay_by_bank: Deserialize::default(),
                payco: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                promptpay: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                samsung_pay: Deserialize::default(),
                satispay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                swish: Deserialize::default(),
                twint: Deserialize::default(),
                type_: Deserialize::default(),
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
                Some(allow_redisplay),
                Some(alma),
                Some(amazon_pay),
                Some(au_becs_debit),
                Some(bacs_debit),
                Some(bancontact),
                Some(billie),
                Some(billing_details),
                Some(blik),
                Some(boleto),
                Some(card),
                Some(card_present),
                Some(cashapp),
                Some(crypto),
                Some(customer),
                Some(customer_balance),
                Some(eps),
                Some(fpx),
                Some(giropay),
                Some(grabpay),
                Some(ideal),
                Some(interac_present),
                Some(kakao_pay),
                Some(klarna),
                Some(konbini),
                Some(kr_card),
                Some(link),
                Some(mb_way),
                Some(mobilepay),
                Some(multibanco),
                Some(naver_pay),
                Some(nz_bank_account),
                Some(oxxo),
                Some(p24),
                Some(pay_by_bank),
                Some(payco),
                Some(paynow),
                Some(paypal),
                Some(pix),
                Some(promptpay),
                Some(revolut_pay),
                Some(samsung_pay),
                Some(satispay),
                Some(sepa_debit),
                Some(sofort),
                Some(swish),
                Some(twint),
                Some(type_),
                Some(us_bank_account),
                Some(wechat_pay),
                Some(zip),
            ) = (
                self.acss_debit.take(),
                self.affirm,
                self.afterpay_clearpay,
                self.alipay,
                self.allow_redisplay,
                self.alma,
                self.amazon_pay,
                self.au_becs_debit.take(),
                self.bacs_debit.take(),
                self.bancontact,
                self.billie,
                self.billing_details.take(),
                self.blik,
                self.boleto.take(),
                self.card.take(),
                self.card_present.take(),
                self.cashapp.take(),
                self.crypto,
                self.customer.take(),
                self.customer_balance,
                self.eps.take(),
                self.fpx.take(),
                self.giropay,
                self.grabpay,
                self.ideal.take(),
                self.interac_present.take(),
                self.kakao_pay,
                self.klarna,
                self.konbini,
                self.kr_card.take(),
                self.link.take(),
                self.mb_way,
                self.mobilepay,
                self.multibanco,
                self.naver_pay.take(),
                self.nz_bank_account.take(),
                self.oxxo,
                self.p24.take(),
                self.pay_by_bank,
                self.payco,
                self.paynow,
                self.paypal.take(),
                self.pix,
                self.promptpay,
                self.revolut_pay,
                self.samsung_pay,
                self.satispay,
                self.sepa_debit.take(),
                self.sofort.take(),
                self.swish,
                self.twint,
                self.type_.take(),
                self.us_bank_account.take(),
                self.wechat_pay,
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
                allow_redisplay,
                alma,
                amazon_pay,
                au_becs_debit,
                bacs_debit,
                bancontact,
                billie,
                billing_details,
                blik,
                boleto,
                card,
                card_present,
                cashapp,
                crypto,
                customer,
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
                pix,
                promptpay,
                revolut_pay,
                samsung_pay,
                satispay,
                sepa_debit,
                sofort,
                swish,
                twint,
                type_,
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
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for ConfirmationTokensResourcePaymentMethodPreview {
        type Builder = ConfirmationTokensResourcePaymentMethodPreviewBuilder;
    }

    impl FromValueOpt for ConfirmationTokensResourcePaymentMethodPreview {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ConfirmationTokensResourcePaymentMethodPreviewBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit" => b.acss_debit = FromValueOpt::from_value(v),
                    "affirm" => b.affirm = FromValueOpt::from_value(v),
                    "afterpay_clearpay" => b.afterpay_clearpay = FromValueOpt::from_value(v),
                    "alipay" => b.alipay = FromValueOpt::from_value(v),
                    "allow_redisplay" => b.allow_redisplay = FromValueOpt::from_value(v),
                    "alma" => b.alma = FromValueOpt::from_value(v),
                    "amazon_pay" => b.amazon_pay = FromValueOpt::from_value(v),
                    "au_becs_debit" => b.au_becs_debit = FromValueOpt::from_value(v),
                    "bacs_debit" => b.bacs_debit = FromValueOpt::from_value(v),
                    "bancontact" => b.bancontact = FromValueOpt::from_value(v),
                    "billie" => b.billie = FromValueOpt::from_value(v),
                    "billing_details" => b.billing_details = FromValueOpt::from_value(v),
                    "blik" => b.blik = FromValueOpt::from_value(v),
                    "boleto" => b.boleto = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "card_present" => b.card_present = FromValueOpt::from_value(v),
                    "cashapp" => b.cashapp = FromValueOpt::from_value(v),
                    "crypto" => b.crypto = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
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
                    "pix" => b.pix = FromValueOpt::from_value(v),
                    "promptpay" => b.promptpay = FromValueOpt::from_value(v),
                    "revolut_pay" => b.revolut_pay = FromValueOpt::from_value(v),
                    "samsung_pay" => b.samsung_pay = FromValueOpt::from_value(v),
                    "satispay" => b.satispay = FromValueOpt::from_value(v),
                    "sepa_debit" => b.sepa_debit = FromValueOpt::from_value(v),
                    "sofort" => b.sofort = FromValueOpt::from_value(v),
                    "swish" => b.swish = FromValueOpt::from_value(v),
                    "twint" => b.twint = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
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
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to “unspecified”.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for ConfirmationTokensResourcePaymentMethodPreviewAllowRedisplay",
            )
        })
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ConfirmationTokensResourcePaymentMethodPreviewType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AmazonPay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
    Blik,
    Boleto,
    Card,
    CardPresent,
    Cashapp,
    Crypto,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    KakaoPay,
    Klarna,
    Konbini,
    KrCard,
    Link,
    MbWay,
    Mobilepay,
    Multibanco,
    NaverPay,
    NzBankAccount,
    Oxxo,
    P24,
    PayByBank,
    Payco,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    SepaDebit,
    Sofort,
    Swish,
    Twint,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ConfirmationTokensResourcePaymentMethodPreviewType {
    pub fn as_str(&self) -> &str {
        use ConfirmationTokensResourcePaymentMethodPreviewType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            Alma => "alma",
            AmazonPay => "amazon_pay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Billie => "billie",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            CardPresent => "card_present",
            Cashapp => "cashapp",
            Crypto => "crypto",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            InteracPresent => "interac_present",
            KakaoPay => "kakao_pay",
            Klarna => "klarna",
            Konbini => "konbini",
            KrCard => "kr_card",
            Link => "link",
            MbWay => "mb_way",
            Mobilepay => "mobilepay",
            Multibanco => "multibanco",
            NaverPay => "naver_pay",
            NzBankAccount => "nz_bank_account",
            Oxxo => "oxxo",
            P24 => "p24",
            PayByBank => "pay_by_bank",
            Payco => "payco",
            Paynow => "paynow",
            Paypal => "paypal",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SamsungPay => "samsung_pay",
            Satispay => "satispay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            Twint => "twint",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ConfirmationTokensResourcePaymentMethodPreviewType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ConfirmationTokensResourcePaymentMethodPreviewType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "alma" => Ok(Alma),
            "amazon_pay" => Ok(AmazonPay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "billie" => Ok(Billie),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "card_present" => Ok(CardPresent),
            "cashapp" => Ok(Cashapp),
            "crypto" => Ok(Crypto),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "interac_present" => Ok(InteracPresent),
            "kakao_pay" => Ok(KakaoPay),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "kr_card" => Ok(KrCard),
            "link" => Ok(Link),
            "mb_way" => Ok(MbWay),
            "mobilepay" => Ok(Mobilepay),
            "multibanco" => Ok(Multibanco),
            "naver_pay" => Ok(NaverPay),
            "nz_bank_account" => Ok(NzBankAccount),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "pay_by_bank" => Ok(PayByBank),
            "payco" => Ok(Payco),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "samsung_pay" => Ok(SamsungPay),
            "satispay" => Ok(Satispay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "twint" => Ok(Twint),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ConfirmationTokensResourcePaymentMethodPreviewType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ConfirmationTokensResourcePaymentMethodPreviewType::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ConfirmationTokensResourcePaymentMethodPreviewType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ConfirmationTokensResourcePaymentMethodPreviewType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
