/// A payment link is a shareable URL that will take your customers to a hosted payment page.
///
/// A payment link can be shared and used multiple times.  When a customer opens a payment link it will open a new [checkout session](https://stripe.com/docs/api/checkout/sessions) to render the payment page.
/// You can use [checkout session events](https://stripe.com/docs/api/events/types#event_types-checkout.session.completed) to track payments through payment links.  Related guide: [Payment Links API](https://stripe.com/docs/payment-links).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PaymentLink {
    /// Whether the payment link's `url` is active.
    ///
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub active: bool,
    pub after_completion: stripe_types::PaymentLinksResourceAfterCompletion,
    /// Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,
    /// The ID of the Connect application that created the Payment Link.
    pub application: Option<stripe_types::Expandable<stripe_types::Application>>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_types::PaymentLinksResourceAutomaticTax,
    /// Configuration for collecting the customer's billing address.
    pub billing_address_collection: PaymentLinkBillingAddressCollection,
    /// When set, provides configuration to gather active consent from customers.
    pub consent_collection: Option<stripe_types::PaymentLinksResourceConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Collect additional information from your customer using custom fields.
    ///
    /// Up to 2 fields are supported.
    pub custom_fields: Vec<stripe_types::PaymentLinksResourceCustomFields>,
    pub custom_text: stripe_types::PaymentLinksResourceCustomText,
    /// Configuration for Customer creation during checkout.
    pub customer_creation: PaymentLinkCustomerCreation,
    /// Unique identifier for the object.
    pub id: stripe_types::payment_link::PaymentLinkId,
    /// Configuration for creating invoice for payment mode payment links.
    pub invoice_creation: Option<stripe_types::PaymentLinksResourceInvoiceCreation>,
    /// The line items representing what is being sold.
    #[serde(default)]
    pub line_items: stripe_types::List<stripe_types::LineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The account on behalf of which to charge.
    ///
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_types::Account>>,
    /// Indicates the parameters to be passed to PaymentIntent creation during checkout.
    pub payment_intent_data: Option<stripe_types::PaymentLinksResourcePaymentIntentData>,
    /// Configuration for collecting a payment method during checkout.
    pub payment_method_collection: PaymentLinkPaymentMethodCollection,
    /// The list of payment method types that customers can use.
    ///
    /// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<PaymentLinkPaymentMethodTypes>>,
    pub phone_number_collection: stripe_types::PaymentLinksResourcePhoneNumberCollection,
    /// Configuration for collecting the customer's shipping address.
    pub shipping_address_collection:
        Option<stripe_types::PaymentLinksResourceShippingAddressCollection>,
    /// The shipping rate options applied to the session.
    pub shipping_options: Vec<stripe_types::PaymentLinksResourceShippingOption>,
    /// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
    pub submit_type: PaymentLinkSubmitType,
    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub subscription_data: Option<stripe_types::PaymentLinksResourceSubscriptionData>,
    pub tax_id_collection: stripe_types::PaymentLinksResourceTaxIdCollection,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub transfer_data: Option<stripe_types::PaymentLinksResourceTransferData>,
    /// The public URL that can be shared with customers.
    pub url: String,
}
/// Configuration for collecting the customer's billing address.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinkBillingAddressCollection {
    Auto,
    Required,
}

impl PaymentLinkBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkBillingAddressCollection::*;
        match self {
            Auto => "auto",
            Required => "required",
        }
    }
}

impl std::str::FromStr for PaymentLinkBillingAddressCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkBillingAddressCollection::*;
        match s {
            "auto" => Ok(Auto),
            "required" => Ok(Required),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentLinkBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkBillingAddressCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkBillingAddressCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinkBillingAddressCollection")
        })
    }
}
/// Configuration for Customer creation during checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinkCustomerCreation {
    Always,
    IfRequired,
}

impl PaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for PaymentLinkCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentLinkCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkCustomerCreation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkCustomerCreation"))
    }
}
/// Configuration for collecting a payment method during checkout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}

impl PaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for PaymentLinkPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentLinkPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinkPaymentMethodCollection")
        })
    }
}
/// The list of payment method types that customers can use.
///
/// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinkPaymentMethodTypes {
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    Cashapp,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
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
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl PaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkPaymentMethodTypes::*;
        match self {
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Blik => "blik",
            Boleto => "boleto",
            Card => "card",
            Cashapp => "cashapp",
            Eps => "eps",
            Fpx => "fpx",
            Giropay => "giropay",
            Grabpay => "grabpay",
            Ideal => "ideal",
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
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for PaymentLinkPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkPaymentMethodTypes::*;
        match s {
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "blik" => Ok(Blik),
            "boleto" => Ok(Boleto),
            "card" => Ok(Card),
            "cashapp" => Ok(Cashapp),
            "eps" => Ok(Eps),
            "fpx" => Ok(Fpx),
            "giropay" => Ok(Giropay),
            "grabpay" => Ok(Grabpay),
            "ideal" => Ok(Ideal),
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
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkPaymentMethodTypes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(PaymentLinkPaymentMethodTypes::Unknown))
    }
}
/// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PaymentLinkSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

impl PaymentLinkSubmitType {
    pub fn as_str(self) -> &'static str {
        use PaymentLinkSubmitType::*;
        match self {
            Auto => "auto",
            Book => "book",
            Donate => "donate",
            Pay => "pay",
        }
    }
}

impl std::str::FromStr for PaymentLinkSubmitType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkSubmitType::*;
        match s {
            "auto" => Ok(Auto),
            "book" => Ok(Book),
            "donate" => Ok(Donate),
            "pay" => Ok(Pay),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentLinkSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for PaymentLinkSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for PaymentLinkSubmitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkSubmitType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkSubmitType"))
    }
}
impl stripe_types::Object for PaymentLink {
    type Id = stripe_types::payment_link::PaymentLinkId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(PaymentLinkId, "plink_");
