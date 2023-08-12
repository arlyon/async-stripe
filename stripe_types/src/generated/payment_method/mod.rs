/// PaymentMethod objects represent your customer's payment instruments.
/// You can use them with [PaymentIntents](https://stripe.com/docs/payments/payment-intents) to collect payments or save them to
/// Customer objects to store instrument details for future payments.
///
/// Related guides: [Payment Methods](https://stripe.com/docs/payments/payment-methods) and [More Payment Scenarios](https://stripe.com/docs/payments/more-payment-scenarios).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<stripe_types::PaymentMethodAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<stripe_types::PaymentMethodAffirm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<stripe_types::PaymentMethodAfterpayClearpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<stripe_types::PaymentFlowsPrivatePaymentMethodsAlipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<stripe_types::PaymentMethodAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<stripe_types::PaymentMethodBacsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<stripe_types::PaymentMethodBancontact>,
    pub billing_details: stripe_types::BillingDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<stripe_types::PaymentMethodBlik>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<stripe_types::PaymentMethodBoleto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<stripe_types::PaymentMethodCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<stripe_types::PaymentMethodCardPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp: Option<stripe_types::PaymentMethodCashapp>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<stripe_types::Expandable<stripe_types::Customer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<stripe_types::PaymentMethodCustomerBalance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<stripe_types::PaymentMethodEps>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<stripe_types::PaymentMethodFpx>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<stripe_types::PaymentMethodGiropay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<stripe_types::PaymentMethodGrabpay>,
    /// Unique identifier for the object.
    pub id: stripe_types::payment_method::PaymentMethodId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<stripe_types::PaymentMethodIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<stripe_types::PaymentMethodInteracPresent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<stripe_types::PaymentMethodKlarna>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<stripe_types::PaymentMethodKonbini>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<stripe_types::PaymentMethodLink>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<stripe_types::PaymentMethodOxxo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<stripe_types::PaymentMethodP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<stripe_types::PaymentMethodPaynow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<stripe_types::PaymentMethodPaypal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<stripe_types::PaymentMethodPix>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<stripe_types::PaymentMethodPromptpay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<stripe_types::RadarRadarOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<stripe_types::PaymentMethodSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<stripe_types::PaymentMethodSofort>,
    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<stripe_types::PaymentMethodUsBankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<stripe_types::PaymentMethodWechatPay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<stripe_types::PaymentMethodZip>,
}
/// The type of the PaymentMethod.
///
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
        Ok(Self::from_str(&s).unwrap_or(PaymentMethodType::Unknown))
    }
}
impl stripe_types::Object for PaymentMethod {
    type Id = stripe_types::payment_method::PaymentMethodId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(PaymentMethodId, "pm_" | "card_" | "src_" | "ba_");
