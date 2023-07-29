/// `Source` objects allow you to accept a variety of payment methods.
///
/// They represent a customer's payment instrument, and can be used with the Stripe API just like a `Card` object: once chargeable, they can be charged, or can be attached to customers.  Stripe doesn't recommend using the deprecated [Sources API](https://stripe.com/docs/api/sources). We recommend that you adopt the [PaymentMethods API](https://stripe.com/docs/api/payment_methods). This newer API provides access to our latest features and payment method types.  Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<stripe_types::source::ach_credit_transfer::AchCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<stripe_types::source::ach_debit::AchDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types::source::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types::source::alipay::Alipay>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types::source::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types::source::bancontact::Bancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::source::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types::source::card_present::CardPresent>,
    /// The client secret of the source.
    ///
    /// Used for client-side retrieval using a publishable key.
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verification:
        Option<stripe_types::source::code_verification_flow::CodeVerificationFlow>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    ///
    /// This is the currency for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub currency: Option<stripe_types::Currency>,
    /// The ID of the customer to which this source is attached.
    ///
    /// This will not be present when the source has not been attached to a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types::source::eps::Eps>,
    /// The authentication `flow` of the source.
    ///
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types::source::giropay::Giropay>,
    /// Unique identifier for the object.
    pub id: stripe_types::source::SourceId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types::source::ideal::Ideal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types::source::klarna::Klarna>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<stripe_types::source::multibanco::Multibanco>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SourceObject,
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub owner: Option<stripe_types::source::owner::Owner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types::source::p24::P24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<stripe_types::source::receiver_flow::ReceiverFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<stripe_types::source::redirect_flow::RedirectFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer:
        Option<stripe_types::source::sepa_credit_transfer::SepaCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types::source::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types::source::sofort::Sofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<stripe_types::source::order::Order>,
    /// Extra information about a source.
    ///
    /// This will appear on your customer's statement every time you charge the source.
    pub statement_descriptor: Option<String>,
    /// The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`.
    ///
    /// Only `chargeable` sources can be used to create a charge.
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<stripe_types::source::three_d_secure::ThreeDSecure>,
    /// The `type` of the source.
    ///
    /// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
    /// An additional hash is included on the source with a name matching this value.
    /// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
    #[serde(rename = "type")]
    pub type_: SourceType,
    /// Either `reusable` or `single_use`.
    ///
    /// Whether this source should be reusable or not.
    /// Some source types may or may not be reusable by construction, while others may leave the option at creation.
    /// If an incompatible value is passed, an error will be returned.
    pub usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<stripe_types::source::wechat::Wechat>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Source {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SourceObject {
    Source,
}

impl SourceObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
        }
    }
}

impl std::str::FromStr for SourceObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "source" => Ok(Self::Source),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SourceObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SourceObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SourceObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SourceObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SourceObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SourceObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SourceObject::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
/// The `type` of the source.
///
/// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
/// An additional hash is included on the source with a name matching this value.
/// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
}

impl SourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AchCreditTransfer => "ach_credit_transfer",
            Self::AchDebit => "ach_debit",
            Self::AcssDebit => "acss_debit",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::Eps => "eps",
            Self::Giropay => "giropay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Multibanco => "multibanco",
            Self::P24 => "p24",
            Self::SepaCreditTransfer => "sepa_credit_transfer",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::ThreeDSecure => "three_d_secure",
            Self::Wechat => "wechat",
        }
    }
}

impl std::str::FromStr for SourceType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ach_credit_transfer" => Ok(Self::AchCreditTransfer),
            "ach_debit" => Ok(Self::AchDebit),
            "acss_debit" => Ok(Self::AcssDebit),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "card" => Ok(Self::Card),
            "card_present" => Ok(Self::CardPresent),
            "eps" => Ok(Self::Eps),
            "giropay" => Ok(Self::Giropay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "multibanco" => Ok(Self::Multibanco),
            "p24" => Ok(Self::P24),
            "sepa_credit_transfer" => Ok(Self::SepaCreditTransfer),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "three_d_secure" => Ok(Self::ThreeDSecure),
            "wechat" => Ok(Self::Wechat),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SourceType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SourceType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SourceType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::de::Visitor for crate::Place<SourceType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(SourceType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}
impl stripe_types::Object for Source {
    type Id = stripe_types::source::SourceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(SourceId, "src_");
pub mod code_verification_flow;
pub use code_verification_flow::CodeVerificationFlow;
pub mod order;
pub use order::Order;
pub mod order_item;
pub use order_item::OrderItem;
pub mod owner;
pub use owner::Owner;
pub mod receiver_flow;
pub use receiver_flow::ReceiverFlow;
pub mod redirect_flow;
pub use redirect_flow::RedirectFlow;
pub mod ach_credit_transfer;
pub use ach_credit_transfer::AchCreditTransfer;
pub mod ach_debit;
pub use ach_debit::AchDebit;
pub mod acss_debit;
pub use acss_debit::AcssDebit;
pub mod alipay;
pub use alipay::Alipay;
pub mod au_becs_debit;
pub use au_becs_debit::AuBecsDebit;
pub mod bancontact;
pub use bancontact::Bancontact;
pub mod card;
pub use card::Card;
pub mod card_present;
pub use card_present::CardPresent;
pub mod eps;
pub use eps::Eps;
pub mod giropay;
pub use giropay::Giropay;
pub mod ideal;
pub use ideal::Ideal;
pub mod klarna;
pub use klarna::Klarna;
pub mod multibanco;
pub use multibanco::Multibanco;
pub mod p24;
pub use p24::P24;
pub mod sepa_credit_transfer;
pub use sepa_credit_transfer::SepaCreditTransfer;
pub mod sepa_debit;
pub use sepa_debit::SepaDebit;
pub mod sofort;
pub use sofort::Sofort;
pub mod three_d_secure;
pub use three_d_secure::ThreeDSecure;
pub mod wechat;
pub use wechat::Wechat;
