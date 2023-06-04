/// `Source` objects allow you to accept a variety of payment methods.
///
/// They represent a customer's payment instrument, and can be used with the Stripe API just like a `Card` object: once chargeable, they can be charged, or can be attached to customers.  Stripe doesn't recommend using the deprecated [Sources API](https://stripe.com/docs/api/sources). We recommend that you adopt the [PaymentMethods API](https://stripe.com/docs/api/payment_methods). This newer API provides access to our latest features and payment method types.  Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<crate::source::ach_credit_transfer::AchCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<crate::source::ach_debit::AchDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::source::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<crate::source::alipay::Alipay>,
    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<crate::source::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::source::bancontact::Bancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::source::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<crate::source::card_present::CardPresent>,
    /// The client secret of the source.
    ///
    /// Used for client-side retrieval using a publishable key.
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verification: Option<crate::source::code_verification_flow::CodeVerificationFlow>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    ///
    /// This is the currency for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub currency: Option<crate::Currency>,
    /// The ID of the customer to which this source is attached.
    ///
    /// This will not be present when the source has not been attached to a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<crate::source::eps::Eps>,
    /// The authentication `flow` of the source.
    ///
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<crate::source::giropay::Giropay>,
    /// Unique identifier for the object.
    pub id: crate::source::SourceId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::source::ideal::Ideal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::source::klarna::Klarna>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<crate::source::multibanco::Multibanco>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SourceObject,
    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub owner: Option<crate::source::owner::Owner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<crate::source::p24::P24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<crate::source::receiver_flow::ReceiverFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<crate::source::redirect_flow::RedirectFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<crate::source::sepa_credit_transfer::SepaCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::source::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::source::sofort::Sofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<crate::source::order::Order>,
    /// Extra information about a source.
    ///
    /// This will appear on your customer's statement every time you charge the source.
    pub statement_descriptor: Option<String>,
    /// The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`.
    ///
    /// Only `chargeable` sources can be used to create a charge.
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<crate::source::three_d_secure::ThreeDSecure>,
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
    pub wechat: Option<crate::source::wechat::Wechat>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// The `type` of the source.
///
/// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
/// An additional hash is included on the source with a name matching this value.
/// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for Source {
    type Id = crate::source::SourceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(SourceId, "src_");
pub mod code_verification_flow;
pub mod requests;
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
