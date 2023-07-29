/// A Checkout Session represents your customer's session as they pay for
/// one-time purchases or subscriptions through [Checkout](https://stripe.com/docs/payments/checkout)
/// or [Payment Links](https://stripe.com/docs/payments/payment-links).
///
/// We recommend creating a new Session each time your customer attempts to pay.  Once payment is successful, the Checkout Session will contain a reference to the [Customer](https://stripe.com/docs/api/customers), and either the successful [PaymentIntent](https://stripe.com/docs/api/payment_intents) or an active [Subscription](https://stripe.com/docs/api/subscriptions).  You can create a Checkout Session on your server and pass its ID to the client to begin Checkout.  Related guide: [Checkout Quickstart](https://stripe.com/docs/checkout/quickstart).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Session {
    /// When set, provides configuration for actions to take if this Checkout Session expires.
    pub after_expiration:
        Option<stripe_checkout::checkout::session::after_expiration::AfterExpiration>,
    /// Enables user redeemable promotion codes.
    pub allow_promotion_codes: Option<bool>,
    /// Total of all items before discounts or taxes are applied.
    pub amount_subtotal: Option<i64>,
    /// Total of all items after discounts and taxes are applied.
    pub amount_total: Option<i64>,
    pub automatic_tax: stripe_checkout::checkout::session::automatic_tax::AutomaticTax,
    /// Describes whether Checkout should collect the customer's billing address.
    pub billing_address_collection: Option<SessionBillingAddressCollection>,
    /// The URL the customer will be directed to if they decide to cancel payment and return to your website.
    pub cancel_url: String,
    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the Session with your internal systems.
    pub client_reference_id: Option<String>,
    /// Results of `consent_collection` for this session.
    pub consent: Option<stripe_checkout::checkout::session::consent::Consent>,
    /// When set, provides configuration for the Checkout Session to gather active consent from customers.
    pub consent_collection:
        Option<stripe_checkout::checkout::session::consent_collection::ConsentCollection>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// The ID of the customer for this Session.
    /// For Checkout Sessions in `payment` or `subscription` mode, Checkout
    /// will create a new customer object based on information provided
    /// during the payment flow unless an existing customer was provided when
    /// the Session was created.
    pub customer: Option<stripe_types::Expandable<stripe_types::customer::Customer>>,
    /// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
    pub customer_creation: Option<SessionCustomerCreation>,
    /// The customer details including the customer's tax exempt status and the customer's tax IDs.
    ///
    /// Only the customer's email is present on Sessions in `setup` mode.
    pub customer_details:
        Option<stripe_checkout::checkout::session::customer_details::CustomerDetails>,
    /// If provided, this value will be used when the Customer object is created.
    /// If not provided, customers will be asked to enter their email address.
    /// Use this parameter to prefill customer data if you already have an email
    /// on file.
    ///
    /// To access information about the customer once the payment flow is complete, use the `customer` attribute.
    pub customer_email: Option<String>,
    /// The timestamp at which the Checkout Session will expire.
    pub expires_at: stripe_types::Timestamp,
    /// Unique identifier for the object.
    ///
    /// Used to pass to `redirectToCheckout` in Stripe.js.
    pub id: stripe_checkout::checkout::session::CheckoutSessionId,
    /// The line items purchased by the customer.
    #[serde(default)]
    pub line_items: stripe_types::List<stripe_types::line_item::LineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The IETF language tag of the locale Checkout is displayed in.
    ///
    /// If blank or `auto`, the browser's locale is used.
    pub locale: Option<SessionLocale>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The mode of the Checkout Session.
    pub mode: SessionMode,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: SessionObject,
    /// The ID of the PaymentIntent for Checkout Sessions in `payment` mode.
    pub payment_intent:
        Option<stripe_types::Expandable<stripe_types::payment_intent::PaymentIntent>>,
    /// The ID of the Payment Link that created this Session.
    pub payment_link: Option<stripe_types::Expandable<stripe_payment::payment_link::PaymentLink>>,
    /// Configure whether a Checkout Session should collect a payment method.
    pub payment_method_collection: Option<SessionPaymentMethodCollection>,
    /// Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession.
    pub payment_method_options:
        Option<stripe_checkout::checkout::session::payment_method_options::PaymentMethodOptions>,
    /// A list of the types of payment methods (e.g.
    ///
    /// card) this Checkout Session is allowed to accept.
    pub payment_method_types: Vec<String>,
    /// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
    /// You can use this value to decide when to fulfill your customer's order.
    pub payment_status: SessionPaymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection:
        Option<stripe_checkout::checkout::session::phone_number_collection::PhoneNumberCollection>,
    /// The ID of the original expired Checkout Session that triggered the recovery flow.
    pub recovered_from: Option<String>,
    /// The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    pub setup_intent: Option<stripe_types::Expandable<stripe_types::setup_intent::SetupIntent>>,
    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    pub shipping_address_collection: Option<
        stripe_checkout::checkout::session::shipping_address_collection::ShippingAddressCollection,
    >,
    /// The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<stripe_checkout::checkout::session::shipping_cost::ShippingCost>,
    /// Shipping information for this Checkout Session.
    pub shipping_details: Option<stripe_types::shipping_details::ShippingDetails>,
    /// The shipping rate options applied to this Session.
    pub shipping_options: Vec<stripe_checkout::checkout::session::shipping_option::ShippingOption>,
    /// The status of the Checkout Session, one of `open`, `complete`, or `expired`.
    pub status: Option<SessionStatus>,
    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
    pub submit_type: Option<SessionSubmitType>,
    /// The ID of the subscription for Checkout Sessions in `subscription` mode.
    pub subscription: Option<stripe_types::Expandable<stripe_types::subscription::Subscription>>,
    /// The URL the customer will be directed to after the payment or
    /// subscription creation is successful.
    pub success_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection:
        Option<stripe_checkout::checkout::session::tax_id_collection::TaxIdCollection>,
    /// Tax and discount details for the computed total amount.
    pub total_details: Option<stripe_checkout::checkout::session::total_details::TotalDetails>,
    /// The URL to the Checkout Session.
    ///
    /// Redirect customers to this URL to take them to Checkout.
    /// If you’re using [Custom Domains](https://stripe.com/docs/payments/checkout/custom-domains), the URL will use your subdomain.
    /// Otherwise, it’ll use `checkout.stripe.com.` This value is only present when the session is active.
    pub url: Option<String>,
}
/// Describes whether Checkout should collect the customer's billing address.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionBillingAddressCollection {
    Auto,
    Required,
}

impl SessionBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Required => "required",
        }
    }
}

impl std::str::FromStr for SessionBillingAddressCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "required" => Ok(Self::Required),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SessionBillingAddressCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SessionBillingAddressCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionBillingAddressCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SessionBillingAddressCollection")
        })
    }
}
/// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionCustomerCreation {
    Always,
    IfRequired,
}

impl SessionCustomerCreation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for SessionCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "if_required" => Ok(Self::IfRequired),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SessionCustomerCreation {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SessionCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionCustomerCreation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SessionCustomerCreation"))
    }
}
/// The IETF language tag of the locale Checkout is displayed in.
///
/// If blank or `auto`, the browser's locale is used.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionLocale {
    Auto,
    Bg,
    Cs,
    Da,
    De,
    El,
    En,
    EnMinusGb,
    Es,
    EsMinus419,
    Et,
    Fi,
    Fil,
    Fr,
    FrMinusCa,
    Hr,
    Hu,
    Id,
    It,
    Ja,
    Ko,
    Lt,
    Lv,
    Ms,
    Mt,
    Nb,
    Nl,
    Pl,
    Pt,
    PtMinusBr,
    Ro,
    Ru,
    Sk,
    Sl,
    Sv,
    Th,
    Tr,
    Vi,
    Zh,
    ZhMinusHk,
    ZhMinusTw,
}

impl SessionLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Bg => "bg",
            Self::Cs => "cs",
            Self::Da => "da",
            Self::De => "de",
            Self::El => "el",
            Self::En => "en",
            Self::EnMinusGb => "en-GB",
            Self::Es => "es",
            Self::EsMinus419 => "es-419",
            Self::Et => "et",
            Self::Fi => "fi",
            Self::Fil => "fil",
            Self::Fr => "fr",
            Self::FrMinusCa => "fr-CA",
            Self::Hr => "hr",
            Self::Hu => "hu",
            Self::Id => "id",
            Self::It => "it",
            Self::Ja => "ja",
            Self::Ko => "ko",
            Self::Lt => "lt",
            Self::Lv => "lv",
            Self::Ms => "ms",
            Self::Mt => "mt",
            Self::Nb => "nb",
            Self::Nl => "nl",
            Self::Pl => "pl",
            Self::Pt => "pt",
            Self::PtMinusBr => "pt-BR",
            Self::Ro => "ro",
            Self::Ru => "ru",
            Self::Sk => "sk",
            Self::Sl => "sl",
            Self::Sv => "sv",
            Self::Th => "th",
            Self::Tr => "tr",
            Self::Vi => "vi",
            Self::Zh => "zh",
            Self::ZhMinusHk => "zh-HK",
            Self::ZhMinusTw => "zh-TW",
        }
    }
}

impl std::str::FromStr for SessionLocale {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "bg" => Ok(Self::Bg),
            "cs" => Ok(Self::Cs),
            "da" => Ok(Self::Da),
            "de" => Ok(Self::De),
            "el" => Ok(Self::El),
            "en" => Ok(Self::En),
            "en-GB" => Ok(Self::EnMinusGb),
            "es" => Ok(Self::Es),
            "es-419" => Ok(Self::EsMinus419),
            "et" => Ok(Self::Et),
            "fi" => Ok(Self::Fi),
            "fil" => Ok(Self::Fil),
            "fr" => Ok(Self::Fr),
            "fr-CA" => Ok(Self::FrMinusCa),
            "hr" => Ok(Self::Hr),
            "hu" => Ok(Self::Hu),
            "id" => Ok(Self::Id),
            "it" => Ok(Self::It),
            "ja" => Ok(Self::Ja),
            "ko" => Ok(Self::Ko),
            "lt" => Ok(Self::Lt),
            "lv" => Ok(Self::Lv),
            "ms" => Ok(Self::Ms),
            "mt" => Ok(Self::Mt),
            "nb" => Ok(Self::Nb),
            "nl" => Ok(Self::Nl),
            "pl" => Ok(Self::Pl),
            "pt" => Ok(Self::Pt),
            "pt-BR" => Ok(Self::PtMinusBr),
            "ro" => Ok(Self::Ro),
            "ru" => Ok(Self::Ru),
            "sk" => Ok(Self::Sk),
            "sl" => Ok(Self::Sl),
            "sv" => Ok(Self::Sv),
            "th" => Ok(Self::Th),
            "tr" => Ok(Self::Tr),
            "vi" => Ok(Self::Vi),
            "zh" => Ok(Self::Zh),
            "zh-HK" => Ok(Self::ZhMinusHk),
            "zh-TW" => Ok(Self::ZhMinusTw),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SessionLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SessionLocale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionLocale {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SessionLocale"))
    }
}
/// The mode of the Checkout Session.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionMode {
    Payment,
    Setup,
    Subscription,
}

impl SessionMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Payment => "payment",
            Self::Setup => "setup",
            Self::Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for SessionMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "payment" => Ok(Self::Payment),
            "setup" => Ok(Self::Setup),
            "subscription" => Ok(Self::Subscription),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SessionMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SessionMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SessionMode"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionObject {
    CheckoutSession,
}

impl SessionObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CheckoutSession => "checkout.session",
        }
    }
}

impl std::str::FromStr for SessionObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "checkout.session" => Ok(Self::CheckoutSession),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SessionObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SessionObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SessionObject"))
    }
}
/// Configure whether a Checkout Session should collect a payment method.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionPaymentMethodCollection {
    Always,
    IfRequired,
}

impl SessionPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for SessionPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "if_required" => Ok(Self::IfRequired),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SessionPaymentMethodCollection {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SessionPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SessionPaymentMethodCollection")
        })
    }
}
/// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
/// You can use this value to decide when to fulfill your customer's order.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionPaymentStatus {
    NoPaymentRequired,
    Paid,
    Unpaid,
}

impl SessionPaymentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoPaymentRequired => "no_payment_required",
            Self::Paid => "paid",
            Self::Unpaid => "unpaid",
        }
    }
}

impl std::str::FromStr for SessionPaymentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no_payment_required" => Ok(Self::NoPaymentRequired),
            "paid" => Ok(Self::Paid),
            "unpaid" => Ok(Self::Unpaid),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SessionPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SessionPaymentStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionPaymentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SessionPaymentStatus"))
    }
}
/// The status of the Checkout Session, one of `open`, `complete`, or `expired`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionStatus {
    Complete,
    Expired,
    Open,
}

impl SessionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Expired => "expired",
            Self::Open => "open",
        }
    }
}

impl std::str::FromStr for SessionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "complete" => Ok(Self::Complete),
            "expired" => Ok(Self::Expired),
            "open" => Ok(Self::Open),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SessionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SessionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for SessionStatus"))
    }
}
/// Describes the type of transaction being performed by Checkout in order to customize
/// relevant text on the page, such as the submit button.
///
/// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SessionSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

impl SessionSubmitType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Book => "book",
            Self::Donate => "donate",
            Self::Pay => "pay",
        }
    }
}

impl std::str::FromStr for SessionSubmitType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "book" => Ok(Self::Book),
            "donate" => Ok(Self::Donate),
            "pay" => Ok(Self::Pay),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for SessionSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SessionSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for SessionSubmitType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionSubmitType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SessionSubmitType"))
    }
}
impl stripe_types::Object for Session {
    type Id = stripe_checkout::checkout::session::CheckoutSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(CheckoutSessionId, "cs_");
pub mod payment_method_options;
pub use payment_method_options::PaymentMethodOptions;
pub mod after_expiration;
pub use after_expiration::AfterExpiration;
pub mod automatic_tax;
pub use automatic_tax::AutomaticTax;
pub mod consent;
pub use consent::Consent;
pub mod consent_collection;
pub use consent_collection::ConsentCollection;
pub mod customer_details;
pub use customer_details::CustomerDetails;
pub mod phone_number_collection;
pub use phone_number_collection::PhoneNumberCollection;
pub mod shipping_address_collection;
pub use shipping_address_collection::ShippingAddressCollection;
pub mod shipping_cost;
pub use shipping_cost::ShippingCost;
pub mod shipping_option;
pub use shipping_option::ShippingOption;
pub mod tax_id_collection;
pub use tax_id_collection::TaxIdCollection;
pub mod total_details;
pub use total_details::TotalDetails;
