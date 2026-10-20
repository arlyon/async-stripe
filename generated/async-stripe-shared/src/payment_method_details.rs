#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodDetails {
    pub ach_credit_transfer: Option<stripe_shared::PaymentMethodDetailsAchCreditTransfer>,
    pub ach_debit: Option<stripe_shared::PaymentMethodDetailsAchDebit>,
    pub acss_debit: Option<stripe_shared::PaymentMethodDetailsAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodDetailsAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodDetailsAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentMethodDetailsAlipay>,
    pub alma: Option<stripe_shared::PaymentMethodDetailsAlma>,
    pub amazon_pay: Option<stripe_shared::PaymentMethodDetailsAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::PaymentMethodDetailsAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentMethodDetailsBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodDetailsBancontact>,
    pub billie: Option<stripe_shared::PaymentMethodDetailsBillie>,
    pub bizum: Option<stripe_shared::PaymentMethodDetailsBizum>,
    pub blik: Option<stripe_shared::PaymentMethodDetailsBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodDetailsBoleto>,
    pub card: Option<stripe_shared::PaymentMethodDetailsCard>,
    pub card_present: Option<stripe_shared::PaymentMethodDetailsCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodDetailsCashapp>,
    pub crypto: Option<stripe_shared::PaymentMethodDetailsCrypto>,
    pub customer_balance: Option<stripe_shared::PaymentMethodDetailsCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentMethodDetailsEps>,
    pub fpx: Option<stripe_shared::PaymentMethodDetailsFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodDetailsGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodDetailsGrabpay>,
    pub ideal: Option<stripe_shared::PaymentMethodDetailsIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodDetailsInteracPresent>,
    pub kakao_pay: Option<stripe_shared::PaymentMethodDetailsKakaoPay>,
    pub klarna: Option<stripe_shared::PaymentMethodDetailsKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodDetailsKonbini>,
    pub kr_card: Option<stripe_shared::PaymentMethodDetailsKrCard>,
    pub link: Option<stripe_shared::PaymentMethodDetailsLink>,
    pub mb_way: Option<stripe_shared::PaymentMethodDetailsMbWay>,
    pub mobilepay: Option<stripe_shared::PaymentMethodDetailsMobilepay>,
    pub multibanco: Option<stripe_shared::PaymentMethodDetailsMultibanco>,
    pub naver_pay: Option<stripe_shared::PaymentMethodDetailsNaverPay>,
    pub nz_bank_account: Option<stripe_shared::PaymentMethodDetailsNzBankAccount>,
    pub oxxo: Option<stripe_shared::PaymentMethodDetailsOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodDetailsP24>,
    pub pay_by_bank: Option<stripe_shared::PaymentMethodDetailsPayByBank>,
    pub payco: Option<stripe_shared::PaymentMethodDetailsPayco>,
    pub paynow: Option<stripe_shared::PaymentMethodDetailsPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodDetailsPaypal>,
    pub payto: Option<stripe_shared::PaymentMethodDetailsPayto>,
    pub pix: Option<stripe_shared::PaymentMethodDetailsPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodDetailsPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodDetailsRevolutPay>,
    pub samsung_pay: Option<stripe_shared::PaymentMethodDetailsSamsungPay>,
    pub satispay: Option<stripe_shared::PaymentMethodDetailsSatispay>,
    pub scalapay: Option<stripe_shared::PaymentMethodDetailsScalapay>,
    pub sepa_credit_transfer: Option<stripe_shared::PaymentMethodDetailsSepaCreditTransfer>,
    pub sepa_debit: Option<stripe_shared::PaymentMethodDetailsSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodDetailsSofort>,
    pub stripe_account: Option<stripe_shared::PaymentMethodDetailsStripeAccount>,
    pub sunbit: Option<stripe_shared::PaymentMethodDetailsSunbit>,
    pub swish: Option<stripe_shared::PaymentMethodDetailsSwish>,
    pub twint: Option<stripe_shared::PaymentMethodDetailsTwint>,
    /// The type of transaction-specific details of the payment method used in the payment.
    /// See [PaymentMethod.type](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type) for the full list of possible types.
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains information specific to the payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub upi: Option<stripe_shared::PaymentMethodDetailsUpi>,
    pub us_bank_account: Option<stripe_shared::PaymentMethodDetailsUsBankAccount>,
    pub wechat: Option<stripe_shared::PaymentMethodDetailsWechat>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodDetailsWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodDetailsZip>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethodDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodDetailsBuilder {
    ach_credit_transfer: Option<stripe_shared::PaymentMethodDetailsAchCreditTransfer>,
    ach_debit: Option<stripe_shared::PaymentMethodDetailsAchDebit>,
    acss_debit: Option<stripe_shared::PaymentMethodDetailsAcssDebit>,
    affirm: Option<stripe_shared::PaymentMethodDetailsAffirm>,
    afterpay_clearpay: Option<stripe_shared::PaymentMethodDetailsAfterpayClearpay>,
    alipay: Option<stripe_shared::PaymentMethodDetailsAlipay>,
    alma: Option<stripe_shared::PaymentMethodDetailsAlma>,
    amazon_pay: Option<stripe_shared::PaymentMethodDetailsAmazonPay>,
    au_becs_debit: Option<stripe_shared::PaymentMethodDetailsAuBecsDebit>,
    bacs_debit: Option<stripe_shared::PaymentMethodDetailsBacsDebit>,
    bancontact: Option<stripe_shared::PaymentMethodDetailsBancontact>,
    billie: Option<stripe_shared::PaymentMethodDetailsBillie>,
    bizum: Option<stripe_shared::PaymentMethodDetailsBizum>,
    blik: Option<stripe_shared::PaymentMethodDetailsBlik>,
    boleto: Option<stripe_shared::PaymentMethodDetailsBoleto>,
    card: Option<stripe_shared::PaymentMethodDetailsCard>,
    card_present: Option<stripe_shared::PaymentMethodDetailsCardPresent>,
    cashapp: Option<stripe_shared::PaymentMethodDetailsCashapp>,
    crypto: Option<stripe_shared::PaymentMethodDetailsCrypto>,
    customer_balance: Option<stripe_shared::PaymentMethodDetailsCustomerBalance>,
    eps: Option<stripe_shared::PaymentMethodDetailsEps>,
    fpx: Option<stripe_shared::PaymentMethodDetailsFpx>,
    giropay: Option<stripe_shared::PaymentMethodDetailsGiropay>,
    grabpay: Option<stripe_shared::PaymentMethodDetailsGrabpay>,
    ideal: Option<stripe_shared::PaymentMethodDetailsIdeal>,
    interac_present: Option<stripe_shared::PaymentMethodDetailsInteracPresent>,
    kakao_pay: Option<stripe_shared::PaymentMethodDetailsKakaoPay>,
    klarna: Option<stripe_shared::PaymentMethodDetailsKlarna>,
    konbini: Option<stripe_shared::PaymentMethodDetailsKonbini>,
    kr_card: Option<stripe_shared::PaymentMethodDetailsKrCard>,
    link: Option<stripe_shared::PaymentMethodDetailsLink>,
    mb_way: Option<stripe_shared::PaymentMethodDetailsMbWay>,
    mobilepay: Option<stripe_shared::PaymentMethodDetailsMobilepay>,
    multibanco: Option<stripe_shared::PaymentMethodDetailsMultibanco>,
    naver_pay: Option<stripe_shared::PaymentMethodDetailsNaverPay>,
    nz_bank_account: Option<stripe_shared::PaymentMethodDetailsNzBankAccount>,
    oxxo: Option<stripe_shared::PaymentMethodDetailsOxxo>,
    p24: Option<stripe_shared::PaymentMethodDetailsP24>,
    pay_by_bank: Option<stripe_shared::PaymentMethodDetailsPayByBank>,
    payco: Option<stripe_shared::PaymentMethodDetailsPayco>,
    paynow: Option<stripe_shared::PaymentMethodDetailsPaynow>,
    paypal: Option<stripe_shared::PaymentMethodDetailsPaypal>,
    payto: Option<stripe_shared::PaymentMethodDetailsPayto>,
    pix: Option<stripe_shared::PaymentMethodDetailsPix>,
    promptpay: Option<stripe_shared::PaymentMethodDetailsPromptpay>,
    revolut_pay: Option<stripe_shared::PaymentMethodDetailsRevolutPay>,
    samsung_pay: Option<stripe_shared::PaymentMethodDetailsSamsungPay>,
    satispay: Option<stripe_shared::PaymentMethodDetailsSatispay>,
    scalapay: Option<stripe_shared::PaymentMethodDetailsScalapay>,
    sepa_credit_transfer: Option<stripe_shared::PaymentMethodDetailsSepaCreditTransfer>,
    sepa_debit: Option<stripe_shared::PaymentMethodDetailsSepaDebit>,
    sofort: Option<stripe_shared::PaymentMethodDetailsSofort>,
    stripe_account: Option<stripe_shared::PaymentMethodDetailsStripeAccount>,
    sunbit: Option<stripe_shared::PaymentMethodDetailsSunbit>,
    swish: Option<stripe_shared::PaymentMethodDetailsSwish>,
    twint: Option<stripe_shared::PaymentMethodDetailsTwint>,
    type_: Option<String>,
    upi: Option<stripe_shared::PaymentMethodDetailsUpi>,
    us_bank_account: Option<stripe_shared::PaymentMethodDetailsUsBankAccount>,
    wechat: Option<stripe_shared::PaymentMethodDetailsWechat>,
    wechat_pay: Option<stripe_shared::PaymentMethodDetailsWechatPay>,
    zip: Option<stripe_shared::PaymentMethodDetailsZip>,
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
                "ach_credit_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.ach_credit_transfer)
                }
                "ach_debit" => stripe_types::miniserde_helpers::begin_nullable(&mut self.ach_debit),
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
                "sepa_credit_transfer" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sepa_credit_transfer)
                }
                "sepa_debit" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.sepa_debit)
                }
                "sofort" => stripe_types::miniserde_helpers::begin_nullable(&mut self.sofort),
                "stripe_account" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.stripe_account)
                }
                "sunbit" => stripe_types::miniserde_helpers::begin_nullable(&mut self.sunbit),
                "swish" => stripe_types::miniserde_helpers::begin_nullable(&mut self.swish),
                "twint" => stripe_types::miniserde_helpers::begin_nullable(&mut self.twint),
                "type" => Deserialize::begin(&mut self.type_),
                "upi" => stripe_types::miniserde_helpers::begin_nullable(&mut self.upi),
                "us_bank_account" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.us_bank_account)
                }
                "wechat" => stripe_types::miniserde_helpers::begin_nullable(&mut self.wechat),
                "wechat_pay" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.wechat_pay)
                }
                "zip" => stripe_types::miniserde_helpers::begin_nullable(&mut self.zip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                ach_credit_transfer: None,
                ach_debit: None,
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
                sepa_credit_transfer: None,
                sepa_debit: None,
                sofort: None,
                stripe_account: None,
                sunbit: None,
                swish: None,
                twint: None,
                type_: None,
                upi: None,
                us_bank_account: None,
                wechat: None,
                wechat_pay: None,
                zip: None,
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                ach_credit_transfer,
                ach_debit,
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
                sepa_credit_transfer,
                sepa_debit,
                sofort,
                stripe_account,
                sunbit,
                swish,
                twint,
                Some(type_),
                upi,
                us_bank_account,
                wechat,
                wechat_pay,
                zip,
            ) = (
                self.ach_credit_transfer.take(),
                self.ach_debit.take(),
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
                self.bizum.take(),
                self.blik.take(),
                self.boleto.take(),
                self.card.take(),
                self.card_present.take(),
                self.cashapp.take(),
                self.crypto.take(),
                self.customer_balance,
                self.eps.take(),
                self.fpx.take(),
                self.giropay.take(),
                self.grabpay.take(),
                self.ideal.take(),
                self.interac_present.take(),
                self.kakao_pay.take(),
                self.klarna.take(),
                self.konbini.take(),
                self.kr_card.take(),
                self.link.take(),
                self.mb_way,
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
                self.sepa_credit_transfer.take(),
                self.sepa_debit.take(),
                self.sofort.take(),
                self.stripe_account,
                self.sunbit.take(),
                self.swish.take(),
                self.twint.take(),
                self.type_.take(),
                self.upi.take(),
                self.us_bank_account.take(),
                self.wechat,
                self.wechat_pay.take(),
                self.zip,
            )
            else {
                return None;
            };
            Some(Self::Out {
                ach_credit_transfer,
                ach_debit,
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
                sepa_credit_transfer,
                sepa_debit,
                sofort,
                stripe_account,
                sunbit,
                swish,
                twint,
                type_,
                upi,
                us_bank_account,
                wechat,
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

    impl ObjectDeser for PaymentMethodDetails {
        type Builder = PaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for PaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
