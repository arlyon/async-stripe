/// PaymentMethod objects represent your customer's payment instruments.
/// You can use them with [PaymentIntents](https://stripe.com/docs/payments/payment-intents) to collect payments or save them to
/// Customer objects to store instrument details for future payments.
///
/// Related guides: [Payment Methods](https://stripe.com/docs/payments/payment-methods) and [More Payment Scenarios](https://stripe.com/docs/payments/more-payment-scenarios).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types::payment_method::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types::payment_method::affirm::Affirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay:
        Option<stripe_types::payment_method::afterpay_clearpay::AfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types::payment_method::alipay::Alipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types::payment_method::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types::payment_method::bacs_debit::BacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types::payment_method::bancontact::Bancontact>,
    pub billing_details: stripe_types::payment_method::billing_details::BillingDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types::payment_method::blik::Blik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types::payment_method::boleto::Boleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::payment_method::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types::payment_method::card_present::CardPresent>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<stripe_types::Expandable<stripe_types::customer::Customer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types::payment_method::customer_balance::CustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types::payment_method::eps::Eps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types::payment_method::fpx::Fpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types::payment_method::giropay::Giropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types::payment_method::grabpay::Grabpay>,
    /// Unique identifier for the object.
    pub id: stripe_types::payment_method::PaymentMethodId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types::payment_method::ideal::Ideal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_types::payment_method::interac_present::InteracPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types::payment_method::klarna::Klarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types::payment_method::konbini::Konbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::payment_method::link::Link>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: PaymentMethodObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types::payment_method::oxxo::Oxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types::payment_method::p24::P24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types::payment_method::paynow::Paynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types::payment_method::pix::Pix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types::payment_method::promptpay::Promptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<stripe_types::charge::radar_options::RadarOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types::payment_method::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types::payment_method::sofort::Sofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types::payment_method::us_bank_account::UsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types::payment_method::wechat_pay::WechatPay>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentMethodObject {
    PaymentMethod,
}

impl PaymentMethodObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PaymentMethod => "payment_method",
        }
    }
}

impl std::str::FromStr for PaymentMethodObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "payment_method" => Ok(Self::PaymentMethod),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PaymentMethodObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentMethodObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodObject"))
    }
}
/// The type of the PaymentMethod.
///
/// An additional hash is included on the PaymentMethod with a name matching this value.
/// It contains additional information specific to the PaymentMethod type.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl PaymentMethodType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::InteracPresent => "interac_present",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl std::str::FromStr for PaymentMethodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "acss_debit" => Ok(Self::AcssDebit),
            "affirm" => Ok(Self::Affirm),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "blik" => Ok(Self::Blik),
            "boleto" => Ok(Self::Boleto),
            "card" => Ok(Self::Card),
            "card_present" => Ok(Self::CardPresent),
            "customer_balance" => Ok(Self::CustomerBalance),
            "eps" => Ok(Self::Eps),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "interac_present" => Ok(Self::InteracPresent),
            "klarna" => Ok(Self::Klarna),
            "konbini" => Ok(Self::Konbini),
            "link" => Ok(Self::Link),
            "oxxo" => Ok(Self::Oxxo),
            "p24" => Ok(Self::P24),
            "paynow" => Ok(Self::Paynow),
            "pix" => Ok(Self::Pix),
            "promptpay" => Ok(Self::Promptpay),
            "sepa_debit" => Ok(Self::SepaDebit),
            "sofort" => Ok(Self::Sofort),
            "us_bank_account" => Ok(Self::UsBankAccount),
            "wechat_pay" => Ok(Self::WechatPay),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentMethodType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentMethodType"))
    }
}
impl stripe_types::Object for PaymentMethod {
    type Id = stripe_types::payment_method::PaymentMethodId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PaymentMethodId, "pm_" | "card_" | "src_" | "ba_");
pub mod billing_details;
pub use billing_details::BillingDetails;
pub mod alipay;
pub use alipay::Alipay;
pub mod acss_debit;
pub use acss_debit::AcssDebit;
pub mod affirm;
pub use affirm::Affirm;
pub mod afterpay_clearpay;
pub use afterpay_clearpay::AfterpayClearpay;
pub mod au_becs_debit;
pub use au_becs_debit::AuBecsDebit;
pub mod bacs_debit;
pub use bacs_debit::BacsDebit;
pub mod bancontact;
pub use bancontact::Bancontact;
pub mod blik;
pub use blik::Blik;
pub mod boleto;
pub use boleto::Boleto;
pub mod card;
pub use card::Card;
pub mod card_present;
pub use card_present::CardPresent;
pub mod customer_balance;
pub use customer_balance::CustomerBalance;
pub mod eps;
pub use eps::Eps;
pub mod fpx;
pub use fpx::Fpx;
pub mod giropay;
pub use giropay::Giropay;
pub mod grabpay;
pub use grabpay::Grabpay;
pub mod ideal;
pub use ideal::Ideal;
pub mod interac_present;
pub use interac_present::InteracPresent;
pub mod klarna;
pub use klarna::Klarna;
pub mod konbini;
pub use konbini::Konbini;
pub mod link;
pub use link::Link;
pub mod oxxo;
pub use oxxo::Oxxo;
pub mod p24;
pub use p24::P24;
pub mod paynow;
pub use paynow::Paynow;
pub mod pix;
pub use pix::Pix;
pub mod promptpay;
pub use promptpay::Promptpay;
pub mod sepa_debit;
pub use sepa_debit::SepaDebit;
pub mod sofort;
pub use sofort::Sofort;
pub mod us_bank_account;
pub use us_bank_account::UsBankAccount;
pub mod wechat_pay;
pub use wechat_pay::WechatPay;
