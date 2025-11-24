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
    /// Configuration for collecting the customer's billing address. Defaults to `auto`.
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
    pub name_collection: Option<stripe_shared::PaymentLinksResourceNameCollection>,
    /// The account on behalf of which to charge.
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// The optional items presented to the customer at checkout.
    pub optional_items: Option<Vec<stripe_shared::PaymentLinksResourceOptionalItem>>,
    /// Indicates the parameters to be passed to PaymentIntent creation during checkout.
    pub payment_intent_data: Option<stripe_shared::PaymentLinksResourcePaymentIntentData>,
    /// Configuration for collecting a payment method during checkout. Defaults to `always`.
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
    name_collection: Option<Option<stripe_shared::PaymentLinksResourceNameCollection>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    optional_items: Option<Option<Vec<stripe_shared::PaymentLinksResourceOptionalItem>>>,
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

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
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
                "name_collection" => Deserialize::begin(&mut self.name_collection),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "optional_items" => Deserialize::begin(&mut self.optional_items),
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
                name_collection: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                optional_items: Deserialize::default(),
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
            let (
                Some(active),
                Some(after_completion),
                Some(allow_promotion_codes),
                Some(application),
                Some(application_fee_amount),
                Some(application_fee_percent),
                Some(automatic_tax),
                Some(billing_address_collection),
                Some(consent_collection),
                Some(currency),
                Some(custom_fields),
                Some(custom_text),
                Some(customer_creation),
                Some(id),
                Some(inactive_message),
                Some(invoice_creation),
                Some(line_items),
                Some(livemode),
                Some(metadata),
                Some(name_collection),
                Some(on_behalf_of),
                Some(optional_items),
                Some(payment_intent_data),
                Some(payment_method_collection),
                Some(payment_method_types),
                Some(phone_number_collection),
                Some(restrictions),
                Some(shipping_address_collection),
                Some(shipping_options),
                Some(submit_type),
                Some(subscription_data),
                Some(tax_id_collection),
                Some(transfer_data),
                Some(url),
            ) = (
                self.active,
                self.after_completion.take(),
                self.allow_promotion_codes,
                self.application.take(),
                self.application_fee_amount,
                self.application_fee_percent,
                self.automatic_tax.take(),
                self.billing_address_collection.take(),
                self.consent_collection.take(),
                self.currency.take(),
                self.custom_fields.take(),
                self.custom_text.take(),
                self.customer_creation.take(),
                self.id.take(),
                self.inactive_message.take(),
                self.invoice_creation.take(),
                self.line_items.take(),
                self.livemode,
                self.metadata.take(),
                self.name_collection,
                self.on_behalf_of.take(),
                self.optional_items.take(),
                self.payment_intent_data.take(),
                self.payment_method_collection.take(),
                self.payment_method_types.take(),
                self.phone_number_collection,
                self.restrictions,
                self.shipping_address_collection.take(),
                self.shipping_options.take(),
                self.submit_type.take(),
                self.subscription_data.take(),
                self.tax_id_collection.take(),
                self.transfer_data.take(),
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                active,
                after_completion,
                allow_promotion_codes,
                application,
                application_fee_amount,
                application_fee_percent,
                automatic_tax,
                billing_address_collection,
                consent_collection,
                currency,
                custom_fields,
                custom_text,
                customer_creation,
                id,
                inactive_message,
                invoice_creation,
                line_items,
                livemode,
                metadata,
                name_collection,
                on_behalf_of,
                optional_items,
                payment_intent_data,
                payment_method_collection,
                payment_method_types,
                phone_number_collection,
                restrictions,
                shipping_address_collection,
                shipping_options,
                submit_type,
                subscription_data,
                tax_id_collection,
                transfer_data,
                url,
            })
        }
    }

    impl Map for Builder<'_> {
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
                    "active" => b.active = FromValueOpt::from_value(v),
                    "after_completion" => b.after_completion = FromValueOpt::from_value(v),
                    "allow_promotion_codes" => {
                        b.allow_promotion_codes = FromValueOpt::from_value(v)
                    }
                    "application" => b.application = FromValueOpt::from_value(v),
                    "application_fee_amount" => {
                        b.application_fee_amount = FromValueOpt::from_value(v)
                    }
                    "application_fee_percent" => {
                        b.application_fee_percent = FromValueOpt::from_value(v)
                    }
                    "automatic_tax" => b.automatic_tax = FromValueOpt::from_value(v),
                    "billing_address_collection" => {
                        b.billing_address_collection = FromValueOpt::from_value(v)
                    }
                    "consent_collection" => b.consent_collection = FromValueOpt::from_value(v),
                    "currency" => b.currency = FromValueOpt::from_value(v),
                    "custom_fields" => b.custom_fields = FromValueOpt::from_value(v),
                    "custom_text" => b.custom_text = FromValueOpt::from_value(v),
                    "customer_creation" => b.customer_creation = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "inactive_message" => b.inactive_message = FromValueOpt::from_value(v),
                    "invoice_creation" => b.invoice_creation = FromValueOpt::from_value(v),
                    "line_items" => b.line_items = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "name_collection" => b.name_collection = FromValueOpt::from_value(v),
                    "on_behalf_of" => b.on_behalf_of = FromValueOpt::from_value(v),
                    "optional_items" => b.optional_items = FromValueOpt::from_value(v),
                    "payment_intent_data" => b.payment_intent_data = FromValueOpt::from_value(v),
                    "payment_method_collection" => {
                        b.payment_method_collection = FromValueOpt::from_value(v)
                    }
                    "payment_method_types" => b.payment_method_types = FromValueOpt::from_value(v),
                    "phone_number_collection" => {
                        b.phone_number_collection = FromValueOpt::from_value(v)
                    }
                    "restrictions" => b.restrictions = FromValueOpt::from_value(v),
                    "shipping_address_collection" => {
                        b.shipping_address_collection = FromValueOpt::from_value(v)
                    }
                    "shipping_options" => b.shipping_options = FromValueOpt::from_value(v),
                    "submit_type" => b.submit_type = FromValueOpt::from_value(v),
                    "subscription_data" => b.subscription_data = FromValueOpt::from_value(v),
                    "tax_id_collection" => b.tax_id_collection = FromValueOpt::from_value(v),
                    "transfer_data" => b.transfer_data = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),
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
        let mut s = s.serialize_struct("PaymentLink", 35)?;
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
        s.serialize_field("name_collection", &self.name_collection)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("optional_items", &self.optional_items)?;
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinkCustomerCreation {
    Always,
    IfRequired,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinkCustomerCreation {
    pub fn as_str(&self) -> &str {
        use PaymentLinkCustomerCreation::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinkCustomerCreation {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkCustomerCreation::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinkCustomerCreation"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        self.out = Some(PaymentLinkCustomerCreation::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkCustomerCreation);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinkCustomerCreation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Configuration for collecting a payment method during checkout. Defaults to `always`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinkPaymentMethodCollection {
    pub fn as_str(&self) -> &str {
        use PaymentLinkPaymentMethodCollection::*;
        match self {
            Always => "always",
            IfRequired => "if_required",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinkPaymentMethodCollection {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkPaymentMethodCollection::*;
        match s {
            "always" => Ok(Always),
            "if_required" => Ok(IfRequired),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinkPaymentMethodCollection"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        self.out = Some(PaymentLinkPaymentMethodCollection::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkPaymentMethodCollection);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinkPaymentMethodCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinkBillingAddressCollection {
    Auto,
    Required,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinkBillingAddressCollection {
    pub fn as_str(&self) -> &str {
        use PaymentLinkBillingAddressCollection::*;
        match self {
            Auto => "auto",
            Required => "required",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinkBillingAddressCollection {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkBillingAddressCollection::*;
        match s {
            "auto" => Ok(Auto),
            "required" => Ok(Required),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinkBillingAddressCollection"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        self.out = Some(PaymentLinkBillingAddressCollection::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkBillingAddressCollection);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinkBillingAddressCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinkPaymentMethodTypes {
    Affirm,
    AfterpayClearpay,
    Alipay,
    Alma,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Billie,
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
    MbWay,
    Mobilepay,
    Multibanco,
    Oxxo,
    P24,
    PayByBank,
    Paynow,
    Paypal,
    Pix,
    Promptpay,
    Satispay,
    SepaDebit,
    Sofort,
    Swish,
    Twint,
    UsBankAccount,
    WechatPay,
    Zip,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinkPaymentMethodTypes {
    pub fn as_str(&self) -> &str {
        use PaymentLinkPaymentMethodTypes::*;
        match self {
            Affirm => "affirm",
            AfterpayClearpay => "afterpay_clearpay",
            Alipay => "alipay",
            Alma => "alma",
            AuBecsDebit => "au_becs_debit",
            BacsDebit => "bacs_debit",
            Bancontact => "bancontact",
            Billie => "billie",
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
            MbWay => "mb_way",
            Mobilepay => "mobilepay",
            Multibanco => "multibanco",
            Oxxo => "oxxo",
            P24 => "p24",
            PayByBank => "pay_by_bank",
            Paynow => "paynow",
            Paypal => "paypal",
            Pix => "pix",
            Promptpay => "promptpay",
            Satispay => "satispay",
            SepaDebit => "sepa_debit",
            Sofort => "sofort",
            Swish => "swish",
            Twint => "twint",
            UsBankAccount => "us_bank_account",
            WechatPay => "wechat_pay",
            Zip => "zip",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinkPaymentMethodTypes {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkPaymentMethodTypes::*;
        match s {
            "affirm" => Ok(Affirm),
            "afterpay_clearpay" => Ok(AfterpayClearpay),
            "alipay" => Ok(Alipay),
            "alma" => Ok(Alma),
            "au_becs_debit" => Ok(AuBecsDebit),
            "bacs_debit" => Ok(BacsDebit),
            "bancontact" => Ok(Bancontact),
            "billie" => Ok(Billie),
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
            "mb_way" => Ok(MbWay),
            "mobilepay" => Ok(Mobilepay),
            "multibanco" => Ok(Multibanco),
            "oxxo" => Ok(Oxxo),
            "p24" => Ok(P24),
            "pay_by_bank" => Ok(PayByBank),
            "paynow" => Ok(Paynow),
            "paypal" => Ok(Paypal),
            "pix" => Ok(Pix),
            "promptpay" => Ok(Promptpay),
            "satispay" => Ok(Satispay),
            "sepa_debit" => Ok(SepaDebit),
            "sofort" => Ok(Sofort),
            "swish" => Ok(Swish),
            "twint" => Ok(Twint),
            "us_bank_account" => Ok(UsBankAccount),
            "wechat_pay" => Ok(WechatPay),
            "zip" => Ok(Zip),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "PaymentLinkPaymentMethodTypes"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        self.out = Some(PaymentLinkPaymentMethodTypes::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkPaymentMethodTypes);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinkPaymentMethodTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum PaymentLinkSubmitType {
    Auto,
    Book,
    Donate,
    Pay,
    Subscribe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl PaymentLinkSubmitType {
    pub fn as_str(&self) -> &str {
        use PaymentLinkSubmitType::*;
        match self {
            Auto => "auto",
            Book => "book",
            Donate => "donate",
            Pay => "pay",
            Subscribe => "subscribe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for PaymentLinkSubmitType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use PaymentLinkSubmitType::*;
        match s {
            "auto" => Ok(Auto),
            "book" => Ok(Book),
            "donate" => Ok(Donate),
            "pay" => Ok(Pay),
            "subscribe" => Ok(Subscribe),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "PaymentLinkSubmitType");
                Ok(Unknown(v.to_owned()))
            }
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
        self.out = Some(PaymentLinkSubmitType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(PaymentLinkSubmitType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for PaymentLinkSubmitType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
