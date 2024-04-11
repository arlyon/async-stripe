/// A payment link is a shareable URL that will take your customers to a hosted payment page.
/// A payment link can be shared and used multiple times.
///
/// When a customer opens a payment link it will open a new [checkout session](https://stripe.com/docs/api/checkout/sessions) to render the payment page.
/// You can use [checkout session events](https://stripe.com/docs/api/events/types#event_types-checkout.session.completed) to track payments through payment links.
///
/// Related guide: [Payment Links API](https://stripe.com/docs/payment-links)
///
/// For more details see <<https://stripe.com/docs/api/payment_links/payment_links/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PaymentLink {
    /// Whether the payment link's `url` is active.
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub active: bool,
    pub after_completion: stripe_shared::PaymentLinksResourceAfterCompletion,
    /// Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,
    /// The ID of the Connect application that created the Payment Link.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: stripe_shared::PaymentLinksResourceAutomaticTax,
    /// Configuration for collecting the customer's billing address.
    pub billing_address_collection: stripe_shared::PaymentLinkBillingAddressCollection,
    /// When set, provides configuration to gather active consent from customers.
    pub consent_collection: Option<stripe_shared::PaymentLinksResourceConsentCollection>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Collect additional information from your customer using custom fields.
    /// Up to 3 fields are supported.
    pub custom_fields: Vec<stripe_shared::PaymentLinksResourceCustomFields>,
    pub custom_text: stripe_shared::PaymentLinksResourceCustomText,
    /// Configuration for Customer creation during checkout.
    pub customer_creation: PaymentLinkCustomerCreation,
    /// Unique identifier for the object.
    pub id: stripe_shared::PaymentLinkId,
    /// The custom message to be displayed to a customer when a payment link is no longer active.
    pub inactive_message: Option<String>,
    /// Configuration for creating invoice for payment mode payment links.
    pub invoice_creation: Option<stripe_shared::PaymentLinksResourceInvoiceCreation>,
    /// The line items representing what is being sold.
    pub line_items: Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The account on behalf of which to charge.
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// Indicates the parameters to be passed to PaymentIntent creation during checkout.
    pub payment_intent_data: Option<stripe_shared::PaymentLinksResourcePaymentIntentData>,
    /// Configuration for collecting a payment method during checkout.
    pub payment_method_collection: PaymentLinkPaymentMethodCollection,
    /// The list of payment method types that customers can use.
    /// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>,
    pub phone_number_collection: stripe_shared::PaymentLinksResourcePhoneNumberCollection,
    /// Settings that restrict the usage of a payment link.
    pub restrictions: Option<stripe_shared::PaymentLinksResourceRestrictions>,
    /// Configuration for collecting the customer's shipping address.
    pub shipping_address_collection:
        Option<stripe_shared::PaymentLinksResourceShippingAddressCollection>,
    /// The shipping rate options applied to the session.
    pub shipping_options: Vec<stripe_shared::PaymentLinksResourceShippingOption>,
    /// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
    pub submit_type: stripe_shared::PaymentLinkSubmitType,
    /// When creating a subscription, the specified configuration data will be used.
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub subscription_data: Option<stripe_shared::PaymentLinksResourceSubscriptionData>,
    pub tax_id_collection: stripe_shared::PaymentLinksResourceTaxIdCollection,
    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub transfer_data: Option<stripe_shared::PaymentLinksResourceTransferData>,
    /// The public URL that can be shared with customers.
    pub url: String,
}
#[doc(hidden)]
pub struct PaymentLinkBuilder {
    active: Option<bool>,
    after_completion: Option<stripe_shared::PaymentLinksResourceAfterCompletion>,
    allow_promotion_codes: Option<bool>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    application_fee_amount: Option<Option<i64>>,
    application_fee_percent: Option<Option<f64>>,
    automatic_tax: Option<stripe_shared::PaymentLinksResourceAutomaticTax>,
    billing_address_collection: Option<stripe_shared::PaymentLinkBillingAddressCollection>,
    consent_collection: Option<Option<stripe_shared::PaymentLinksResourceConsentCollection>>,
    currency: Option<stripe_types::Currency>,
    custom_fields: Option<Vec<stripe_shared::PaymentLinksResourceCustomFields>>,
    custom_text: Option<stripe_shared::PaymentLinksResourceCustomText>,
    customer_creation: Option<PaymentLinkCustomerCreation>,
    id: Option<stripe_shared::PaymentLinkId>,
    inactive_message: Option<Option<String>>,
    invoice_creation: Option<Option<stripe_shared::PaymentLinksResourceInvoiceCreation>>,
    line_items: Option<Option<stripe_types::List<stripe_shared::CheckoutSessionItem>>>,
    livemode: Option<bool>,
    metadata: Option<std::collections::HashMap<String, String>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    payment_intent_data: Option<Option<stripe_shared::PaymentLinksResourcePaymentIntentData>>,
    payment_method_collection: Option<PaymentLinkPaymentMethodCollection>,
    payment_method_types: Option<Option<Vec<stripe_shared::PaymentLinkPaymentMethodTypes>>>,
    phone_number_collection: Option<stripe_shared::PaymentLinksResourcePhoneNumberCollection>,
    restrictions: Option<Option<stripe_shared::PaymentLinksResourceRestrictions>>,
    shipping_address_collection:
        Option<Option<stripe_shared::PaymentLinksResourceShippingAddressCollection>>,
    shipping_options: Option<Vec<stripe_shared::PaymentLinksResourceShippingOption>>,
    submit_type: Option<stripe_shared::PaymentLinkSubmitType>,
    subscription_data: Option<Option<stripe_shared::PaymentLinksResourceSubscriptionData>>,
    tax_id_collection: Option<stripe_shared::PaymentLinksResourceTaxIdCollection>,
    transfer_data: Option<Option<stripe_shared::PaymentLinksResourceTransferData>>,
    url: Option<String>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for PaymentLink {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PaymentLink>,
        builder: PaymentLinkBuilder,
    }

    impl Visitor for Place<PaymentLink> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PaymentLinkBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PaymentLinkBuilder {
        type Out = PaymentLink;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "after_completion" => Deserialize::begin(&mut self.after_completion),
                "allow_promotion_codes" => Deserialize::begin(&mut self.allow_promotion_codes),
                "application" => Deserialize::begin(&mut self.application),
                "application_fee_amount" => Deserialize::begin(&mut self.application_fee_amount),
                "application_fee_percent" => Deserialize::begin(&mut self.application_fee_percent),
                "automatic_tax" => Deserialize::begin(&mut self.automatic_tax),
                "billing_address_collection" => {
                    Deserialize::begin(&mut self.billing_address_collection)
                }
                "consent_collection" => Deserialize::begin(&mut self.consent_collection),
                "currency" => Deserialize::begin(&mut self.currency),
                "custom_fields" => Deserialize::begin(&mut self.custom_fields),
                "custom_text" => Deserialize::begin(&mut self.custom_text),
                "customer_creation" => Deserialize::begin(&mut self.customer_creation),
                "id" => Deserialize::begin(&mut self.id),
                "inactive_message" => Deserialize::begin(&mut self.inactive_message),
                "invoice_creation" => Deserialize::begin(&mut self.invoice_creation),
                "line_items" => Deserialize::begin(&mut self.line_items),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "payment_intent_data" => Deserialize::begin(&mut self.payment_intent_data),
                "payment_method_collection" => {
                    Deserialize::begin(&mut self.payment_method_collection)
                }
                "payment_method_types" => Deserialize::begin(&mut self.payment_method_types),
                "phone_number_collection" => Deserialize::begin(&mut self.phone_number_collection),
                "restrictions" => Deserialize::begin(&mut self.restrictions),
                "shipping_address_collection" => {
                    Deserialize::begin(&mut self.shipping_address_collection)
                }
                "shipping_options" => Deserialize::begin(&mut self.shipping_options),
                "submit_type" => Deserialize::begin(&mut self.submit_type),
                "subscription_data" => Deserialize::begin(&mut self.subscription_data),
                "tax_id_collection" => Deserialize::begin(&mut self.tax_id_collection),
                "transfer_data" => Deserialize::begin(&mut self.transfer_data),
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                after_completion: Deserialize::default(),
                allow_promotion_codes: Deserialize::default(),
                application: Deserialize::default(),
                application_fee_amount: Deserialize::default(),
                application_fee_percent: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                billing_address_collection: Deserialize::default(),
                consent_collection: Deserialize::default(),
                currency: Deserialize::default(),
                custom_fields: Deserialize::default(),
                custom_text: Deserialize::default(),
                customer_creation: Deserialize::default(),
                id: Deserialize::default(),
                inactive_message: Deserialize::default(),
                invoice_creation: Deserialize::default(),
                line_items: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                payment_intent_data: Deserialize::default(),
                payment_method_collection: Deserialize::default(),
                payment_method_types: Deserialize::default(),
                phone_number_collection: Deserialize::default(),
                restrictions: Deserialize::default(),
                shipping_address_collection: Deserialize::default(),
                shipping_options: Deserialize::default(),
                submit_type: Deserialize::default(),
                subscription_data: Deserialize::default(),
                tax_id_collection: Deserialize::default(),
                transfer_data: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                active: self.active?,
                after_completion: self.after_completion.take()?,
                allow_promotion_codes: self.allow_promotion_codes?,
                application: self.application.take()?,
                application_fee_amount: self.application_fee_amount?,
                application_fee_percent: self.application_fee_percent?,
                automatic_tax: self.automatic_tax.take()?,
                billing_address_collection: self.billing_address_collection?,
                consent_collection: self.consent_collection?,
                currency: self.currency?,
                custom_fields: self.custom_fields.take()?,
                custom_text: self.custom_text.take()?,
                customer_creation: self.customer_creation?,
                id: self.id.take()?,
                inactive_message: self.inactive_message.take()?,
                invoice_creation: self.invoice_creation.take()?,
                line_items: self.line_items.take()?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                on_behalf_of: self.on_behalf_of.take()?,
                payment_intent_data: self.payment_intent_data.take()?,
                payment_method_collection: self.payment_method_collection?,
                payment_method_types: self.payment_method_types.take()?,
                phone_number_collection: self.phone_number_collection?,
                restrictions: self.restrictions?,
                shipping_address_collection: self.shipping_address_collection.take()?,
                shipping_options: self.shipping_options.take()?,
                submit_type: self.submit_type?,
                subscription_data: self.subscription_data.take()?,
                tax_id_collection: self.tax_id_collection?,
                transfer_data: self.transfer_data.take()?,
                url: self.url.take()?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for PaymentLink {
        type Builder = PaymentLinkBuilder;
    }

    impl FromValueOpt for PaymentLink {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PaymentLinkBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = Some(FromValueOpt::from_value(v)?),
                    "after_completion" => b.after_completion = Some(FromValueOpt::from_value(v)?),
                    "allow_promotion_codes" => {
                        b.allow_promotion_codes = Some(FromValueOpt::from_value(v)?)
                    }
                    "application" => b.application = Some(FromValueOpt::from_value(v)?),
                    "application_fee_amount" => {
                        b.application_fee_amount = Some(FromValueOpt::from_value(v)?)
                    }
                    "application_fee_percent" => {
                        b.application_fee_percent = Some(FromValueOpt::from_value(v)?)
                    }
                    "automatic_tax" => b.automatic_tax = Some(FromValueOpt::from_value(v)?),
                    "billing_address_collection" => {
                        b.billing_address_collection = Some(FromValueOpt::from_value(v)?)
                    }
                    "consent_collection" => {
                        b.consent_collection = Some(FromValueOpt::from_value(v)?)
                    }
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "custom_fields" => b.custom_fields = Some(FromValueOpt::from_value(v)?),
                    "custom_text" => b.custom_text = Some(FromValueOpt::from_value(v)?),
                    "customer_creation" => b.customer_creation = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "inactive_message" => b.inactive_message = Some(FromValueOpt::from_value(v)?),
                    "invoice_creation" => b.invoice_creation = Some(FromValueOpt::from_value(v)?),
                    "line_items" => b.line_items = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "on_behalf_of" => b.on_behalf_of = Some(FromValueOpt::from_value(v)?),
                    "payment_intent_data" => {
                        b.payment_intent_data = Some(FromValueOpt::from_value(v)?)
                    }
                    "payment_method_collection" => {
                        b.payment_method_collection = Some(FromValueOpt::from_value(v)?)
                    }
                    "payment_method_types" => {
                        b.payment_method_types = Some(FromValueOpt::from_value(v)?)
                    }
                    "phone_number_collection" => {
                        b.phone_number_collection = Some(FromValueOpt::from_value(v)?)
                    }
                    "restrictions" => b.restrictions = Some(FromValueOpt::from_value(v)?),
                    "shipping_address_collection" => {
                        b.shipping_address_collection = Some(FromValueOpt::from_value(v)?)
                    }
                    "shipping_options" => b.shipping_options = Some(FromValueOpt::from_value(v)?),
                    "submit_type" => b.submit_type = Some(FromValueOpt::from_value(v)?),
                    "subscription_data" => b.subscription_data = Some(FromValueOpt::from_value(v)?),
                    "tax_id_collection" => b.tax_id_collection = Some(FromValueOpt::from_value(v)?),
                    "transfer_data" => b.transfer_data = Some(FromValueOpt::from_value(v)?),
                    "url" => b.url = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLink {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("PaymentLink", 33)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("after_completion", &self.after_completion)?;
        s.serialize_field("allow_promotion_codes", &self.allow_promotion_codes)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("application_fee_amount", &self.application_fee_amount)?;
        s.serialize_field("application_fee_percent", &self.application_fee_percent)?;
        s.serialize_field("automatic_tax", &self.automatic_tax)?;
        s.serialize_field("billing_address_collection", &self.billing_address_collection)?;
        s.serialize_field("consent_collection", &self.consent_collection)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("custom_fields", &self.custom_fields)?;
        s.serialize_field("custom_text", &self.custom_text)?;
        s.serialize_field("customer_creation", &self.customer_creation)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("inactive_message", &self.inactive_message)?;
        s.serialize_field("invoice_creation", &self.invoice_creation)?;
        s.serialize_field("line_items", &self.line_items)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("payment_intent_data", &self.payment_intent_data)?;
        s.serialize_field("payment_method_collection", &self.payment_method_collection)?;
        s.serialize_field("payment_method_types", &self.payment_method_types)?;
        s.serialize_field("phone_number_collection", &self.phone_number_collection)?;
        s.serialize_field("restrictions", &self.restrictions)?;
        s.serialize_field("shipping_address_collection", &self.shipping_address_collection)?;
        s.serialize_field("shipping_options", &self.shipping_options)?;
        s.serialize_field("submit_type", &self.submit_type)?;
        s.serialize_field("subscription_data", &self.subscription_data)?;
        s.serialize_field("tax_id_collection", &self.tax_id_collection)?;
        s.serialize_field("transfer_data", &self.transfer_data)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "payment_link")?;
        s.end()
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
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinkCustomerCreation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentLinkCustomerCreation {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentLinkCustomerCreation> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkCustomerCreation::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkCustomerCreation);
#[cfg(feature = "deserialize")]
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
#[cfg(feature = "serialize")]
impl serde::Serialize for PaymentLinkPaymentMethodCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for PaymentLinkPaymentMethodCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentLinkPaymentMethodCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentLinkPaymentMethodCollection::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkPaymentMethodCollection);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinkPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinkPaymentMethodCollection")
        })
    }
}
impl stripe_types::Object for PaymentLink {
    type Id = stripe_shared::PaymentLinkId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(PaymentLinkId);
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
impl miniserde::Deserialize for PaymentLinkBillingAddressCollection {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentLinkBillingAddressCollection> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(PaymentLinkBillingAddressCollection::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkBillingAddressCollection);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinkBillingAddressCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for PaymentLinkBillingAddressCollection")
        })
    }
}
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
    Swish,
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
            Swish => "swish",
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
            "swish" => Ok(Swish),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            _ => Err(()),
        }
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
impl miniserde::Deserialize for PaymentLinkPaymentMethodTypes {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentLinkPaymentMethodTypes> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            PaymentLinkPaymentMethodTypes::from_str(s)
                .unwrap_or(PaymentLinkPaymentMethodTypes::Unknown),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkPaymentMethodTypes);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinkPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown))
    }
}
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
impl miniserde::Deserialize for PaymentLinkSubmitType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<PaymentLinkSubmitType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(PaymentLinkSubmitType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkSubmitType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinkSubmitType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for PaymentLinkSubmitType"))
    }
}
