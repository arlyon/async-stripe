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
    pub after_completion: crate::payment_link::after_completion::AfterCompletion,
    /// Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: crate::payment_link::automatic_tax::AutomaticTax,
    /// Configuration for collecting the customer's billing address.
    pub billing_address_collection: PaymentLinkBillingAddressCollection,
    /// When set, provides configuration to gather active consent from customers.
    pub consent_collection: Option<crate::payment_link::consent_collection::ConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// Configuration for Customer creation during checkout.
    pub customer_creation: PaymentLinkCustomerCreation,
    /// Unique identifier for the object.
    pub id: crate::payment_link::PaymentLinkId,
    /// The line items representing what is being sold.
    #[serde(default)]
    pub line_items: crate::List<Option<crate::line_item::LineItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: PaymentLinkObject,
    /// The account on behalf of which to charge.
    ///
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<crate::Expandable<crate::account::Account>>,
    /// Indicates the parameters to be passed to PaymentIntent creation during checkout.
    pub payment_intent_data: Option<crate::payment_link::payment_intent_data::PaymentIntentData>,
    /// Configuration for collecting a payment method during checkout.
    pub payment_method_collection: PaymentLinkPaymentMethodCollection,
    /// The list of payment method types that customers can use.
    ///
    /// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<PaymentLinkPaymentMethodTypes>>,
    pub phone_number_collection:
        crate::payment_link::phone_number_collection::PhoneNumberCollection,
    /// Configuration for collecting the customer's shipping address.
    pub shipping_address_collection:
        Option<crate::payment_link::shipping_address_collection::ShippingAddressCollection>,
    /// The shipping rate options applied to the session.
    pub shipping_options: Vec<crate::payment_link::shipping_option::ShippingOption>,
    /// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
    pub submit_type: PaymentLinkSubmitType,
    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub subscription_data: Option<crate::payment_link::subscription_data::SubscriptionData>,
    pub tax_id_collection: crate::payment_link::tax_id_collection::TaxIdCollection,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub transfer_data: Option<crate::payment_link::transfer_data::TransferData>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// Configuration for Customer creation during checkout.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// Configuration for collecting a payment method during checkout.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// The list of payment method types that customers can use.
///
/// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
/// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
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
impl crate::Object for PaymentLink {
    type Id = crate::payment_link::PaymentLinkId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(PaymentLinkId, "plink_");
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
