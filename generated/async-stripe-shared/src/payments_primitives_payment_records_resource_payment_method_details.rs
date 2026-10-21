/// Details about the Payment Method used in this payment attempt.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails {
    pub ach_credit_transfer: Option<stripe_shared::PaymentMethodDetailsAchCreditTransfer>,
    pub ach_debit: Option<stripe_shared::PaymentMethodDetailsAchDebit>,
    pub acss_debit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAlipay>,
    pub alma: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAlma>,
    pub amazon_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAmazonPay>,
    pub au_becs_debit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBancontact>,
    pub billie: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBillie>,
    /// The billing details associated with the method of payment.
    pub billing_details:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceBillingDetails>,
    pub bizum: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBizum>,
    pub blik: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBoleto>,
    pub card:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetails>,
    pub card_present: Option<stripe_shared::PaymentMethodDetailsCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodDetailsPaymentRecordCashapp>,
    pub crypto: Option<stripe_shared::PaymentMethodDetailsCrypto>,
    pub custom:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails>,
    pub customer_balance: Option<stripe_shared::PaymentMethodDetailsCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentMethodDetailsPaymentRecordEps>,
    pub fpx: Option<stripe_shared::PaymentMethodDetailsFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodDetailsGrabpay>,
    pub ideal: Option<stripe_shared::PaymentMethodDetailsPaymentRecordIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodDetailsInteracPresent>,
    pub kakao_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordKakaoPay>,
    pub klarna: Option<stripe_shared::PaymentMethodDetailsPaymentRecordKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodDetailsPaymentRecordKonbini>,
    pub kr_card: Option<stripe_shared::PaymentMethodDetailsKrCard>,
    pub link: Option<stripe_shared::PaymentMethodDetailsPaymentRecordLink>,
    pub mb_way: Option<stripe_shared::PaymentMethodDetailsPaymentRecordMbWay>,
    pub mobilepay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordMobilepay>,
    pub multibanco: Option<stripe_shared::PaymentMethodDetailsPaymentRecordMultibanco>,
    pub naver_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordNaverPay>,
    pub nz_bank_account: Option<stripe_shared::PaymentMethodDetailsPaymentRecordNzBankAccount>,
    pub oxxo: Option<stripe_shared::PaymentMethodDetailsPaymentRecordOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodDetailsPaymentRecordP24>,
    pub pay_by_bank: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPayByBank>,
    pub payco: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPayco>,
    /// ID of the Stripe PaymentMethod used to make this payment.
    pub payment_method: Option<String>,
    pub paynow: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodDetailsPaypal>,
    pub payto: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPayto>,
    pub pix: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPromptpay>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordRevolutPay>,
    pub samsung_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSamsungPay>,
    pub satispay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSatispay>,
    pub scalapay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordScalapay>,
    pub sepa_credit_transfer: Option<stripe_shared::PaymentMethodDetailsSepaCreditTransfer>,
    pub sepa_debit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSofort>,
    pub stripe_account: Option<stripe_shared::PaymentMethodDetailsStripeAccount>,
    pub sunbit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSunbit>,
    pub swish: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSwish>,
    pub twint: Option<stripe_shared::PaymentMethodDetailsPaymentRecordTwint>,
    /// The type of transaction-specific details of the payment method used in the payment.
    /// See [PaymentMethod.type](https://docs.stripe.com/api/payment_methods/object#payment_method_object-type) for the full list of possible types.
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains information specific to the payment method.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: String,
    pub upi: Option<stripe_shared::PaymentMethodDetailsPaymentRecordUpi>,
    pub us_bank_account: Option<stripe_shared::PaymentMethodDetailsPaymentRecordUsBankAccount>,
    pub wechat: Option<stripe_shared::PaymentMethodDetailsWechat>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodDetailsPaymentRecordZip>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails")
            .finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetailsBuilder {
    ach_credit_transfer: Option<stripe_shared::PaymentMethodDetailsAchCreditTransfer>,
    ach_debit: Option<stripe_shared::PaymentMethodDetailsAchDebit>,
    acss_debit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAcssDebit>,
    affirm: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAffirm>,
    afterpay_clearpay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAfterpayClearpay>,
    alipay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAlipay>,
    alma: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAlma>,
    amazon_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAmazonPay>,
    au_becs_debit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordAuBecsDebit>,
    bacs_debit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBacsDebit>,
    bancontact: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBancontact>,
    billie: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBillie>,
    billing_details: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourceBillingDetails>,
    bizum: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBizum>,
    blik: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBlik>,
    boleto: Option<stripe_shared::PaymentMethodDetailsPaymentRecordBoleto>,
    card: Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCardDetails>,
    card_present: Option<stripe_shared::PaymentMethodDetailsCardPresent>,
    cashapp: Option<stripe_shared::PaymentMethodDetailsPaymentRecordCashapp>,
    crypto: Option<stripe_shared::PaymentMethodDetailsCrypto>,
    custom:
        Option<stripe_shared::PaymentsPrimitivesPaymentRecordsResourcePaymentMethodCustomDetails>,
    customer_balance: Option<stripe_shared::PaymentMethodDetailsCustomerBalance>,
    eps: Option<stripe_shared::PaymentMethodDetailsPaymentRecordEps>,
    fpx: Option<stripe_shared::PaymentMethodDetailsFpx>,
    giropay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordGiropay>,
    grabpay: Option<stripe_shared::PaymentMethodDetailsGrabpay>,
    ideal: Option<stripe_shared::PaymentMethodDetailsPaymentRecordIdeal>,
    interac_present: Option<stripe_shared::PaymentMethodDetailsInteracPresent>,
    kakao_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordKakaoPay>,
    klarna: Option<stripe_shared::PaymentMethodDetailsPaymentRecordKlarna>,
    konbini: Option<stripe_shared::PaymentMethodDetailsPaymentRecordKonbini>,
    kr_card: Option<stripe_shared::PaymentMethodDetailsKrCard>,
    link: Option<stripe_shared::PaymentMethodDetailsPaymentRecordLink>,
    mb_way: Option<stripe_shared::PaymentMethodDetailsPaymentRecordMbWay>,
    mobilepay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordMobilepay>,
    multibanco: Option<stripe_shared::PaymentMethodDetailsPaymentRecordMultibanco>,
    naver_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordNaverPay>,
    nz_bank_account: Option<stripe_shared::PaymentMethodDetailsPaymentRecordNzBankAccount>,
    oxxo: Option<stripe_shared::PaymentMethodDetailsPaymentRecordOxxo>,
    p24: Option<stripe_shared::PaymentMethodDetailsPaymentRecordP24>,
    pay_by_bank: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPayByBank>,
    payco: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPayco>,
    payment_method: Option<Option<String>>,
    paynow: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPaynow>,
    paypal: Option<stripe_shared::PaymentMethodDetailsPaypal>,
    payto: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPayto>,
    pix: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPix>,
    promptpay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordPromptpay>,
    revolut_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordRevolutPay>,
    samsung_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSamsungPay>,
    satispay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSatispay>,
    scalapay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordScalapay>,
    sepa_credit_transfer: Option<stripe_shared::PaymentMethodDetailsSepaCreditTransfer>,
    sepa_debit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSepaDebit>,
    sofort: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSofort>,
    stripe_account: Option<stripe_shared::PaymentMethodDetailsStripeAccount>,
    sunbit: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSunbit>,
    swish: Option<stripe_shared::PaymentMethodDetailsPaymentRecordSwish>,
    twint: Option<stripe_shared::PaymentMethodDetailsPaymentRecordTwint>,
    type_: Option<String>,
    upi: Option<stripe_shared::PaymentMethodDetailsPaymentRecordUpi>,
    us_bank_account: Option<stripe_shared::PaymentMethodDetailsPaymentRecordUsBankAccount>,
    wechat: Option<stripe_shared::PaymentMethodDetailsWechat>,
    wechat_pay: Option<stripe_shared::PaymentMethodDetailsPaymentRecordWechatPay>,
    zip: Option<stripe_shared::PaymentMethodDetailsPaymentRecordZip>,
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

    impl Deserialize for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails>,
        builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetailsBuilder,
    }

    impl Visitor for Place<PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
            out: &mut self.out,
            builder: PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetailsBuilder::deser_default(),
        }))
        }
    }

    impl MapBuilder for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetailsBuilder {
        type Out = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails;
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
                "billing_details" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.billing_details)
                }
                "bizum" => stripe_types::miniserde_helpers::begin_nullable(&mut self.bizum),
                "blik" => stripe_types::miniserde_helpers::begin_nullable(&mut self.blik),
                "boleto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.boleto),
                "card" => stripe_types::miniserde_helpers::begin_nullable(&mut self.card),
                "card_present" => {
                    stripe_types::miniserde_helpers::begin_nullable(&mut self.card_present)
                }
                "cashapp" => stripe_types::miniserde_helpers::begin_nullable(&mut self.cashapp),
                "crypto" => stripe_types::miniserde_helpers::begin_nullable(&mut self.crypto),
                "custom" => stripe_types::miniserde_helpers::begin_nullable(&mut self.custom),
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
                "payment_method" => Deserialize::begin(&mut self.payment_method),
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
                billing_details: None,
                bizum: None,
                blik: None,
                boleto: None,
                card: None,
                card_present: None,
                cashapp: None,
                crypto: None,
                custom: None,
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
                payment_method: Some(None),
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
            match core::mem::replace(self, Self::deser_default()) {
                Self {
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
                    billing_details,
                    bizum,
                    blik,
                    boleto,
                    card,
                    card_present,
                    cashapp,
                    crypto,
                    custom,
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
                    payment_method: Some(payment_method),
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
                    type_: Some(type_),
                    upi,
                    us_bank_account,
                    wechat,
                    wechat_pay,
                    zip,
                } => Some(Self::Out {
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
                    billing_details,
                    bizum,
                    blik,
                    boleto,
                    card,
                    card_present,
                    cashapp,
                    crypto,
                    custom,
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
                    payment_method,
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
                }),
                builder => {
                    *self = builder;
                    None
                }
            }
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

    impl ObjectDeser for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails {
        type Builder = PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetailsBuilder;
    }

    impl FromValueOpt for PaymentsPrimitivesPaymentRecordsResourcePaymentMethodDetails {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
