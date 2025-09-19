/// `Source` objects allow you to accept a variety of payment methods. They
/// represent a customer's payment instrument, and can be used with the Stripe API
/// just like a `Card` object: once chargeable, they can be charged, or can be
/// attached to customers.
///
/// Stripe doesn't recommend using the deprecated [Sources API](https://stripe.com/docs/api/sources).
/// We recommend that you adopt the [PaymentMethods API](https://stripe.com/docs/api/payment_methods).
/// This newer API provides access to our latest features and payment method types.
///
/// Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).
///
/// For more details see <<https://stripe.com/docs/api/sources/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Source {
    pub ach_credit_transfer: Option<stripe_shared::SourceTypeAchCreditTransfer>,
    pub ach_debit: Option<stripe_shared::SourceTypeAchDebit>,
    pub acss_debit: Option<stripe_shared::SourceTypeAcssDebit>,
    pub alipay: Option<stripe_shared::SourceTypeAlipay>,
    /// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
    /// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
    /// The field defaults to “unspecified”.
    pub allow_redisplay: Option<SourceAllowRedisplay>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source.
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub amount: Option<i64>,
    pub au_becs_debit: Option<stripe_shared::SourceTypeAuBecsDebit>,
    pub bancontact: Option<stripe_shared::SourceTypeBancontact>,
    pub card: Option<stripe_shared::SourceTypeCard>,
    pub card_present: Option<stripe_shared::SourceTypeCardPresent>,
    /// The client secret of the source. Used for client-side retrieval using a publishable key.
    pub client_secret: String,
    pub code_verification: Option<stripe_shared::SourceCodeVerificationFlow>,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    /// This is the currency for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub currency: Option<stripe_types::Currency>,
    /// The ID of the customer to which this source is attached.
    /// This will not be present when the source has not been attached to a customer.
    pub customer: Option<String>,
    pub eps: Option<stripe_shared::SourceTypeEps>,
    /// The authentication `flow` of the source.
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: String,
    pub giropay: Option<stripe_shared::SourceTypeGiropay>,
    /// Unique identifier for the object.
    pub id: stripe_shared::SourceId,
    pub ideal: Option<stripe_shared::SourceTypeIdeal>,
    pub klarna: Option<stripe_shared::SourceTypeKlarna>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub multibanco: Option<stripe_shared::SourceTypeMultibanco>,
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub owner: Option<stripe_shared::SourceOwner>,
    pub p24: Option<stripe_shared::SourceTypeP24>,
    pub receiver: Option<stripe_shared::SourceReceiverFlow>,
    pub redirect: Option<stripe_shared::SourceRedirectFlow>,
    pub sepa_credit_transfer: Option<stripe_shared::SourceTypeSepaCreditTransfer>,
    pub sepa_debit: Option<stripe_shared::SourceTypeSepaDebit>,
    pub sofort: Option<stripe_shared::SourceTypeSofort>,
    pub source_order: Option<stripe_shared::SourceOrder>,
    /// Extra information about a source.
    /// This will appear on your customer's statement every time you charge the source.
    pub statement_descriptor: Option<String>,
    /// The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`.
    /// Only `chargeable` sources can be used to create a charge.
    pub status: String,
    pub three_d_secure: Option<stripe_shared::SourceTypeThreeDSecure>,
    /// The `type` of the source.
    /// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
    /// An additional hash is included on the source with a name matching this value.
    /// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: SourceType,
    /// Either `reusable` or `single_use`.
    /// Whether this source should be reusable or not.
    /// Some source types may or may not be reusable by construction, while others may leave the option at creation.
    /// If an incompatible value is passed, an error will be returned.
    pub usage: Option<String>,
    pub wechat: Option<stripe_shared::SourceTypeWechat>,
}
#[doc(hidden)]
pub struct SourceBuilder {
    ach_credit_transfer: Option<Option<stripe_shared::SourceTypeAchCreditTransfer>>,
    ach_debit: Option<Option<stripe_shared::SourceTypeAchDebit>>,
    acss_debit: Option<Option<stripe_shared::SourceTypeAcssDebit>>,
    alipay: Option<Option<stripe_shared::SourceTypeAlipay>>,
    allow_redisplay: Option<Option<SourceAllowRedisplay>>,
    amount: Option<Option<i64>>,
    au_becs_debit: Option<Option<stripe_shared::SourceTypeAuBecsDebit>>,
    bancontact: Option<Option<stripe_shared::SourceTypeBancontact>>,
    card: Option<Option<stripe_shared::SourceTypeCard>>,
    card_present: Option<Option<stripe_shared::SourceTypeCardPresent>>,
    client_secret: Option<String>,
    code_verification: Option<Option<stripe_shared::SourceCodeVerificationFlow>>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<Option<stripe_types::Currency>>,
    customer: Option<Option<String>>,
    eps: Option<Option<stripe_shared::SourceTypeEps>>,
    flow: Option<String>,
    giropay: Option<Option<stripe_shared::SourceTypeGiropay>>,
    id: Option<stripe_shared::SourceId>,
    ideal: Option<Option<stripe_shared::SourceTypeIdeal>>,
    klarna: Option<Option<stripe_shared::SourceTypeKlarna>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    multibanco: Option<Option<stripe_shared::SourceTypeMultibanco>>,
    owner: Option<Option<stripe_shared::SourceOwner>>,
    p24: Option<Option<stripe_shared::SourceTypeP24>>,
    receiver: Option<Option<stripe_shared::SourceReceiverFlow>>,
    redirect: Option<Option<stripe_shared::SourceRedirectFlow>>,
    sepa_credit_transfer: Option<Option<stripe_shared::SourceTypeSepaCreditTransfer>>,
    sepa_debit: Option<Option<stripe_shared::SourceTypeSepaDebit>>,
    sofort: Option<Option<stripe_shared::SourceTypeSofort>>,
    source_order: Option<Option<stripe_shared::SourceOrder>>,
    statement_descriptor: Option<Option<String>>,
    status: Option<String>,
    three_d_secure: Option<Option<stripe_shared::SourceTypeThreeDSecure>>,
    type_: Option<SourceType>,
    usage: Option<Option<String>>,
    wechat: Option<Option<stripe_shared::SourceTypeWechat>>,
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

    impl Deserialize for Source {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Source>,
        builder: SourceBuilder,
    }

    impl Visitor for Place<Source> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: SourceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for SourceBuilder {
        type Out = Source;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ach_credit_transfer" => Deserialize::begin(&mut self.ach_credit_transfer),
                "ach_debit" => Deserialize::begin(&mut self.ach_debit),
                "acss_debit" => Deserialize::begin(&mut self.acss_debit),
                "alipay" => Deserialize::begin(&mut self.alipay),
                "allow_redisplay" => Deserialize::begin(&mut self.allow_redisplay),
                "amount" => Deserialize::begin(&mut self.amount),
                "au_becs_debit" => Deserialize::begin(&mut self.au_becs_debit),
                "bancontact" => Deserialize::begin(&mut self.bancontact),
                "card" => Deserialize::begin(&mut self.card),
                "card_present" => Deserialize::begin(&mut self.card_present),
                "client_secret" => Deserialize::begin(&mut self.client_secret),
                "code_verification" => Deserialize::begin(&mut self.code_verification),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "eps" => Deserialize::begin(&mut self.eps),
                "flow" => Deserialize::begin(&mut self.flow),
                "giropay" => Deserialize::begin(&mut self.giropay),
                "id" => Deserialize::begin(&mut self.id),
                "ideal" => Deserialize::begin(&mut self.ideal),
                "klarna" => Deserialize::begin(&mut self.klarna),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "multibanco" => Deserialize::begin(&mut self.multibanco),
                "owner" => Deserialize::begin(&mut self.owner),
                "p24" => Deserialize::begin(&mut self.p24),
                "receiver" => Deserialize::begin(&mut self.receiver),
                "redirect" => Deserialize::begin(&mut self.redirect),
                "sepa_credit_transfer" => Deserialize::begin(&mut self.sepa_credit_transfer),
                "sepa_debit" => Deserialize::begin(&mut self.sepa_debit),
                "sofort" => Deserialize::begin(&mut self.sofort),
                "source_order" => Deserialize::begin(&mut self.source_order),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "status" => Deserialize::begin(&mut self.status),
                "three_d_secure" => Deserialize::begin(&mut self.three_d_secure),
                "type" => Deserialize::begin(&mut self.type_),
                "usage" => Deserialize::begin(&mut self.usage),
                "wechat" => Deserialize::begin(&mut self.wechat),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                ach_credit_transfer: Deserialize::default(),
                ach_debit: Deserialize::default(),
                acss_debit: Deserialize::default(),
                alipay: Deserialize::default(),
                allow_redisplay: Deserialize::default(),
                amount: Deserialize::default(),
                au_becs_debit: Deserialize::default(),
                bancontact: Deserialize::default(),
                card: Deserialize::default(),
                card_present: Deserialize::default(),
                client_secret: Deserialize::default(),
                code_verification: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                eps: Deserialize::default(),
                flow: Deserialize::default(),
                giropay: Deserialize::default(),
                id: Deserialize::default(),
                ideal: Deserialize::default(),
                klarna: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                multibanco: Deserialize::default(),
                owner: Deserialize::default(),
                p24: Deserialize::default(),
                receiver: Deserialize::default(),
                redirect: Deserialize::default(),
                sepa_credit_transfer: Deserialize::default(),
                sepa_debit: Deserialize::default(),
                sofort: Deserialize::default(),
                source_order: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                three_d_secure: Deserialize::default(),
                type_: Deserialize::default(),
                usage: Deserialize::default(),
                wechat: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(ach_credit_transfer),
                Some(ach_debit),
                Some(acss_debit),
                Some(alipay),
                Some(allow_redisplay),
                Some(amount),
                Some(au_becs_debit),
                Some(bancontact),
                Some(card),
                Some(card_present),
                Some(client_secret),
                Some(code_verification),
                Some(created),
                Some(currency),
                Some(customer),
                Some(eps),
                Some(flow),
                Some(giropay),
                Some(id),
                Some(ideal),
                Some(klarna),
                Some(livemode),
                Some(metadata),
                Some(multibanco),
                Some(owner),
                Some(p24),
                Some(receiver),
                Some(redirect),
                Some(sepa_credit_transfer),
                Some(sepa_debit),
                Some(sofort),
                Some(source_order),
                Some(statement_descriptor),
                Some(status),
                Some(three_d_secure),
                Some(type_),
                Some(usage),
                Some(wechat),
            ) = (
                self.ach_credit_transfer.take(),
                self.ach_debit.take(),
                self.acss_debit.take(),
                self.alipay.take(),
                self.allow_redisplay,
                self.amount,
                self.au_becs_debit.take(),
                self.bancontact.take(),
                self.card.take(),
                self.card_present.take(),
                self.client_secret.take(),
                self.code_verification.take(),
                self.created,
                self.currency.take(),
                self.customer.take(),
                self.eps.take(),
                self.flow.take(),
                self.giropay.take(),
                self.id.take(),
                self.ideal.take(),
                self.klarna.take(),
                self.livemode,
                self.metadata.take(),
                self.multibanco.take(),
                self.owner.take(),
                self.p24.take(),
                self.receiver.take(),
                self.redirect.take(),
                self.sepa_credit_transfer.take(),
                self.sepa_debit.take(),
                self.sofort.take(),
                self.source_order.take(),
                self.statement_descriptor.take(),
                self.status.take(),
                self.three_d_secure.take(),
                self.type_.take(),
                self.usage.take(),
                self.wechat.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                ach_credit_transfer,
                ach_debit,
                acss_debit,
                alipay,
                allow_redisplay,
                amount,
                au_becs_debit,
                bancontact,
                card,
                card_present,
                client_secret,
                code_verification,
                created,
                currency,
                customer,
                eps,
                flow,
                giropay,
                id,
                ideal,
                klarna,
                livemode,
                metadata,
                multibanco,
                owner,
                p24,
                receiver,
                redirect,
                sepa_credit_transfer,
                sepa_debit,
                sofort,
                source_order,
                statement_descriptor,
                status,
                three_d_secure,
                type_,
                usage,
                wechat,
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

    impl ObjectDeser for Source {
        type Builder = SourceBuilder;
    }

    impl FromValueOpt for Source {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ach_credit_transfer" => b.ach_credit_transfer = FromValueOpt::from_value(v),
                    "ach_debit" => b.ach_debit = FromValueOpt::from_value(v),
                    "acss_debit" => b.acss_debit = FromValueOpt::from_value(v),
                    "alipay" => b.alipay = FromValueOpt::from_value(v),
                    "allow_redisplay" => b.allow_redisplay = FromValueOpt::from_value(v),
                    "amount" => b.amount = FromValueOpt::from_value(v),
                    "au_becs_debit" => b.au_becs_debit = FromValueOpt::from_value(v),
                    "bancontact" => b.bancontact = FromValueOpt::from_value(v),
                    "card" => b.card = FromValueOpt::from_value(v),
                    "card_present" => b.card_present = FromValueOpt::from_value(v),
                    "client_secret" => b.client_secret = FromValueOpt::from_value(v),
                    "code_verification" => b.code_verification = FromValueOpt::from_value(v),
                    "created" => b.created = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "customer" => b.customer = FromValueOpt::from_value(v),
                    "eps" => b.eps = FromValueOpt::from_value(v),
                    "flow" => b.flow = FromValueOpt::from_value(v),
                    "giropay" => b.giropay = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "ideal" => b.ideal = FromValueOpt::from_value(v),
                    "klarna" => b.klarna = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "multibanco" => b.multibanco = FromValueOpt::from_value(v),
                    "owner" => b.owner = FromValueOpt::from_value(v),
                    "p24" => b.p24 = FromValueOpt::from_value(v),
                    "receiver" => b.receiver = FromValueOpt::from_value(v),
                    "redirect" => b.redirect = FromValueOpt::from_value(v),
                    "sepa_credit_transfer" => b.sepa_credit_transfer = FromValueOpt::from_value(v),
                    "sepa_debit" => b.sepa_debit = FromValueOpt::from_value(v),
                    "sofort" => b.sofort = FromValueOpt::from_value(v),
                    "source_order" => b.source_order = FromValueOpt::from_value(v),
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "three_d_secure" => b.three_d_secure = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    "usage" => b.usage = FromValueOpt::from_value(v),
                    "wechat" => b.wechat = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Source {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Source", 39)?;
        s.serialize_field("ach_credit_transfer", &self.ach_credit_transfer)?;
        s.serialize_field("ach_debit", &self.ach_debit)?;
        s.serialize_field("acss_debit", &self.acss_debit)?;
        s.serialize_field("alipay", &self.alipay)?;
        s.serialize_field("allow_redisplay", &self.allow_redisplay)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("au_becs_debit", &self.au_becs_debit)?;
        s.serialize_field("bancontact", &self.bancontact)?;
        s.serialize_field("card", &self.card)?;
        s.serialize_field("card_present", &self.card_present)?;
        s.serialize_field("client_secret", &self.client_secret)?;
        s.serialize_field("code_verification", &self.code_verification)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("eps", &self.eps)?;
        s.serialize_field("flow", &self.flow)?;
        s.serialize_field("giropay", &self.giropay)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("ideal", &self.ideal)?;
        s.serialize_field("klarna", &self.klarna)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("multibanco", &self.multibanco)?;
        s.serialize_field("owner", &self.owner)?;
        s.serialize_field("p24", &self.p24)?;
        s.serialize_field("receiver", &self.receiver)?;
        s.serialize_field("redirect", &self.redirect)?;
        s.serialize_field("sepa_credit_transfer", &self.sepa_credit_transfer)?;
        s.serialize_field("sepa_debit", &self.sepa_debit)?;
        s.serialize_field("sofort", &self.sofort)?;
        s.serialize_field("source_order", &self.source_order)?;
        s.serialize_field("statement_descriptor", &self.statement_descriptor)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("three_d_secure", &self.three_d_secure)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("usage", &self.usage)?;
        s.serialize_field("wechat", &self.wechat)?;

        s.serialize_field("object", "source")?;
        s.end()
    }
}
/// This field indicates whether this payment method can be shown again to its customer in a checkout flow.
/// Stripe products such as Checkout and Elements use this field to determine whether a payment method can be shown as a saved payment method in a checkout flow.
/// The field defaults to “unspecified”.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SourceAllowRedisplay {
    Always,
    Limited,
    Unspecified,
}
impl SourceAllowRedisplay {
    pub fn as_str(self) -> &'static str {
        use SourceAllowRedisplay::*;
        match self {
            Always => "always",
            Limited => "limited",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for SourceAllowRedisplay {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SourceAllowRedisplay::*;
        match s {
            "always" => Ok(Always),
            "limited" => Ok(Limited),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for SourceAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SourceAllowRedisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SourceAllowRedisplay {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SourceAllowRedisplay {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SourceAllowRedisplay> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SourceAllowRedisplay::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SourceAllowRedisplay);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SourceAllowRedisplay {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SourceAllowRedisplay"))
    }
}
/// The `type` of the source.
/// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
/// An additional hash is included on the source with a name matching this value.
/// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum SourceType {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    Alipay,
    AuBecsDebit,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl SourceType {
    pub fn as_str(&self) -> &str {
        use SourceType::*;
        match self {
            AchCreditTransfer => "ach_credit_transfer",
            AchDebit => "ach_debit",
            AcssDebit => "acss_debit",
            Alipay => "alipay",
            AuBecsDebit => "au_becs_debit",
            Bancontact => "bancontact",
            Card => "card",
            CardPresent => "card_present",
            Eps => "eps",
            Giropay => "giropay",
            Ideal => "ideal",
            Klarna => "klarna",
            Multibanco => "multibanco",
            P24 => "p24",
            SepaCreditTransfer => "sepa_credit_transfer",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            ThreeDSecure => "three_d_secure",
            Wechat => "wechat",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for SourceType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SourceType::*;
        match s {
            "ach_credit_transfer" => Ok(AchCreditTransfer),
            "ach_debit" => Ok(AchDebit),
            "acss_debit" => Ok(AcssDebit),
            "alipay" => Ok(Alipay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bancontact" => Ok(Bancontact),
            "card" => Ok(Card),
            "card_present" => Ok(CardPresent),
            "eps" => Ok(Eps),
            "giropay" => Ok(Giropay),
            "ideal" => Ok(Ideal),
            "klarna" => Ok(Klarna),
            "multibanco" => Ok(Multibanco),
            "p24" => Ok(P24),
            "sepa_credit_transfer" => Ok(SepaCreditTransfer),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "three_d_secure" => Ok(ThreeDSecure),
            "wechat" => Ok(Wechat),
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for SourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for SourceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<SourceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SourceType::from_str(s).unwrap());
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(SourceType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for SourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap())
    }
}
impl stripe_types::Object for Source {
    type Id = stripe_shared::SourceId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(SourceId);
