/// PaymentMethod objects represent your customer's payment instruments.
/// You can use them with [PaymentIntents](https://stripe.com/docs/payments/payment-intents) to collect payments or save them to
/// Customer objects to store instrument details for future payments.
///
/// Related guides: [Payment Methods](https://stripe.com/docs/payments/payment-methods) and [More Payment Scenarios](https://stripe.com/docs/payments/more-payment-scenarios).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types::acss_debit::AcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types::affirm::Affirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types::afterpay_clearpay::AfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types::alipay::Alipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types::au_becs_debit::AuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types::bacs_debit::BacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types::bancontact::Bancontact>,
    pub billing_details: stripe_types::billing_details::BillingDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types::blik::Blik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types::boleto::Boleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::card::Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types::card_present::CardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types::cashapp::Cashapp>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<stripe_types::Expandable<stripe_types::customer::Customer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types::customer_balance::CustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types::eps::Eps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types::fpx::Fpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types::giropay::Giropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types::grabpay::Grabpay>,
    /// Unique identifier for the object.
    pub id: stripe_types::payment_method::PaymentMethodId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types::ideal::Ideal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_types::interac_present::InteracPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types::klarna::Klarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types::konbini::Konbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::link::Link>,
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
    pub oxxo: Option<stripe_types::oxxo::Oxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types::p24::P24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types::paynow::Paynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types::paypal::Paypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types::pix::Pix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types::promptpay::Promptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<stripe_types::radar_options::RadarOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types::sepa_debit::SepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types::sofort::Sofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types::us_bank_account::UsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types::wechat_pay::WechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types::zip::Zip>,
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
        use PaymentMethodObject::*;
        match self {
            PaymentMethod => "payment_method",
        }
    }
}

impl std::str::FromStr for PaymentMethodObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentMethodObject::*;
        match s {
            "payment_method" => Ok(PaymentMethod),
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
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
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
    Zip,
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
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
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
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
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
