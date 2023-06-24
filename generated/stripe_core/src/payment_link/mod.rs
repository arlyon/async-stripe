/// A payment link is a shareable URL that will take your customers to a hosted payment page.
///
/// A payment link can be shared and used multiple times.  When a customer opens a payment link it will open a new [checkout session](https://stripe.com/docs/api/checkout/sessions) to render the payment page.
/// You can use [checkout session events](https://stripe.com/docs/api/events/types#event_types-checkout.session.completed) to track payments through payment links.  Related guide: [Payment Links API](https://stripe.com/docs/payments/payment-links/api).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct PaymentLink {
    /// Whether the payment link's `url` is active.
    ///
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub active: bool,
    pub after_completion: stripe_core::payment_link::after_completion::AfterCompletion,
    /// Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_core::payment_link::automatic_tax::AutomaticTax,
    /// Configuration for collecting the customer's billing address.
    pub billing_address_collection: PaymentLinkBillingAddressCollection,
    /// When set, provides configuration to gather active consent from customers.
    pub consent_collection:
        Option<stripe_core::payment_link::consent_collection::ConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Configuration for Customer creation during checkout.
    pub customer_creation: PaymentLinkCustomerCreation,
    /// Unique identifier for the object.
    pub id: stripe_core::payment_link::PaymentLinkId,
    /// The line items representing what is being sold.
    #[serde(default)]
    pub line_items: stripe_types::List<Option<stripe_core::line_item::LineItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: PaymentLinkObject,
    /// The account on behalf of which to charge.
    ///
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_core::account::Account>>,
    /// Indicates the parameters to be passed to PaymentIntent creation during checkout.
    pub payment_intent_data:
        Option<stripe_core::payment_link::payment_intent_data::PaymentIntentData>,
    /// Configuration for collecting a payment method during checkout.
    pub payment_method_collection: PaymentLinkPaymentMethodCollection,
    /// The list of payment method types that customers can use.
    ///
    /// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<PaymentLinkPaymentMethodTypes>>,
    pub phone_number_collection:
        stripe_core::payment_link::phone_number_collection::PhoneNumberCollection,
    /// Configuration for collecting the customer's shipping address.
    pub shipping_address_collection:
        Option<stripe_core::payment_link::shipping_address_collection::ShippingAddressCollection>,
    /// The shipping rate options applied to the session.
    pub shipping_options: Vec<stripe_core::payment_link::shipping_option::ShippingOption>,
    /// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
    pub submit_type: PaymentLinkSubmitType,
    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub subscription_data: Option<stripe_core::payment_link::subscription_data::SubscriptionData>,
    pub tax_id_collection: stripe_core::payment_link::tax_id_collection::TaxIdCollection,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub transfer_data: Option<stripe_core::payment_link::transfer_data::TransferData>,
    /// The public URL that can be shared with customers.
    pub url: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLink {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// Configuration for collecting the customer's billing address.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentLinkBillingAddressCollection {
    Auto,
    Required,
}

impl PaymentLinkBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Required => "required",
        }
    }
}

impl std::str::FromStr for PaymentLinkBillingAddressCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "required" => Ok(Self::Required),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinkBillingAddressCollection")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkBillingAddressCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<PaymentLinkBillingAddressCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkBillingAddressCollection::from_str(s)?);
        Ok(())
    }
}
/// Configuration for Customer creation during checkout.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentLinkCustomerCreation {
    Always,
    IfRequired,
}

impl PaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for PaymentLinkCustomerCreation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "if_required" => Ok(Self::IfRequired),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkCustomerCreation"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkCustomerCreation {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<PaymentLinkCustomerCreation> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkCustomerCreation::from_str(s)?);
        Ok(())
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentLinkObject {
    PaymentLink,
}

impl PaymentLinkObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PaymentLink => "payment_link",
        }
    }
}

impl std::str::FromStr for PaymentLinkObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "payment_link" => Ok(Self::PaymentLink),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for PaymentLinkObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for PaymentLinkObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PaymentLinkObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<PaymentLinkObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkObject::from_str(s)?);
        Ok(())
    }
}
/// Configuration for collecting a payment method during checkout.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}

impl PaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::IfRequired => "if_required",
        }
    }
}

impl std::str::FromStr for PaymentLinkPaymentMethodCollection {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "always" => Ok(Self::Always),
            "if_required" => Ok(Self::IfRequired),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinkPaymentMethodCollection")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkPaymentMethodCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<PaymentLinkPaymentMethodCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkPaymentMethodCollection::from_str(s)?);
        Ok(())
    }
}
/// The list of payment method types that customers can use.
///
/// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
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

impl PaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
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

impl std::str::FromStr for PaymentLinkPaymentMethodTypes {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "affirm" => Ok(Self::Affirm),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "alipay" => Ok(Self::Alipay),
            "au_becs_debit" => Ok(Self::AuBecsDebit),
            "bacs_debit" => Ok(Self::BacsDebit),
            "bancontact" => Ok(Self::Bancontact),
            "blik" => Ok(Self::Blik),
            "boleto" => Ok(Self::Boleto),
            "card" => Ok(Self::Card),
            "eps" => Ok(Self::Eps),
            "fpx" => Ok(Self::Fpx),
            "giropay" => Ok(Self::Giropay),
            "grabpay" => Ok(Self::Grabpay),
            "ideal" => Ok(Self::Ideal),
            "klarna" => Ok(Self::Klarna),
            "konbini" => Ok(Self::Konbini),
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

impl AsRef<str> for PaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinkPaymentMethodTypes")
        })
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<PaymentLinkPaymentMethodTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkPaymentMethodTypes::from_str(s)?);
        Ok(())
    }
}
/// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PaymentLinkSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
}

impl PaymentLinkSubmitType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Book => "book",
            Self::Donate => "donate",
            Self::Pay => "pay",
        }
    }
}

impl std::str::FromStr for PaymentLinkSubmitType {
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

impl AsRef<str> for PaymentLinkSubmitType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinkSubmitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkSubmitType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for PaymentLinkSubmitType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<PaymentLinkSubmitType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkSubmitType::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for PaymentLink {
    type Id = stripe_core::payment_link::PaymentLinkId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(PaymentLinkId, "plink_");
pub mod after_completion;
pub mod requests;
pub use after_completion::AfterCompletion;
pub mod automatic_tax;
pub use automatic_tax::AutomaticTax;
pub mod consent_collection;
pub use consent_collection::ConsentCollection;
pub mod payment_intent_data;
pub use payment_intent_data::PaymentIntentData;
pub mod phone_number_collection;
pub use phone_number_collection::PhoneNumberCollection;
pub mod shipping_address_collection;
pub use shipping_address_collection::ShippingAddressCollection;
pub mod shipping_option;
pub use shipping_option::ShippingOption;
pub mod subscription_data;
pub use subscription_data::SubscriptionData;
pub mod tax_id_collection;
pub use tax_id_collection::TaxIdCollection;
pub mod transfer_data;
pub use transfer_data::TransferData;
