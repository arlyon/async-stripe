/// A Checkout Session represents your customer's session as they pay for
/// one-time purchases or subscriptions through [Checkout](https://stripe.com/docs/payments/checkout)
/// or [Payment Links](https://stripe.com/docs/payments/payment-links).
///
/// We recommend creating a new Session each time your customer attempts to pay.  Once payment is successful, the Checkout Session will contain a reference to the [Customer](https://stripe.com/docs/api/customers), and either the successful [PaymentIntent](https://stripe.com/docs/api/payment_intents) or an active [Subscription](https://stripe.com/docs/api/subscriptions).  You can create a Checkout Session on your server and redirect to its URL to begin Checkout.  Related guide: [Checkout quickstart](https://stripe.com/docs/checkout/quickstart).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Session {
    /// When set, provides configuration for actions to take if this Checkout Session expires.
    pub after_expiration: Option<stripe_checkout::PaymentPagesCheckoutSessionAfterExpiration>,
    /// Enables user redeemable promotion codes.
    pub allow_promotion_codes: Option<bool>,
    /// Total of all items before discounts or taxes are applied.
    pub amount_subtotal: Option<i64>,
    /// Total of all items after discounts and taxes are applied.
    pub amount_total: Option<i64>,
    pub automatic_tax: stripe_checkout::PaymentPagesCheckoutSessionAutomaticTax,
    /// Describes whether Checkout should collect the customer's billing address.
    pub billing_address_collection: Option<SessionBillingAddressCollection>,
    /// If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment and return to your website.
    pub cancel_url: Option<String>,
    /// A unique string to reference the Checkout Session.
    ///
    /// This can be a customer ID, a cart ID, or similar, and can be used to reconcile the Session with your internal systems.
    pub client_reference_id: Option<String>,
    /// Results of `consent_collection` for this session.
    pub consent: Option<stripe_checkout::PaymentPagesCheckoutSessionConsent>,
    /// When set, provides configuration for the Checkout Session to gather active consent from customers.
    pub consent_collection: Option<stripe_checkout::PaymentPagesCheckoutSessionConsentCollection>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<stripe_types::Currency>,
    /// Currency conversion details for automatic currency conversion sessions.
    pub currency_conversion: Option<stripe_checkout::PaymentPagesCheckoutSessionCurrencyConversion>,
    /// Collect additional information from your customer using custom fields.
    ///
    /// Up to 2 fields are supported.
    pub custom_fields: Vec<stripe_checkout::PaymentPagesCheckoutSessionCustomFields>,
    pub custom_text: stripe_checkout::PaymentPagesCheckoutSessionCustomText,
    /// The ID of the customer for this Session.
    /// For Checkout Sessions in `payment` or `subscription` mode, Checkout
    /// will create a new customer object based on information provided
    /// during the payment flow unless an existing customer was provided when
    /// the Session was created.
    pub customer: Option<stripe_types::Expandable<stripe_types::Customer>>,
    /// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
    pub customer_creation: Option<SessionCustomerCreation>,
    /// The customer details including the customer's tax exempt status and the customer's tax IDs.
    ///
    /// Only the customer's email is present on Sessions in `setup` mode.
    pub customer_details: Option<stripe_checkout::PaymentPagesCheckoutSessionCustomerDetails>,
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
    pub id: stripe_checkout::session::CheckoutSessionId,
    /// ID of the invoice created by the Checkout Session, if it exists.
    pub invoice: Option<stripe_types::Expandable<stripe_types::Invoice>>,
    /// Details on the state of invoice creation for the Checkout Session.
    pub invoice_creation: Option<stripe_checkout::PaymentPagesCheckoutSessionInvoiceCreation>,
    /// The line items purchased by the customer.
    #[serde(default)]
    pub line_items: stripe_types::List<stripe_types::LineItem>,
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
    /// The ID of the PaymentIntent for Checkout Sessions in `payment` mode.
    pub payment_intent: Option<stripe_types::Expandable<stripe_types::PaymentIntent>>,
    /// The ID of the Payment Link that created this Session.
    pub payment_link: Option<stripe_types::Expandable<stripe_types::PaymentLink>>,
    /// Configure whether a Checkout Session should collect a payment method.
    pub payment_method_collection: Option<SessionPaymentMethodCollection>,
    /// Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession.
    pub payment_method_options: Option<stripe_checkout::CheckoutSessionPaymentMethodOptions>,
    /// A list of the types of payment methods (e.g.
    ///
    /// card) this Checkout Session is allowed to accept.
    pub payment_method_types: Vec<String>,
    /// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
    /// You can use this value to decide when to fulfill your customer's order.
    pub payment_status: SessionPaymentStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection:
        Option<stripe_checkout::PaymentPagesCheckoutSessionPhoneNumberCollection>,
    /// The ID of the original expired Checkout Session that triggered the recovery flow.
    pub recovered_from: Option<String>,
    /// The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    pub setup_intent: Option<stripe_types::Expandable<stripe_types::SetupIntent>>,
    /// When set, provides configuration for Checkout to collect a shipping address from a customer.
    pub shipping_address_collection:
        Option<stripe_checkout::PaymentPagesCheckoutSessionShippingAddressCollection>,
    /// The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<stripe_checkout::PaymentPagesCheckoutSessionShippingCost>,
    /// Shipping information for this Checkout Session.
    pub shipping_details: Option<stripe_types::Shipping>,
    /// The shipping rate options applied to this Session.
    pub shipping_options: Vec<stripe_checkout::PaymentPagesCheckoutSessionShippingOption>,
    /// The status of the Checkout Session, one of `open`, `complete`, or `expired`.
    pub status: Option<SessionStatus>,
    /// Describes the type of transaction being performed by Checkout in order to customize
    /// relevant text on the page, such as the submit button.
    ///
    /// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
    pub submit_type: Option<SessionSubmitType>,
    /// The ID of the subscription for Checkout Sessions in `subscription` mode.
    pub subscription: Option<stripe_types::Expandable<stripe_types::Subscription>>,
    /// The URL the customer will be directed to after the payment or
    /// subscription creation is successful.
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<stripe_checkout::PaymentPagesCheckoutSessionTaxIdCollection>,
    /// Tax and discount details for the computed total amount.
    pub total_details: Option<stripe_checkout::PaymentPagesCheckoutSessionTotalDetails>,
    /// The URL to the Checkout Session.
    ///
    /// Redirect customers to this URL to take them to Checkout.
    /// If you’re using [Custom Domains](https://stripe.com/docs/payments/checkout/custom-domains), the URL will use your subdomain.
    /// Otherwise, it’ll use `checkout.stripe.com.` This value is only present when the session is active.
    pub url: Option<String>,
}
/// Describes whether Checkout should collect the customer's billing address.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SessionBillingAddressCollection {
    Auto,
    Required,
}

impl SessionBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        use SessionBillingAddressCollection::*;
        match self {
            Auto => "auto",
            Required => "required",
        }
    }
}

impl std::str::FromStr for SessionBillingAddressCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SessionBillingAddressCollection::*;
        match s {
            "auto" => Ok(Auto),
            "required" => Ok(Required),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SessionBillingAddressCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SessionBillingAddressCollection")
        })
    }
}
/// Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SessionCustomerCreation {
    Always,
    IfRequired,
}

impl SessionCustomerCreation {
    pub fn as_str(self) -> &'static str {
        use SessionCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for SessionCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SessionCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SessionCustomerCreation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SessionCustomerCreation"))
    }
}
/// The IETF language tag of the locale Checkout is displayed in.
///
/// If blank or `auto`, the browser's locale is used.
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use SessionLocale::*;
        match self {
            Auto => "auto",
            Bg => "bg",
            Cs => "cs",
            Da => "da",
            De => "de",
            El => "el",
            En => "en",
            EnMinusGb => "en-GB",
            Es => "es",
            EsMinus419 => "es-419",
            Et => "et",
            Fi => "fi",
            Fil => "fil",
            Fr => "fr",
            FrMinusCa => "fr-CA",
            Hr => "hr",
            Hu => "hu",
            Id => "id",
            It => "it",
            Ja => "ja",
            Ko => "ko",
            Lt => "lt",
            Lv => "lv",
            Ms => "ms",
            Mt => "mt",
            Nb => "nb",
            Nl => "nl",
            Pl => "pl",
            Pt => "pt",
            PtMinusBr => "pt-BR",
            Ro => "ro",
            Ru => "ru",
            Sk => "sk",
            Sl => "sl",
            Sv => "sv",
            Th => "th",
            Tr => "tr",
            Vi => "vi",
            Zh => "zh",
            ZhMinusHk => "zh-HK",
            ZhMinusTw => "zh-TW",
        }
    }
}

impl std::str::FromStr for SessionLocale {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SessionLocale::*;
        match s {
            "auto" => Ok(Auto),
            "bg" => Ok(Bg),
            "cs" => Ok(Cs),
            "da" => Ok(Da),
            "de" => Ok(De),
            "el" => Ok(El),
            "en" => Ok(En),
            "en-GB" => Ok(EnMinusGb),
            "es" => Ok(Es),
            "es-419" => Ok(EsMinus419),
            "et" => Ok(Et),
            "fi" => Ok(Fi),
            "fil" => Ok(Fil),
            "fr" => Ok(Fr),
            "fr-CA" => Ok(FrMinusCa),
            "hr" => Ok(Hr),
            "hu" => Ok(Hu),
            "id" => Ok(Id),
            "it" => Ok(It),
            "ja" => Ok(Ja),
            "ko" => Ok(Ko),
            "lt" => Ok(Lt),
            "lv" => Ok(Lv),
            "ms" => Ok(Ms),
            "mt" => Ok(Mt),
            "nb" => Ok(Nb),
            "nl" => Ok(Nl),
            "pl" => Ok(Pl),
            "pt" => Ok(Pt),
            "pt-BR" => Ok(PtMinusBr),
            "ro" => Ok(Ro),
            "ru" => Ok(Ru),
            "sk" => Ok(Sk),
            "sl" => Ok(Sl),
            "sv" => Ok(Sv),
            "th" => Ok(Th),
            "tr" => Ok(Tr),
            "vi" => Ok(Vi),
            "zh" => Ok(Zh),
            "zh-HK" => Ok(ZhMinusHk),
            "zh-TW" => Ok(ZhMinusTw),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SessionLocale {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SessionLocale"))
    }
}
/// The mode of the Checkout Session.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SessionMode {
    Payment,
    Setup,
    Subscription,
}

impl SessionMode {
    pub fn as_str(self) -> &'static str {
        use SessionMode::*;
        match self {
            Payment => "payment",
            Setup => "setup",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for SessionMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SessionMode::*;
        match s {
            "payment" => Ok(Payment),
            "setup" => Ok(Setup),
            "subscription" => Ok(Subscription),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SessionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SessionMode"))
    }
}
/// Configure whether a Checkout Session should collect a payment method.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SessionPaymentMethodCollection {
    Always,
    IfRequired,
}

impl SessionPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        use SessionPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for SessionPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SessionPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SessionPaymentMethodCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| {
            serde::de::Error::custom("Unknown value for SessionPaymentMethodCollection")
        })
    }
}
/// The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
/// You can use this value to decide when to fulfill your customer's order.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SessionPaymentStatus {
    NoPaymentRequired,
    Paid,
    Unpaid,
}

impl SessionPaymentStatus {
    pub fn as_str(self) -> &'static str {
        use SessionPaymentStatus::*;
        match self {
            NoPaymentRequired => "no_payment_required",
            Paid => "paid",
            Unpaid => "unpaid",
        }
    }
}

impl std::str::FromStr for SessionPaymentStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SessionPaymentStatus::*;
        match s {
            "no_payment_required" => Ok(NoPaymentRequired),
            "paid" => Ok(Paid),
            "unpaid" => Ok(Unpaid),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SessionPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SessionPaymentStatus"))
    }
}
/// The status of the Checkout Session, one of `open`, `complete`, or `expired`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SessionStatus {
    Complete,
    Expired,
    Open,
}

impl SessionStatus {
    pub fn as_str(self) -> &'static str {
        use SessionStatus::*;
        match self {
            Complete => "complete",
            Expired => "expired",
            Open => "open",
        }
    }
}

impl std::str::FromStr for SessionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SessionStatus::*;
        match s {
            "complete" => Ok(Complete),
            "expired" => Ok(Expired),
            "open" => Ok(Open),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s).map_err(|_| serde::de::Error::custom("Unknown value for SessionStatus"))
    }
}
/// Describes the type of transaction being performed by Checkout in order to customize
/// relevant text on the page, such as the submit button.
///
/// `submit_type` can only be specified on Checkout Sessions in `payment` mode, but not Checkout Sessions in `subscription` or `setup` mode.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SessionSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

impl SessionSubmitType {
    pub fn as_str(self) -> &'static str {
        use SessionSubmitType::*;
        match self {
            Auto => "auto",
            Book => "book",
            Donate => "donate",
            Pay => "pay",
        }
    }
}

impl std::str::FromStr for SessionSubmitType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use SessionSubmitType::*;
        match s {
            "auto" => Ok(Auto),
            "book" => Ok(Book),
            "donate" => Ok(Donate),
            "pay" => Ok(Pay),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for SessionSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        let s: &str = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(s)
            .map_err(|_| serde::de::Error::custom("Unknown value for SessionSubmitType"))
    }
}
impl stripe_types::Object for Session {
    type Id = stripe_checkout::session::CheckoutSessionId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(CheckoutSessionId, "cs_");
#[cfg(feature = "session")]
mod requests;
#[cfg(feature = "session")]
pub use requests::*;
