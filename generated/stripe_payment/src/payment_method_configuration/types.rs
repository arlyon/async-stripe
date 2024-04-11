/// PaymentMethodConfigurations control which payment methods are displayed to your customers when you don't explicitly specify payment method types.
/// You can have multiple configurations with different sets of payment methods for different scenarios.
///
/// There are two types of PaymentMethodConfigurations.
/// Which is used depends on the [charge type](https://stripe.com/docs/connect/charges):.
///
/// **Direct** configurations apply to payments created on your account, including Connect destination charges, Connect separate charges and transfers, and payments not involving Connect.
///
/// **Child** configurations apply to payments created on your connected accounts using direct charges, and charges with the on_behalf_of parameter.
///
/// Child configurations have a `parent` that sets default values and controls which settings connected accounts may override.
/// You can specify a parent ID at payment time, and Stripe will automatically resolve the connected account’s associated child configuration.
/// Parent configurations are [managed in the dashboard](https://dashboard.stripe.com/settings/payment_methods/connected_accounts) and are not available in this API.
///
/// Related guides:
/// - [Payment Method Configurations API](https://stripe.com/docs/connect/payment-method-configurations).
/// - [Multiple configurations on dynamic payment methods](https://stripe.com/docs/payments/multiple-payment-method-configs).
/// - [Multiple configurations for your Connect accounts](https://stripe.com/docs/connect/multiple-payment-method-configurations).
///
/// For more details see <<https://stripe.com/docs/api/payment_method_configurations/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentMethodConfiguration {
    pub acss_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Whether the configuration can be used for new payments.
    pub active: bool,
    pub affirm: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub afterpay_clearpay:
        Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub alipay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub apple_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// For child configs, the Connect application associated with the configuration.
    pub application: Option<String>,
    pub au_becs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub bacs_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub bancontact: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub blik: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub boleto: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub card: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub cartes_bancaires:
        Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub cashapp: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub customer_balance:
        Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub eps: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub fpx: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub giropay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub google_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub grabpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Unique identifier for the object.
    pub id: stripe_payment::PaymentMethodConfigurationId,
    pub ideal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// The default configuration is used whenever a payment method configuration is not specified.
    pub is_default: bool,
    pub jcb: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub klarna: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub konbini: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub link: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The configuration's name.
    pub name: String,
    pub oxxo: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub p24: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    /// For child configs, the configuration's parent configuration.
    pub parent: Option<String>,
    pub paynow: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub paypal: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub promptpay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub revolut_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sepa_debit: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub sofort: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub us_bank_account: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
    pub wechat_pay: Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>,
}
#[doc(hidden)]
pub struct PaymentMethodConfigurationBuilder {
    acss_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    active: Option<bool>,
    affirm: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    afterpay_clearpay:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    alipay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    apple_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    application: Option<Option<String>>,
    au_becs_debit:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    bacs_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    bancontact: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    blik: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    boleto: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    card: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    cartes_bancaires:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    cashapp: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    customer_balance:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    eps: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    fpx: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    giropay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    google_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    grabpay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    id: Option<stripe_payment::PaymentMethodConfigurationId>,
    ideal: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    is_default: Option<bool>,
    jcb: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    klarna: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    konbini: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    link: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    livemode: Option<bool>,
    name: Option<String>,
    oxxo: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    p24: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    parent: Option<Option<String>>,
    paynow: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    paypal: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    promptpay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    revolut_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    sepa_debit: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    sofort: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    us_bank_account:
        Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
    wechat_pay: Option<Option<stripe_payment::PaymentMethodConfigResourcePaymentMethodProperties>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentMethodConfiguration {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentMethodConfiguration>,
        builder: PaymentMethodConfigurationBuilder,
    }

    impl Visitor for Place<PaymentMethodConfiguration> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentMethodConfigurationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentMethodConfigurationBuilder {
        type Out = PaymentMethodConfiguration;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "active" => Deserialize::begin(&mut self.active),
                "affirm" => Deserialize::begin(&mut self.affirm),
                "afterpay_clearpay" => Deserialize::begin(&mut self.afterpay_clearpay),
                "alipay" => Deserialize::begin(&mut self.alipay),
                "apple_pay" => Deserialize::begin(&mut self.apple_pay),
                "application" => Deserialize::begin(&mut self.application),
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bacs_debit" => Deserialize::begin(&mut self.bacs_debit),
                "bancontact" => Deserialize::begin(&mut self.bancontact),
                "blik" => Deserialize::begin(&mut self.blik),
                "boleto" => Deserialize::begin(&mut self.boleto),
                "card" => Deserialize::begin(&mut self.card),
                "cartes_bancaires" => Deserialize::begin(&mut self.cartes_bancaires),
                "cashapp" => Deserialize::begin(&mut self.cashapp),
                "customer_balance" => Deserialize::begin(&mut self.customer_balance),
                "eps" => Deserialize::begin(&mut self.eps),
                "fpx" => Deserialize::begin(&mut self.fpx),
                "giropay" => Deserialize::begin(&mut self.giropay),
                "google_pay" => Deserialize::begin(&mut self.google_pay),
                "grabpay" => Deserialize::begin(&mut self.grabpay),
                "id" => Deserialize::begin(&mut self.id),
                "ideal" => Deserialize::begin(&mut self.ideal),
                "is_default" => Deserialize::begin(&mut self.is_default),
                "jcb" => Deserialize::begin(&mut self.jcb),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "konbini" => Deserialize::begin(&mut self.konbini),
                "link" => Deserialize::begin(&mut self.link),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "name" => Deserialize::begin(&mut self.name),
                "oxxo" => Deserialize::begin(&mut self.oxxo),
                "p24" => Deserialize::begin(&mut self.p24),
                "parent" => Deserialize::begin(&mut self.parent),
                "paynow" => Deserialize::begin(&mut self.paynow),
                "paypal" => Deserialize::begin(&mut self.paypal),
                "promptpay" => Deserialize::begin(&mut self.promptpay),
                "revolut_pay" => Deserialize::begin(&mut self.revolut_pay),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
                "us_bank_account" => Deserialize::begin(&mut self.us_bank_account),
                "wechat_pay" => Deserialize::begin(&mut self.wechat_pay),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                acss_debit: Deserialize::default(),
                active: Deserialize::default(),
                affirm: Deserialize::default(),
                afterpay_clearpay: Deserialize::default(),
                alipay: Deserialize::default(),
                apple_pay: Deserialize::default(),
                application: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bacs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                blik: Deserialize::default(),
                boleto: Deserialize::default(),
                card: Deserialize::default(),
                cartes_bancaires: Deserialize::default(),
                cashapp: Deserialize::default(),
                customer_balance: Deserialize::default(),
                eps: Deserialize::default(),
                fpx: Deserialize::default(),
                giropay: Deserialize::default(),
                google_pay: Deserialize::default(),
                grabpay: Deserialize::default(),
                id: Deserialize::default(),
                ideal: Deserialize::default(),
                is_default: Deserialize::default(),
                jcb: Deserialize::default(),
                klarna: Deserialize::default(),
                konbini: Deserialize::default(),
                link: Deserialize::default(),
                livemode: Deserialize::default(),
                name: Deserialize::default(),
                oxxo: Deserialize::default(),
                p24: Deserialize::default(),
                parent: Deserialize::default(),
                paynow: Deserialize::default(),
                paypal: Deserialize::default(),
                promptpay: Deserialize::default(),
                revolut_pay: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                us_bank_account: Deserialize::default(),
                wechat_pay: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                acss_debit: self.acss_debit?,
                active: self.active?,
                affirm: self.affirm?,
                afterpay_clearpay: self.afterpay_clearpay?,
                alipay: self.alipay?,
                apple_pay: self.apple_pay?,
                application: self.application.take()?,
                au_becs_debit: self.au_becs_debit?,
                bacs_debit: self.bacs_debit?,
                bancontact: self.bancontact?,
                blik: self.blik?,
                boleto: self.boleto?,
                card: self.card?,
                cartes_bancaires: self.cartes_bancaires?,
                cashapp: self.cashapp?,
                customer_balance: self.customer_balance?,
                eps: self.eps?,
                fpx: self.fpx?,
                giropay: self.giropay?,
                google_pay: self.google_pay?,
                grabpay: self.grabpay?,
                id: self.id.take()?,
                ideal: self.ideal?,
                is_default: self.is_default?,
                jcb: self.jcb?,
                klarna: self.klarna?,
                konbini: self.konbini?,
                link: self.link?,
                livemode: self.livemode?,
                name: self.name.take()?,
                oxxo: self.oxxo?,
                p24: self.p24?,
                parent: self.parent.take()?,
                paynow: self.paynow?,
                paypal: self.paypal?,
                promptpay: self.promptpay?,
                revolut_pay: self.revolut_pay?,
                sepa_debit: self.sepa_debit?,
                sofort: self.sofort?,
                us_bank_account: self.us_bank_account?,
                wechat_pay: self.wechat_pay?,
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

    impl ObjectDeser for PaymentMethodConfiguration {
        type Builder = PaymentMethodConfigurationBuilder;
    }

    impl FromValueOpt for PaymentMethodConfiguration {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentMethodConfigurationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "acss_debit" => b.acss_debit = Some(FromValueOpt::from_value(v)?),
                    "active" => b.active = Some(FromValueOpt::from_value(v)?),
                    "affirm" => b.affirm = Some(FromValueOpt::from_value(v)?),
                    "afterpay_clearpay" => b.afterpay_clearpay = Some(FromValueOpt::from_value(v)?),
                    "alipay" => b.alipay = Some(FromValueOpt::from_value(v)?),
                    "apple_pay" => b.apple_pay = Some(FromValueOpt::from_value(v)?),
                    "application" => b.application = Some(FromValueOpt::from_value(v)?),
                    "au_becs_debit" => b.au_becs_debit = Some(FromValueOpt::from_value(v)?),
                    "bacs_debit" => b.bacs_debit = Some(FromValueOpt::from_value(v)?),
                    "bancontact" => b.bancontact = Some(FromValueOpt::from_value(v)?),
                    "blik" => b.blik = Some(FromValueOpt::from_value(v)?),
                    "boleto" => b.boleto = Some(FromValueOpt::from_value(v)?),
                    "card" => b.card = Some(FromValueOpt::from_value(v)?),
                    "cartes_bancaires" => b.cartes_bancaires = Some(FromValueOpt::from_value(v)?),
                    "cashapp" => b.cashapp = Some(FromValueOpt::from_value(v)?),
                    "customer_balance" => b.customer_balance = Some(FromValueOpt::from_value(v)?),
                    "eps" => b.eps = Some(FromValueOpt::from_value(v)?),
                    "fpx" => b.fpx = Some(FromValueOpt::from_value(v)?),
                    "giropay" => b.giropay = Some(FromValueOpt::from_value(v)?),
                    "google_pay" => b.google_pay = Some(FromValueOpt::from_value(v)?),
                    "grabpay" => b.grabpay = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "ideal" => b.ideal = Some(FromValueOpt::from_value(v)?),
                    "is_default" => b.is_default = Some(FromValueOpt::from_value(v)?),
                    "jcb" => b.jcb = Some(FromValueOpt::from_value(v)?),
                    "klarna" => b.klarna = Some(FromValueOpt::from_value(v)?),
                    "konbini" => b.konbini = Some(FromValueOpt::from_value(v)?),
                    "link" => b.link = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "name" => b.name = Some(FromValueOpt::from_value(v)?),
                    "oxxo" => b.oxxo = Some(FromValueOpt::from_value(v)?),
                    "p24" => b.p24 = Some(FromValueOpt::from_value(v)?),
                    "parent" => b.parent = Some(FromValueOpt::from_value(v)?),
                    "paynow" => b.paynow = Some(FromValueOpt::from_value(v)?),
                    "paypal" => b.paypal = Some(FromValueOpt::from_value(v)?),
                    "promptpay" => b.promptpay = Some(FromValueOpt::from_value(v)?),
                    "revolut_pay" => b.revolut_pay = Some(FromValueOpt::from_value(v)?),
                    "sepa_debit" => b.sepa_debit = Some(FromValueOpt::from_value(v)?),
                    "sofort" => b.sofort = Some(FromValueOpt::from_value(v)?),
                    "us_bank_account" => b.us_bank_account = Some(FromValueOpt::from_value(v)?),
                    "wechat_pay" => b.wechat_pay = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentMethodConfiguration {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentMethodConfiguration", 42)?;
        s.serialize_field("acss_debit", &self.acss_debit)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("affirm", &self.affirm)?;
        s.serialize_field("afterpay_clearpay", &self.afterpay_clearpay)?;
        s.serialize_field("alipay", &self.alipay)?;
        s.serialize_field("apple_pay", &self.apple_pay)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("au_becs_debit", &self.au_becs_debit)?;
        s.serialize_field("bacs_debit", &self.bacs_debit)?;
        s.serialize_field("bancontact", &self.bancontact)?;
        s.serialize_field("blik", &self.blik)?;
        s.serialize_field("boleto", &self.boleto)?;
        s.serialize_field("card", &self.card)?;
        s.serialize_field("cartes_bancaires", &self.cartes_bancaires)?;
        s.serialize_field("cashapp", &self.cashapp)?;
        s.serialize_field("customer_balance", &self.customer_balance)?;
        s.serialize_field("eps", &self.eps)?;
        s.serialize_field("fpx", &self.fpx)?;
        s.serialize_field("giropay", &self.giropay)?;
        s.serialize_field("google_pay", &self.google_pay)?;
        s.serialize_field("grabpay", &self.grabpay)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("ideal", &self.ideal)?;
        s.serialize_field("is_default", &self.is_default)?;
        s.serialize_field("jcb", &self.jcb)?;
        s.serialize_field("klarna", &self.klarna)?;
        s.serialize_field("konbini", &self.konbini)?;
        s.serialize_field("link", &self.link)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("oxxo", &self.oxxo)?;
        s.serialize_field("p24", &self.p24)?;
        s.serialize_field("parent", &self.parent)?;
        s.serialize_field("paynow", &self.paynow)?;
        s.serialize_field("paypal", &self.paypal)?;
        s.serialize_field("promptpay", &self.promptpay)?;
        s.serialize_field("revolut_pay", &self.revolut_pay)?;
        s.serialize_field("sepa_debit", &self.sepa_debit)?;
        s.serialize_field("sofort", &self.sofort)?;
        s.serialize_field("us_bank_account", &self.us_bank_account)?;
        s.serialize_field("wechat_pay", &self.wechat_pay)?;

        s.serialize_field("object", "payment_method_configuration")?;
        s.end()
    }
}
impl stripe_types::Object for PaymentMethodConfiguration {
    type Id = stripe_payment::PaymentMethodConfigurationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PaymentMethodConfigurationId);
