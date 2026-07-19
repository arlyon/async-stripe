/// PaymentMethod objects represent your customer's payment instruments.
/// You can use them with [PaymentIntents](https://docs.stripe.com/payments/payment-intents) to collect payments or save them to.
/// Customer objects to store instrument details for future payments.
///
/// Related guides: [Payment Methods](https://docs.stripe.com/payments/payment-methods) and [More Payment Scenarios](https://docs.stripe.com/payments/more-payment-scenarios).
///
/// For more details see <<https://stripe.com/docs/api/payment_methods/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethod {
    pub acss_debit: Option<stripe_shared::PaymentMethodAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipay>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to “unspecified”.
    pub allow_redisplay: Option<stripe_shared::PaymentMethodAllowRedisplay>,
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
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    pub crypto: Option<stripe_shared::PaymentMethodCrypto>,
    pub custom: Option<stripe_shared::PaymentMethodCustom>,
    /// The ID of the Customer to which this PaymentMethod is saved.
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    pub customer_account: Option<String>,
    pub customer_balance: Option<stripe_shared::PaymentMethodCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentMethodEps>,
    pub fpx: Option<stripe_shared::PaymentMethodFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodGrabpay>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PaymentMethodId,
    pub ideal: Option<stripe_shared::PaymentMethodIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodInteracPresent>,
    pub kakao_pay: Option<stripe_shared::PaymentMethodKakaoPay>,
    pub klarna: Option<stripe_shared::PaymentMethodKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodKonbini>,
    pub kr_card: Option<stripe_shared::PaymentMethodKrCard>,
    pub link: Option<stripe_shared::PaymentMethodLink>,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    pub mb_way: Option<stripe_shared::PaymentMethodMbWay>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
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
    pub payto: Option<stripe_shared::PaymentMethodPayto>,
    pub pix: Option<stripe_shared::PaymentMethodPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodPromptpay>,
    pub radar_options: Option<stripe_shared::RadarRadarOptions>,
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
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: PaymentMethodType,
    pub upi: Option<stripe_shared::PaymentMethodUpi>,
    pub us_bank_account: Option<stripe_shared::PaymentMethodUsBankAccount>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodZip>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentMethod").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PaymentMethodBuilder {
    acss_debit: Option<Option<stripe_shared::PaymentMethodAcssDebit>>,
    affirm: Option<Option<stripe_shared::PaymentMethodAffirm>>,
    afterpay_clearpay: Option<Option<stripe_shared::PaymentMethodAfterpayClearpay>>,
    alipay: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipay>>,
    allow_redisplay: Option<Option<stripe_shared::PaymentMethodAllowRedisplay>>,
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
    created: Option<stripe_types::Timestamp>,
    crypto: Option<Option<stripe_shared::PaymentMethodCrypto>>,
    custom: Option<Option<stripe_shared::PaymentMethodCustom>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_account: Option<Option<String>>,
    customer_balance: Option<Option<stripe_shared::PaymentMethodCustomerBalance>>,
    eps: Option<Option<stripe_shared::PaymentMethodEps>>,
    fpx: Option<Option<stripe_shared::PaymentMethodFpx>>,
    giropay: Option<Option<stripe_shared::PaymentMethodGiropay>>,
    grabpay: Option<Option<stripe_shared::PaymentMethodGrabpay>>,
    id: Option<stripe_shared::PaymentMethodId>,
    ideal: Option<Option<stripe_shared::PaymentMethodIdeal>>,
    interac_present: Option<Option<stripe_shared::PaymentMethodInteracPresent>>,
    kakao_pay: Option<Option<stripe_shared::PaymentMethodKakaoPay>>,
    klarna: Option<Option<stripe_shared::PaymentMethodKlarna>>,
    konbini: Option<Option<stripe_shared::PaymentMethodKonbini>>,
    kr_card: Option<Option<stripe_shared::PaymentMethodKrCard>>,
    link: Option<Option<stripe_shared::PaymentMethodLink>>,
    livemode: Option<bool>,
    mb_way: Option<Option<stripe_shared::PaymentMethodMbWay>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
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
    payto: Option<Option<stripe_shared::PaymentMethodPayto>>,
    pix: Option<Option<stripe_shared::PaymentMethodPix>>,
    promptpay: Option<Option<stripe_shared::PaymentMethodPromptpay>>,
    radar_options: Option<Option<stripe_shared::RadarRadarOptions>>,
    revolut_pay: Option<Option<stripe_shared::PaymentMethodRevolutPay>>,
    samsung_pay: Option<Option<stripe_shared::PaymentMethodSamsungPay>>,
    satispay: Option<Option<stripe_shared::PaymentMethodSatispay>>,
    sepa_debit: Option<Option<stripe_shared::PaymentMethodSepaDebit>>,
    sofort: Option<Option<stripe_shared::PaymentMethodSofort>>,
    swish: Option<Option<stripe_shared::PaymentMethodSwish>>,
    twint: Option<Option<stripe_shared::PaymentMethodTwint>>,
    type_: Option<PaymentMethodType>,
    upi: Option<Option<stripe_shared::PaymentMethodUpi>>,
    us_bank_account: Option<Option<stripe_shared::PaymentMethodUsBankAccount>>,
    wechat_pay: Option<Option<stripe_shared::PaymentMethodWechatPay>>,
    zip: Option<Option<stripe_shared::PaymentMethodZip>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for PaymentMethod {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethod>,
        builder: PaymentMethodBuilder,
    }

    impl Visitor for Place<PaymentMethod> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodBuilder {
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
                    created: Deserialize::default(),
                    crypto: Deserialize::default(),
                    custom: Deserialize::default(),
                    customer: Deserialize::default(),
                    customer_account: Deserialize::default(),
                    customer_balance: Deserialize::default(),
                    eps: Deserialize::default(),
                    fpx: Deserialize::default(),
                    giropay: Deserialize::default(),
                    grabpay: Deserialize::default(),
                    id: Deserialize::default(),
                    ideal: Deserialize::default(),
                    interac_present: Deserialize::default(),
                    kakao_pay: Deserialize::default(),
                    klarna: Deserialize::default(),
                    konbini: Deserialize::default(),
                    kr_card: Deserialize::default(),
                    link: Deserialize::default(),
                    livemode: Deserialize::default(),
                    mb_way: Deserialize::default(),
                    metadata: Deserialize::default(),
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
                    payto: Deserialize::default(),
                    pix: Deserialize::default(),
                    promptpay: Deserialize::default(),
                    radar_options: Deserialize::default(),
                    revolut_pay: Deserialize::default(),
                    samsung_pay: Deserialize::default(),
                    satispay: Deserialize::default(),
                    sepa_debit: Deserialize::default(),
                    sofort: Deserialize::default(),
                    swish: Deserialize::default(),
                    twint: Deserialize::default(),
                    type_: Deserialize::default(),
                    upi: Deserialize::default(),
                    us_bank_account: Deserialize::default(),
                    wechat_pay: Deserialize::default(),
                    zip: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.builder.acss_debit),
                "affirm" => Deserialize::begin(&mut self.builder.affirm),
                "afterpay_clearpay" => Deserialize::begin(&mut self.builder.afterpay_clearpay),
                "alipay" => Deserialize::begin(&mut self.builder.alipay),
                "allow_redisplay" => Deserialize::begin(&mut self.builder.allow_redisplay),
                "alma" => Deserialize::begin(&mut self.builder.alma),
                "amazon_pay" => Deserialize::begin(&mut self.builder.amazon_pay),
                "au_becs_debit" => Deserialize::begin(&mut self.builder.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.builder.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.builder.bancontact),
                "billie" => Deserialize::begin(&mut self.builder.billie),
                "billing_details" => Deserialize::begin(&mut self.builder.billing_details),
                "blik" => Deserialize::begin(&mut self.builder.blik),
                "boleto" => Deserialize::begin(&mut self.builder.boleto),
                "card" => Deserialize::begin(&mut self.builder.card),
                "card_present" => Deserialize::begin(&mut self.builder.card_present),
                "cashapp" => Deserialize::begin(&mut self.builder.cashapp),
                "created" => Deserialize::begin(&mut self.builder.created),
                "crypto" => Deserialize::begin(&mut self.builder.crypto),
                "custom" => Deserialize::begin(&mut self.builder.custom),
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "customer_account" => Deserialize::begin(&mut self.builder.customer_account),
                "customer_balance" => Deserialize::begin(&mut self.builder.customer_balance),
                "eps" => Deserialize::begin(&mut self.builder.eps),
                "fpx" => Deserialize::begin(&mut self.builder.fpx),
                "giropay" => Deserialize::begin(&mut self.builder.giropay),
                "grabpay" => Deserialize::begin(&mut self.builder.grabpay),
                "id" => Deserialize::begin(&mut self.builder.id),
                "ideal" => Deserialize::begin(&mut self.builder.ideal),
                "interac_present" => Deserialize::begin(&mut self.builder.interac_present),
                "kakao_pay" => Deserialize::begin(&mut self.builder.kakao_pay),
                "klarna" => Deserialize::begin(&mut self.builder.klarna),
                "konbini" => Deserialize::begin(&mut self.builder.konbini),
                "kr_card" => Deserialize::begin(&mut self.builder.kr_card),
                "link" => Deserialize::begin(&mut self.builder.link),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "mb_way" => Deserialize::begin(&mut self.builder.mb_way),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "mobilepay" => Deserialize::begin(&mut self.builder.mobilepay),
                "multibanco" => Deserialize::begin(&mut self.builder.multibanco),
                "naver_pay" => Deserialize::begin(&mut self.builder.naver_pay),
                "nz_bank_account" => Deserialize::begin(&mut self.builder.nz_bank_account),
                "oxxo" => Deserialize::begin(&mut self.builder.oxxo),
                "p24" => Deserialize::begin(&mut self.builder.p24),
                "pay_by_bank" => Deserialize::begin(&mut self.builder.pay_by_bank),
                "payco" => Deserialize::begin(&mut self.builder.payco),
                "paynow" => Deserialize::begin(&mut self.builder.paynow),
                "paypal" => Deserialize::begin(&mut self.builder.paypal),
                "payto" => Deserialize::begin(&mut self.builder.payto),
                "pix" => Deserialize::begin(&mut self.builder.pix),
                "promptpay" => Deserialize::begin(&mut self.builder.promptpay),
                "radar_options" => Deserialize::begin(&mut self.builder.radar_options),
                "revolut_pay" => Deserialize::begin(&mut self.builder.revolut_pay),
                "samsung_pay" => Deserialize::begin(&mut self.builder.samsung_pay),
                "satispay" => Deserialize::begin(&mut self.builder.satispay),
                "sepa_debit" => Deserialize::begin(&mut self.builder.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.builder.sofort),
                "swish" => Deserialize::begin(&mut self.builder.swish),
                "twint" => Deserialize::begin(&mut self.builder.twint),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "upi" => Deserialize::begin(&mut self.builder.upi),
                "us_bank_account" => Deserialize::begin(&mut self.builder.us_bank_account),
                "wechat_pay" => Deserialize::begin(&mut self.builder.wechat_pay),
                "zip" => Deserialize::begin(&mut self.builder.zip),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
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
                Some(created),
                Some(crypto),
                Some(custom),
                Some(customer),
                Some(customer_account),
                Some(customer_balance),
                Some(eps),
                Some(fpx),
                Some(giropay),
                Some(grabpay),
                Some(id),
                Some(ideal),
                Some(interac_present),
                Some(kakao_pay),
                Some(klarna),
                Some(konbini),
                Some(kr_card),
                Some(link),
                Some(livemode),
                Some(mb_way),
                Some(metadata),
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
                Some(payto),
                Some(pix),
                Some(promptpay),
                Some(radar_options),
                Some(revolut_pay),
                Some(samsung_pay),
                Some(satispay),
                Some(sepa_debit),
                Some(sofort),
                Some(swish),
                Some(twint),
                Some(type_),
                Some(upi),
                Some(us_bank_account),
                Some(wechat_pay),
                Some(zip),
            ) = (
                self.builder.acss_debit.take(),
                self.builder.affirm,
                self.builder.afterpay_clearpay,
                self.builder.alipay,
                self.builder.allow_redisplay.take(),
                self.builder.alma,
                self.builder.amazon_pay,
                self.builder.au_becs_debit.take(),
                self.builder.bacs_debit.take(),
                self.builder.bancontact,
                self.builder.billie,
                self.builder.billing_details.take(),
                self.builder.blik,
                self.builder.boleto.take(),
                self.builder.card.take(),
                self.builder.card_present.take(),
                self.builder.cashapp.take(),
                self.builder.created,
                self.builder.crypto,
                self.builder.custom.take(),
                self.builder.customer.take(),
                self.builder.customer_account.take(),
                self.builder.customer_balance,
                self.builder.eps.take(),
                self.builder.fpx.take(),
                self.builder.giropay,
                self.builder.grabpay,
                self.builder.id.take(),
                self.builder.ideal.take(),
                self.builder.interac_present.take(),
                self.builder.kakao_pay,
                self.builder.klarna,
                self.builder.konbini,
                self.builder.kr_card.take(),
                self.builder.link.take(),
                self.builder.livemode,
                self.builder.mb_way,
                self.builder.metadata.take(),
                self.builder.mobilepay,
                self.builder.multibanco,
                self.builder.naver_pay.take(),
                self.builder.nz_bank_account.take(),
                self.builder.oxxo,
                self.builder.p24.take(),
                self.builder.pay_by_bank,
                self.builder.payco,
                self.builder.paynow,
                self.builder.paypal.take(),
                self.builder.payto.take(),
                self.builder.pix,
                self.builder.promptpay,
                self.builder.radar_options.take(),
                self.builder.revolut_pay,
                self.builder.samsung_pay,
                self.builder.satispay,
                self.builder.sepa_debit.take(),
                self.builder.sofort.take(),
                self.builder.swish,
                self.builder.twint,
                self.builder.type_.take(),
                self.builder.upi.take(),
                self.builder.us_bank_account.take(),
                self.builder.wechat_pay,
                self.builder.zip,
            )
            else {
                return Ok(());
            };
            *self.out = Some(PaymentMethod {
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
                created,
                crypto,
                custom,
                customer,
                customer_account,
                customer_balance,
                eps,
                fpx,
                giropay,
                grabpay,
                id,
                ideal,
                interac_present,
                kakao_pay,
                klarna,
                konbini,
                kr_card,
                link,
                livemode,
                mb_way,
                metadata,
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
                radar_options,
                revolut_pay,
                samsung_pay,
                satispay,
                sepa_debit,
                sofort,
                swish,
                twint,
                type_,
                upi,
                us_bank_account,
                wechat_pay,
                zip,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethod {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentMethod", 65)?;
        s.serialize_field("acss_debit", &self.acss_debit)?;
        s.serialize_field("affirm", &self.affirm)?;
        s.serialize_field("afterpay_clearpay", &self.afterpay_clearpay)?;
        s.serialize_field("alipay", &self.alipay)?;
        s.serialize_field("allow_redisplay", &self.allow_redisplay)?;
        s.serialize_field("alma", &self.alma)?;
        s.serialize_field("amazon_pay", &self.amazon_pay)?;
        s.serialize_field("au_becs_debit", &self.au_becs_debit)?;
        s.serialize_field("bacs_debit", &self.bacs_debit)?;
        s.serialize_field("bancontact", &self.bancontact)?;
        s.serialize_field("billie", &self.billie)?;
        s.serialize_field("billing_details", &self.billing_details)?;
        s.serialize_field("blik", &self.blik)?;
        s.serialize_field("boleto", &self.boleto)?;
        s.serialize_field("card", &self.card)?;
        s.serialize_field("card_present", &self.card_present)?;
        s.serialize_field("cashapp", &self.cashapp)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("crypto", &self.crypto)?;
        s.serialize_field("custom", &self.custom)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_account", &self.customer_account)?;
        s.serialize_field("customer_balance", &self.customer_balance)?;
        s.serialize_field("eps", &self.eps)?;
        s.serialize_field("fpx", &self.fpx)?;
        s.serialize_field("giropay", &self.giropay)?;
        s.serialize_field("grabpay", &self.grabpay)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("ideal", &self.ideal)?;
        s.serialize_field("interac_present", &self.interac_present)?;
        s.serialize_field("kakao_pay", &self.kakao_pay)?;
        s.serialize_field("klarna", &self.klarna)?;
        s.serialize_field("konbini", &self.konbini)?;
        s.serialize_field("kr_card", &self.kr_card)?;
        s.serialize_field("link", &self.link)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("mb_way", &self.mb_way)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("mobilepay", &self.mobilepay)?;
        s.serialize_field("multibanco", &self.multibanco)?;
        s.serialize_field("naver_pay", &self.naver_pay)?;
        s.serialize_field("nz_bank_account", &self.nz_bank_account)?;
        s.serialize_field("oxxo", &self.oxxo)?;
        s.serialize_field("p24", &self.p24)?;
        s.serialize_field("pay_by_bank", &self.pay_by_bank)?;
        s.serialize_field("payco", &self.payco)?;
        s.serialize_field("paynow", &self.paynow)?;
        s.serialize_field("paypal", &self.paypal)?;
        s.serialize_field("payto", &self.payto)?;
        s.serialize_field("pix", &self.pix)?;
        s.serialize_field("promptpay", &self.promptpay)?;
        s.serialize_field("radar_options", &self.radar_options)?;
        s.serialize_field("revolut_pay", &self.revolut_pay)?;
        s.serialize_field("samsung_pay", &self.samsung_pay)?;
        s.serialize_field("satispay", &self.satispay)?;
        s.serialize_field("sepa_debit", &self.sepa_debit)?;
        s.serialize_field("sofort", &self.sofort)?;
        s.serialize_field("swish", &self.swish)?;
        s.serialize_field("twint", &self.twint)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("upi", &self.upi)?;
        s.serialize_field("us_bank_account", &self.us_bank_account)?;
        s.serialize_field("wechat_pay", &self.wechat_pay)?;
        s.serialize_field("zip", &self.zip)?;

        s.serialize_field("object", "payment_method")?;
        s.end()
    }
}
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodType {
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
    Custom,
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
    Payto,
    Pix,
    Promptpay,
    RevolutPay,
    SamsungPay,
    Satispay,
    SepaDebit,
    Sofort,
    Swish,
    Twint,
    Upi,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodType {
    pub fn as_str(&self) -> &str {
        use PaymentMethodType::*;
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
            Custom => "custom",
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
            Payto => "payto",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SamsungPay => "samsung_pay",
            Satispay => "satispay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            Twint => "twint",
            Upi => "upi",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodType::*;
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
            "custom" => Ok(Custom),
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
            "payto" => Ok(Payto),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "samsung_pay" => Ok(SamsungPay),
            "satispay" => Ok(Satispay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "twint" => Ok(Twint),
            "upi" => Ok(Upi),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PaymentMethodType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodType)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
impl stripe_types::Object for PaymentMethod {
    type Id = stripe_shared::PaymentMethodId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PaymentMethodId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodAllowRedisplay {
    Always,
    Limited,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentMethodAllowRedisplay {
    pub fn as_str(&self) -> &str {
        use PaymentMethodAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentMethodAllowRedisplay {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentMethodAllowRedisplay"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for PaymentMethodAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for PaymentMethodAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentMethodAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(PaymentMethodAllowRedisplay)).finish_non_exhaustive()
    }
}
impl serde::Serialize for PaymentMethodAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for PaymentMethodAllowRedisplay {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<PaymentMethodAllowRedisplay> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodAllowRedisplay::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentMethodAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
