/// PaymentMethod objects represent your customer's payment instruments.
/// You can use them with [PaymentIntents](https://stripe.com/docs/payments/payment-intents) to collect payments or save them to.
/// Customer objects to store instrument details for future payments.
///
/// Related guides: [Payment Methods](https://stripe.com/docs/payments/payment-methods) and [More Payment Scenarios](https://stripe.com/docs/payments/more-payment-scenarios).
///
/// For more details see <<https://stripe.com/docs/api/payment_methods/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Serialize))]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentMethod {
    pub acss_debit: Option<stripe_shared::PaymentMethodAcssDebit>,
    pub affirm: Option<stripe_shared::PaymentMethodAffirm>,
    pub afterpay_clearpay: Option<stripe_shared::PaymentMethodAfterpayClearpay>,
    pub alipay: Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipay>,
    pub au_becs_debit: Option<stripe_shared::PaymentMethodAuBecsDebit>,
    pub bacs_debit: Option<stripe_shared::PaymentMethodBacsDebit>,
    pub bancontact: Option<stripe_shared::PaymentMethodBancontact>,
    pub billing_details: stripe_shared::BillingDetails,
    pub blik: Option<stripe_shared::PaymentMethodBlik>,
    pub boleto: Option<stripe_shared::PaymentMethodBoleto>,
    pub card: Option<stripe_shared::PaymentMethodCard>,
    pub card_present: Option<stripe_shared::PaymentMethodCardPresent>,
    pub cashapp: Option<stripe_shared::PaymentMethodCashapp>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the Customer to which this PaymentMethod is saved.
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    pub customer_balance: Option<stripe_shared::PaymentMethodCustomerBalance>,
    pub eps: Option<stripe_shared::PaymentMethodEps>,
    pub fpx: Option<stripe_shared::PaymentMethodFpx>,
    pub giropay: Option<stripe_shared::PaymentMethodGiropay>,
    pub grabpay: Option<stripe_shared::PaymentMethodGrabpay>,
    /// Unique identifier for the object.
    pub id: stripe_shared::PaymentMethodId,
    pub ideal: Option<stripe_shared::PaymentMethodIdeal>,
    pub interac_present: Option<stripe_shared::PaymentMethodInteracPresent>,
    pub klarna: Option<stripe_shared::PaymentMethodKlarna>,
    pub konbini: Option<stripe_shared::PaymentMethodKonbini>,
    pub link: Option<stripe_shared::PaymentMethodLink>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub oxxo: Option<stripe_shared::PaymentMethodOxxo>,
    pub p24: Option<stripe_shared::PaymentMethodP24>,
    pub paynow: Option<stripe_shared::PaymentMethodPaynow>,
    pub paypal: Option<stripe_shared::PaymentMethodPaypal>,
    pub pix: Option<stripe_shared::PaymentMethodPix>,
    pub promptpay: Option<stripe_shared::PaymentMethodPromptpay>,
    pub radar_options: Option<stripe_shared::RadarRadarOptions>,
    pub revolut_pay: Option<stripe_shared::PaymentMethodRevolutPay>,
    pub sepa_debit: Option<stripe_shared::PaymentMethodSepaDebit>,
    pub sofort: Option<stripe_shared::PaymentMethodSofort>,
    /// The type of the PaymentMethod.
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[cfg_attr(not(feature = "min-ser"), serde(rename = "type"))]
    pub type_: PaymentMethodType,
    pub us_bank_account: Option<stripe_shared::PaymentMethodUsBankAccount>,
    pub wechat_pay: Option<stripe_shared::PaymentMethodWechatPay>,
    pub zip: Option<stripe_shared::PaymentMethodZip>,
}
#[cfg(feature = "min-ser")]
pub struct PaymentMethodBuilder {
    acss_debit: Option<Option<stripe_shared::PaymentMethodAcssDebit>>,
    affirm: Option<Option<stripe_shared::PaymentMethodAffirm>>,
    afterpay_clearpay: Option<Option<stripe_shared::PaymentMethodAfterpayClearpay>>,
    alipay: Option<Option<stripe_shared::PaymentFlowsPrivatePaymentMethodsAlipay>>,
    au_becs_debit: Option<Option<stripe_shared::PaymentMethodAuBecsDebit>>,
    bacs_debit: Option<Option<stripe_shared::PaymentMethodBacsDebit>>,
    bancontact: Option<Option<stripe_shared::PaymentMethodBancontact>>,
    billing_details: Option<stripe_shared::BillingDetails>,
    blik: Option<Option<stripe_shared::PaymentMethodBlik>>,
    boleto: Option<Option<stripe_shared::PaymentMethodBoleto>>,
    card: Option<Option<stripe_shared::PaymentMethodCard>>,
    card_present: Option<Option<stripe_shared::PaymentMethodCardPresent>>,
    cashapp: Option<Option<stripe_shared::PaymentMethodCashapp>>,
    created: Option<stripe_types::Timestamp>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_balance: Option<Option<stripe_shared::PaymentMethodCustomerBalance>>,
    eps: Option<Option<stripe_shared::PaymentMethodEps>>,
    fpx: Option<Option<stripe_shared::PaymentMethodFpx>>,
    giropay: Option<Option<stripe_shared::PaymentMethodGiropay>>,
    grabpay: Option<Option<stripe_shared::PaymentMethodGrabpay>>,
    id: Option<stripe_shared::PaymentMethodId>,
    ideal: Option<Option<stripe_shared::PaymentMethodIdeal>>,
    interac_present: Option<Option<stripe_shared::PaymentMethodInteracPresent>>,
    klarna: Option<Option<stripe_shared::PaymentMethodKlarna>>,
    konbini: Option<Option<stripe_shared::PaymentMethodKonbini>>,
    link: Option<Option<stripe_shared::PaymentMethodLink>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    oxxo: Option<Option<stripe_shared::PaymentMethodOxxo>>,
    p24: Option<Option<stripe_shared::PaymentMethodP24>>,
    paynow: Option<Option<stripe_shared::PaymentMethodPaynow>>,
    paypal: Option<Option<stripe_shared::PaymentMethodPaypal>>,
    pix: Option<Option<stripe_shared::PaymentMethodPix>>,
    promptpay: Option<Option<stripe_shared::PaymentMethodPromptpay>>,
    radar_options: Option<Option<stripe_shared::RadarRadarOptions>>,
    revolut_pay: Option<Option<stripe_shared::PaymentMethodRevolutPay>>,
    sepa_debit: Option<Option<stripe_shared::PaymentMethodSepaDebit>>,
    sofort: Option<Option<stripe_shared::PaymentMethodSofort>>,
    type_: Option<PaymentMethodType>,
    us_bank_account: Option<Option<stripe_shared::PaymentMethodUsBankAccount>>,
    wechat_pay: Option<Option<stripe_shared::PaymentMethodWechatPay>>,
    zip: Option<Option<stripe_shared::PaymentMethodZip>>,
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
            Ok(Box::new(Builder { out: &mut self.out, builder: PaymentMethodBuilder::deser_default() }))
        }
    }

    impl MapBuilder for PaymentMethodBuilder {
        type Out = PaymentMethod;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "affirm" => Deserialize::begin(&mut self.affirm),
                "afterpay_clearpay" => Deserialize::begin(&mut self.afterpay_clearpay),
                "alipay" => Deserialize::begin(&mut self.alipay),
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.bancontact),
                "billing_details" => Deserialize::begin(&mut self.billing_details),
                "blik" => Deserialize::begin(&mut self.blik),
                "boleto" => Deserialize::begin(&mut self.boleto),
                "card" => Deserialize::begin(&mut self.card),
                "card_present" => Deserialize::begin(&mut self.card_present),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "created" => Deserialize::begin(&mut self.created),
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_balance" => Deserialize::begin(&mut self.customer_balance),
                "eps" => Deserialize::begin(&mut self.eps),
                "fpx" => Deserialize::begin(&mut self.fpx),
                "giropay" => Deserialize::begin(&mut self.giropay),
                "grabpay" => Deserialize::begin(&mut self.grabpay),
                "id" => Deserialize::begin(&mut self.id),
                "ideal" => Deserialize::begin(&mut self.ideal),
                "interac_present" => Deserialize::begin(&mut self.interac_present),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "konbini" => Deserialize::begin(&mut self.konbini),
                "link" => Deserialize::begin(&mut self.link),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "oxxo" => Deserialize::begin(&mut self.oxxo),
                "p24" => Deserialize::begin(&mut self.p24),
                "paynow" => Deserialize::begin(&mut self.paynow),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "pix" => Deserialize::begin(&mut self.pix),
                "promptpay" => Deserialize::begin(&mut self.promptpay),
                "radar_options" => Deserialize::begin(&mut self.radar_options),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
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
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                billing_details: Deserialize::default(),
                blik: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                cashapp: Deserialize::default(),
                created: Deserialize::default(),
                customer: Deserialize::default(),
                customer_balance: Deserialize::default(),
                eps: Deserialize::default(),
                fpx: Deserialize::default(),
                giropay: Deserialize::default(),
                grabpay: Deserialize::default(),
                id: Deserialize::default(),
                ideal: Deserialize::default(),
                interac_present: Deserialize::default(),
                klarna: Deserialize::default(),
                konbini: Deserialize::default(),
                link: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                pix: Deserialize::default(),
                promptpay: Deserialize::default(),
                radar_options: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                type_: Deserialize::default(),
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
            let billing_details = self.billing_details.take()?;
            let blik = self.blik.take()?;
            let boleto = self.boleto.take()?;
            let card = self.card.take()?;
            let card_present = self.card_present.take()?;
            let cashapp = self.cashapp.take()?;
            let created = self.created.take()?;
            let customer = self.customer.take()?;
            let customer_balance = self.customer_balance.take()?;
            let eps = self.eps.take()?;
            let fpx = self.fpx.take()?;
            let giropay = self.giropay.take()?;
            let grabpay = self.grabpay.take()?;
            let id = self.id.take()?;
            let ideal = self.ideal.take()?;
            let interac_present = self.interac_present.take()?;
            let klarna = self.klarna.take()?;
            let konbini = self.konbini.take()?;
            let link = self.link.take()?;
            let livemode = self.livemode.take()?;
            let metadata = self.metadata.take()?;
            let oxxo = self.oxxo.take()?;
            let p24 = self.p24.take()?;
            let paynow = self.paynow.take()?;
            let paypal = self.paypal.take()?;
            let pix = self.pix.take()?;
            let promptpay = self.promptpay.take()?;
            let radar_options = self.radar_options.take()?;
            let revolut_pay = self.revolut_pay.take()?;
            let sepa_debit = self.sepa_debit.take()?;
            let sofort = self.sofort.take()?;
            let type_ = self.type_.take()?;
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
                billing_details,
                blik,
                boleto,
                card,
                card_present,
                cashapp,
                created,
                customer,
                customer_balance,
                eps,
                fpx,
                giropay,
                grabpay,
                id,
                ideal,
                interac_present,
                klarna,
                konbini,
                link,
                livemode,
                metadata,
                oxxo,
                p24,
                paynow,
                paypal,
                pix,
                promptpay,
                radar_options,
                revolut_pay,
                sepa_debit,
                sofort,
                type_,
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

    impl ObjectDeser for PaymentMethod {
        type Builder = PaymentMethodBuilder;
    }

    impl FromValueOpt for PaymentMethod {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit" => b.acss_debit = Some(FromValueOpt::from_value(v)?),
                    "affirm" => b.affirm = Some(FromValueOpt::from_value(v)?),
                    "afterpay_clearpay" => b.afterpay_clearpay = Some(FromValueOpt::from_value(v)?),
                    "alipay" => b.alipay = Some(FromValueOpt::from_value(v)?),
                    "au_becs_debit" => b.au_becs_debit = Some(FromValueOpt::from_value(v)?),
                    "bacs_debit" => b.bacs_debit = Some(FromValueOpt::from_value(v)?),
                    "bancontact" => b.bancontact = Some(FromValueOpt::from_value(v)?),
                    "billing_details" => b.billing_details = Some(FromValueOpt::from_value(v)?),
                    "blik" => b.blik = Some(FromValueOpt::from_value(v)?),
                    "boleto" => b.boleto = Some(FromValueOpt::from_value(v)?),
                    "card" => b.card = Some(FromValueOpt::from_value(v)?),
                    "card_present" => b.card_present = Some(FromValueOpt::from_value(v)?),
                    "cashapp" => b.cashapp = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "customer_balance" => b.customer_balance = Some(FromValueOpt::from_value(v)?),
                    "eps" => b.eps = Some(FromValueOpt::from_value(v)?),
                    "fpx" => b.fpx = Some(FromValueOpt::from_value(v)?),
                    "giropay" => b.giropay = Some(FromValueOpt::from_value(v)?),
                    "grabpay" => b.grabpay = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "ideal" => b.ideal = Some(FromValueOpt::from_value(v)?),
                    "interac_present" => b.interac_present = Some(FromValueOpt::from_value(v)?),
                    "klarna" => b.klarna = Some(FromValueOpt::from_value(v)?),
                    "konbini" => b.konbini = Some(FromValueOpt::from_value(v)?),
                    "link" => b.link = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "oxxo" => b.oxxo = Some(FromValueOpt::from_value(v)?),
                    "p24" => b.p24 = Some(FromValueOpt::from_value(v)?),
                    "paynow" => b.paynow = Some(FromValueOpt::from_value(v)?),
                    "paypal" => b.paypal = Some(FromValueOpt::from_value(v)?),
                    "pix" => b.pix = Some(FromValueOpt::from_value(v)?),
                    "promptpay" => b.promptpay = Some(FromValueOpt::from_value(v)?),
                    "radar_options" => b.radar_options = Some(FromValueOpt::from_value(v)?),
                    "revolut_pay" => b.revolut_pay = Some(FromValueOpt::from_value(v)?),
                    "sepa_debit" => b.sepa_debit = Some(FromValueOpt::from_value(v)?),
                    "sofort" => b.sofort = Some(FromValueOpt::from_value(v)?),
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
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
/// The type of the PaymentMethod.
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentMethodType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CardPresent,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    RevolutPay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}
impl PaymentMethodType {
    pub fn as_str(self) -> &'static str {
        use PaymentMethodType::*;
        match self {
            AcssDebit => "acss_debit",
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            CardPresent => "card_present",
            Cashapp => "cashapp",
            CustomerBalance => "customer_balance",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
            InteracPresent => "interac_present",
            Klarna => "klarna",
            Konbini => "konbini",
            Link => "link",
            Oxxo => "oxxo",
            P24 => "p24",
            Paynow => "paynow",
            Paypal => "paypal",
            Pix => "pix",
            Promptpay => "promptpay",
            RevolutPay => "revolut_pay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentMethodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodType::*;
        match s {
            "acss_debit" => Ok(AcssDebit),
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "card_present" => Ok(CardPresent),
            "cashapp" => Ok(Cashapp),
            "customer_balance" => Ok(CustomerBalance),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
            "interac_present" => Ok(InteracPresent),
            "klarna" => Ok(Klarna),
            "konbini" => Ok(Konbini),
            "link" => Ok(Link),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "revolut_pay" => Ok(RevolutPay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentMethodType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentMethodType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<PaymentMethodType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentMethodType::from_str(s).unwrap_or(PaymentMethodType::Unknown));
        Ok(())
    }
}

#[cfg(feature = "min-ser")]
stripe_types::impl_from_val_with_from_str!(PaymentMethodType);
impl stripe_types::Object for PaymentMethod {
    type Id = stripe_shared::PaymentMethodId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(PaymentMethodId, "pm_" | "card_" | "src_" | "ba_");
