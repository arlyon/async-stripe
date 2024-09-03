// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::client::{Client, Response};
use crate::ids::PaymentLinkId;
use crate::params::{Expand, Expandable, List, Metadata, Object, Paginable};
use crate::resources::{
    Account, Application, CheckoutSessionItem, ConnectAccountReference, Currency,
    InvoiceSettingRenderingOptions, ShippingRate, SubscriptionsTrialsResourceTrialSettings, TaxId,
};

/// The resource representing a Stripe "PaymentLink".
///
/// For more details see <https://stripe.com/docs/api/payment-link/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLink {
    /// Unique identifier for the object.
    pub id: PaymentLinkId,

    /// Whether the payment link's `url` is active.
    ///
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub active: bool,

    pub after_completion: PaymentLinksResourceAfterCompletion,

    /// Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,

    /// The ID of the Connect application that created the Payment Link.
    pub application: Option<Expandable<Application>>,

    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,

    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    pub application_fee_percent: Option<f64>,

    pub automatic_tax: PaymentLinksResourceAutomaticTax,

    /// Configuration for collecting the customer's billing address.
    pub billing_address_collection: PaymentLinkBillingAddressCollection,

    /// When set, provides configuration to gather active consent from customers.
    pub consent_collection: Option<PaymentLinksResourceConsentCollection>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Collect additional information from your customer using custom fields.
    ///
    /// Up to 3 fields are supported.
    pub custom_fields: Vec<PaymentLinksResourceCustomFields>,

    pub custom_text: PaymentLinksResourceCustomText,

    /// Configuration for Customer creation during checkout.
    pub customer_creation: PaymentLinkCustomerCreation,

    /// The custom message to be displayed to a customer when a payment link is no longer active.
    pub inactive_message: Option<String>,

    /// Configuration for creating invoice for payment mode payment links.
    pub invoice_creation: Option<PaymentLinksResourceInvoiceCreation>,

    /// The line items representing what is being sold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<List<CheckoutSessionItem>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The account on behalf of which to charge.
    ///
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<Expandable<Account>>,

    /// Indicates the parameters to be passed to PaymentIntent creation during checkout.
    pub payment_intent_data: Option<PaymentLinksResourcePaymentIntentData>,

    /// Configuration for collecting a payment method during checkout.
    pub payment_method_collection: PaymentLinkPaymentMethodCollection,

    /// The list of payment method types that customers can use.
    ///
    /// When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<PaymentLinkPaymentMethodTypes>>,

    pub phone_number_collection: PaymentLinksResourcePhoneNumberCollection,

    /// Settings that restrict the usage of a payment link.
    pub restrictions: Option<PaymentLinksResourceRestrictions>,

    /// Configuration for collecting the customer's shipping address.
    pub shipping_address_collection: Option<PaymentLinksResourceShippingAddressCollection>,

    /// The shipping rate options applied to the session.
    pub shipping_options: Vec<PaymentLinksResourceShippingOption>,

    /// Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
    pub submit_type: PaymentLinkSubmitType,

    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    pub subscription_data: Option<PaymentLinksResourceSubscriptionData>,

    pub tax_id_collection: PaymentLinksResourceTaxIdCollection,

    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    pub transfer_data: Option<PaymentLinksResourceTransferData>,

    /// The public URL that can be shared with customers.
    pub url: String,
}

impl PaymentLink {
    /// Returns a list of your payment links.
    pub fn list(client: &Client, params: &ListPaymentLinks<'_>) -> Response<List<PaymentLink>> {
        client.get_query("/payment_links", params)
    }

    /// Creates a payment link.
    pub fn create(client: &Client, params: CreatePaymentLink<'_>) -> Response<PaymentLink> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/payment_links", &params)
    }

    /// Retrieve a payment link.
    pub fn retrieve(client: &Client, id: &PaymentLinkId, expand: &[&str]) -> Response<PaymentLink> {
        client.get_query(&format!("/payment_links/{}", id), Expand { expand })
    }

    /// Updates a payment link.
    pub fn update(
        client: &Client,
        id: &PaymentLinkId,
        params: UpdatePaymentLink<'_>,
    ) -> Response<PaymentLink> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/payment_links/{}", id), &params)
    }
}

impl Object for PaymentLink {
    type Id = PaymentLinkId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "payment_link"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceAfterCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<PaymentLinksResourceCompletionBehaviorConfirmationPage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<PaymentLinksResourceCompletionBehaviorRedirect>,

    /// The specified behavior after the purchase is complete.
    #[serde(rename = "type")]
    pub type_: PaymentLinksResourceAfterCompletionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceAutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<ConnectAccountReference>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPage {
    /// The custom message that is displayed to the customer after the purchase is complete.
    pub custom_message: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCompletionBehaviorRedirect {
    /// The URL the customer will be redirected to after the purchase is complete.
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceConsentCollection {
    /// Settings related to the payment method reuse text shown in the Checkout UI.
    pub payment_method_reuse_agreement: Option<PaymentLinksResourcePaymentMethodReuseAgreement>,

    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    pub promotions: Option<PaymentLinksResourceConsentCollectionPromotions>,

    /// If set to `required`, it requires cutomers to accept the terms of service before being able to pay.
    ///
    /// If set to `none`, customers won't be shown a checkbox to accept the terms of service.
    pub terms_of_service: Option<PaymentLinksResourceConsentCollectionTermsOfService>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCustomFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<PaymentLinksResourceCustomFieldsDropdown>,

    /// String of your choice that your integration can use to reconcile this field.
    ///
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,

    pub label: PaymentLinksResourceCustomFieldsLabel,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<PaymentLinksResourceCustomFieldsNumeric>,

    /// Whether the customer is required to complete the field before completing the Checkout Session.
    ///
    /// Defaults to `false`.
    pub optional: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<PaymentLinksResourceCustomFieldsText>,

    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: PaymentLinksResourceCustomFieldsType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCustomFieldsDropdown {
    /// The options available for the customer to select.
    ///
    /// Up to 200 options allowed.
    pub options: Vec<PaymentLinksResourceCustomFieldsDropdownOption>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCustomFieldsDropdownOption {
    /// The label for the option, displayed to the customer.
    ///
    /// Up to 100 characters.
    pub label: String,

    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    ///
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: Option<String>,

    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: PaymentLinksResourceCustomFieldsLabelType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    pub minimum_length: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCustomText {
    /// Custom text that should be displayed after the payment confirmation button.
    pub after_submit: Option<PaymentLinksResourceCustomTextPosition>,

    /// Custom text that should be displayed alongside shipping address collection.
    pub shipping_address: Option<PaymentLinksResourceCustomTextPosition>,

    /// Custom text that should be displayed alongside the payment confirmation button.
    pub submit: Option<PaymentLinksResourceCustomTextPosition>,

    /// Custom text that should be displayed in place of the default terms of service agreement text.
    pub terms_of_service_acceptance: Option<PaymentLinksResourceCustomTextPosition>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCustomTextPosition {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceInvoiceCreation {
    /// Enable creating an invoice on successful payment.
    pub enabled: bool,

    /// Configuration for the invoice.
    ///
    /// Default invoice values will be used if unspecified.
    pub invoice_data: Option<PaymentLinksResourceInvoiceSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceInvoiceSettings {
    /// The account tax IDs associated with the invoice.
    pub account_tax_ids: Option<Vec<Expandable<TaxId>>>,

    /// A list of up to 4 custom fields to be displayed on the invoice.
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// Footer to be displayed on the invoice.
    pub footer: Option<String>,

    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    pub issuer: Option<ConnectAccountReference>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<Metadata>,

    /// Options for invoice PDF rendering.
    pub rendering_options: Option<InvoiceSettingRenderingOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceSettingCustomField {
    /// The name of the custom field.
    pub name: String,

    /// The value of the custom field.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourcePaymentIntentData {
    /// Indicates when the funds will be captured from the customer's account.
    pub capture_method: Option<PaymentLinksResourcePaymentIntentDataCaptureMethod>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    pub metadata: Metadata,

    /// Indicates that you intend to make future payments with the payment method collected during checkout.
    pub setup_future_usage: Option<PaymentLinksResourcePaymentIntentDataSetupFutureUsage>,

    /// Extra information about the payment.
    ///
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that's set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,

    /// A string that identifies the resulting payment as part of a group.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourcePaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    ///
    /// When set to `auto`, Stripe's defaults will be used.  When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: PaymentLinksResourcePaymentMethodReuseAgreementPosition,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourcePhoneNumberCollection {
    /// If `true`, a phone number will be collected during checkout.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceRestrictions {
    pub completed_sessions: PaymentLinksResourceCompletedSessions,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceCompletedSessions {
    /// The current number of checkout sessions that have been completed on the payment link which count towards the `completed_sessions` restriction to be met.
    pub count: u64,

    /// The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met.
    pub limit: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<PaymentLinksResourceShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceShippingOption {
    /// A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,

    /// The ID of the Shipping Rate to use for this shipping option.
    pub shipping_rate: Expandable<ShippingRate>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    pub description: Option<String>,

    pub invoice_settings: PaymentLinksResourceSubscriptionDataInvoiceSettings,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    pub metadata: Metadata,

    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,

    /// Settings related to subscription trials.
    pub trial_settings: Option<SubscriptionsTrialsResourceTrialSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceSubscriptionDataInvoiceSettings {
    pub issuer: ConnectAccountReference,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceTaxIdCollection {
    /// Indicates whether tax ID collection is enabled for the session.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PaymentLinksResourceTransferData {
    /// The amount in cents (or local equivalent) that will be transferred to the destination account.
    ///
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,

    /// The connected account receiving the transfer.
    pub destination: Expandable<Account>,
}

/// The parameters for `PaymentLink::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreatePaymentLink<'a> {
    /// Behavior after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<CreatePaymentLinkAfterCompletion>,

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// Can only be applied when there are no line items with recurring prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Configuration for automatic tax collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreatePaymentLinkAutomaticTax>,

    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<PaymentLinkBillingAddressCollection>,

    /// Configure fields to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<CreatePaymentLinkConsentCollection>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies) and supported by each line item's price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Collect additional information from your customer using custom fields.
    ///
    /// Up to 3 fields are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreatePaymentLinkCustomFields>>,

    /// Display additional text for your customers using custom text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<CreatePaymentLinkCustomText>,

    /// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<PaymentLinkCustomerCreation>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The custom message to be displayed to a customer when a payment link is no longer active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_message: Option<&'a str>,

    /// Generate a post-purchase Invoice for one-time payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<CreatePaymentLinkInvoiceCreation>,

    /// The line items representing what is being sold.
    ///
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    pub line_items: Vec<CreatePaymentLinkLineItems>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The account on behalf of which to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<CreatePaymentLinkPaymentIntentData>,

    /// Specify whether Checkout should collect a payment method.
    ///
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<PaymentLinkPaymentMethodCollection>,

    /// The list of payment method types that customers can use.
    ///
    /// If no value is passed, Stripe will dynamically show relevant payment methods from your [payment method settings](https://dashboard.stripe.com/settings/payment_methods) (20+ payment methods [supported](https://stripe.com/docs/payments/payment-methods/integration-options#payment-method-product-support)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<CreatePaymentLinkPaymentMethodTypes>>,

    /// Controls phone number collection settings during checkout.
    ///
    /// We recommend that you review your privacy policy and check with your legal contacts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<CreatePaymentLinkPhoneNumberCollection>,

    /// Settings that restrict the usage of a payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<CreatePaymentLinkRestrictions>,

    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<CreatePaymentLinkShippingAddressCollection>,

    /// The shipping rate options to apply to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<CreatePaymentLinkShippingOptions>>,

    /// Describes the type of transaction being performed in order to customize relevant text on the page, such as the submit button.
    ///
    /// Changing this value will also affect the hostname in the [url](https://stripe.com/docs/api/payment_links/payment_links/object#url) property (example: `donate.stripe.com`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<PaymentLinkSubmitType>,

    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreatePaymentLinkSubscriptionData>,

    /// Controls tax ID collection during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<CreatePaymentLinkTaxIdCollection>,

    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreatePaymentLinkTransferData>,
}

impl<'a> CreatePaymentLink<'a> {
    pub fn new(line_items: Vec<CreatePaymentLinkLineItems>) -> Self {
        CreatePaymentLink {
            after_completion: Default::default(),
            allow_promotion_codes: Default::default(),
            application_fee_amount: Default::default(),
            application_fee_percent: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            consent_collection: Default::default(),
            currency: Default::default(),
            custom_fields: Default::default(),
            custom_text: Default::default(),
            customer_creation: Default::default(),
            expand: Default::default(),
            inactive_message: Default::default(),
            invoice_creation: Default::default(),
            line_items,
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            payment_intent_data: Default::default(),
            payment_method_collection: Default::default(),
            payment_method_types: Default::default(),
            phone_number_collection: Default::default(),
            restrictions: Default::default(),
            shipping_address_collection: Default::default(),
            shipping_options: Default::default(),
            submit_type: Default::default(),
            subscription_data: Default::default(),
            tax_id_collection: Default::default(),
            transfer_data: Default::default(),
        }
    }
}

/// The parameters for `PaymentLink::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListPaymentLinks<'a> {
    /// Only return payment links that are active or inactive (e.g., pass `false` to list all inactive payment links).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<PaymentLinkId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<PaymentLinkId>,
}

impl<'a> ListPaymentLinks<'a> {
    pub fn new() -> Self {
        ListPaymentLinks {
            active: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListPaymentLinks<'_> {
    type O = PaymentLink;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `PaymentLink::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdatePaymentLink<'a> {
    /// Whether the payment link's `url` is active.
    ///
    /// If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Behavior after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_completion: Option<UpdatePaymentLinkAfterCompletion>,

    /// Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,

    /// Configuration for automatic tax collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdatePaymentLinkAutomaticTax>,

    /// Configuration for collecting the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<PaymentLinkBillingAddressCollection>,

    /// Collect additional information from your customer using custom fields.
    ///
    /// Up to 3 fields are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<UpdatePaymentLinkCustomFields>>,

    /// Display additional text for your customers using custom text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_text: Option<UpdatePaymentLinkCustomText>,

    /// Configures whether [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link create a [Customer](https://stripe.com/docs/api/customers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<PaymentLinkCustomerCreation>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The custom message to be displayed to a customer when a payment link is no longer active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_message: Option<String>,

    /// Generate a post-purchase Invoice for one-time payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<UpdatePaymentLinkInvoiceCreation>,

    /// The line items representing what is being sold.
    ///
    /// Each line item represents an item being sold.
    /// Up to 20 line items are supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<UpdatePaymentLinkLineItems>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// Metadata associated with this Payment Link will automatically be copied to [checkout sessions](https://stripe.com/docs/api/checkout/sessions) created by this payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// A subset of parameters to be passed to PaymentIntent creation for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<UpdatePaymentLinkPaymentIntentData>,

    /// Specify whether Checkout should collect a payment method.
    ///
    /// When set to `if_required`, Checkout will not collect a payment method when the total due for the session is 0.This may occur if the Checkout Session includes a free trial or a discount.  Can only be set in `subscription` mode.  If you'd like information on how to collect a payment method outside of Checkout, read the guide on [configuring subscriptions with a free trial](https://stripe.com/docs/payments/checkout/free-trials).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<PaymentLinkPaymentMethodCollection>,

    /// The list of payment method types that customers can use.
    ///
    /// Pass an empty string to enable dynamic payment methods that use your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<UpdatePaymentLinkPaymentMethodTypes>>,

    /// Settings that restrict the usage of a payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<UpdatePaymentLinkRestrictions>,

    /// Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<UpdatePaymentLinkShippingAddressCollection>,

    /// When creating a subscription, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price to use `subscription_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<UpdatePaymentLinkSubscriptionData>,
}

impl<'a> UpdatePaymentLink<'a> {
    pub fn new() -> Self {
        UpdatePaymentLink {
            active: Default::default(),
            after_completion: Default::default(),
            allow_promotion_codes: Default::default(),
            automatic_tax: Default::default(),
            billing_address_collection: Default::default(),
            custom_fields: Default::default(),
            custom_text: Default::default(),
            customer_creation: Default::default(),
            expand: Default::default(),
            inactive_message: Default::default(),
            invoice_creation: Default::default(),
            line_items: Default::default(),
            metadata: Default::default(),
            payment_intent_data: Default::default(),
            payment_method_collection: Default::default(),
            payment_method_types: Default::default(),
            restrictions: Default::default(),
            shipping_address_collection: Default::default(),
            subscription_data: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkAfterCompletion {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<CreatePaymentLinkAfterCompletionHostedConfirmation>,

    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<CreatePaymentLinkAfterCompletionRedirect>,

    /// The specified behavior after the purchase is complete.
    ///
    /// Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkAfterCompletionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkAutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreatePaymentLinkAutomaticTaxLiability>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkConsentCollection {
    /// Determines the display of payment method reuse agreement text in the UI.
    ///
    /// If set to `hidden`, it will hide legal text related to the reuse of a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_reuse_agreement:
        Option<CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreement>,

    /// If set to `auto`, enables the collection of customer consent for promotional communications.
    ///
    /// The Checkout Session will determine whether to display an option to opt into promotional communication from the merchant depending on the customer's locale.
    /// Only available to US merchants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<CreatePaymentLinkConsentCollectionPromotions>,

    /// If set to `required`, it requires customers to check a terms of service checkbox before being able to pay.
    /// There must be a valid terms of service URL set in your [Dashboard settings](https://dashboard.stripe.com/settings/public).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<CreatePaymentLinkConsentCollectionTermsOfService>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomFields {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<CreatePaymentLinkCustomFieldsDropdown>,

    /// String of your choice that your integration can use to reconcile this field.
    ///
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,

    /// The label for the field, displayed to the customer.
    pub label: CreatePaymentLinkCustomFieldsLabel,

    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<CreatePaymentLinkCustomFieldsNumeric>,

    /// Whether the customer is required to complete the field before completing the Checkout Session.
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<CreatePaymentLinkCustomFieldsText>,

    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkCustomFieldsType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomText {
    /// Custom text that should be displayed after the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<CreatePaymentLinkCustomTextAfterSubmit>,

    /// Custom text that should be displayed alongside shipping address collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreatePaymentLinkCustomTextShippingAddress>,

    /// Custom text that should be displayed alongside the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<CreatePaymentLinkCustomTextSubmit>,

    /// Custom text that should be displayed in place of the default terms of service agreement text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<CreatePaymentLinkCustomTextTermsOfServiceAcceptance>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkInvoiceCreation {
    /// Whether the feature is enabled.
    pub enabled: bool,

    /// Invoice PDF configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<CreatePaymentLinkInvoiceCreationInvoiceData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkLineItems {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<CreatePaymentLinkLineItemsAdjustableQuantity>,

    /// The ID of the [Price](https://stripe.com/docs/api/prices) or [Plan](https://stripe.com/docs/api/plans) object.
    pub price: String,

    /// The quantity of the line item being purchased.
    pub quantity: u64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkPaymentIntentData {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<CreatePaymentLinkPaymentIntentDataCaptureMethod>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    ///
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Indicates that you intend to [make future payments](https://stripe.com/docs/payments/payment-intents#future-usage) with the payment method collected by this Checkout Session.
    ///
    /// When setting this to `on_session`, Checkout will show a notice to the customer that their payment details will be saved.
    ///
    /// When setting this to `off_session`, Checkout will show a notice to the customer that their payment details will be saved and used for future payments.
    ///
    /// If a Customer has been provided or Checkout creates a new Customer,Checkout will attach the payment method to the Customer.
    ///
    /// If Checkout does not create a Customer, the payment method is not attached to a Customer.
    ///
    /// To reuse the payment method, you can retrieve it from the Checkout Session's PaymentIntent.  When processing card payments, Checkout also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as SCA.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CreatePaymentLinkPaymentIntentDataSetupFutureUsage>,

    /// Extra information about the payment.
    ///
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that's set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    /// A string that identifies the resulting payment as part of a group.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkPhoneNumberCollection {
    /// Set to `true` to enable phone number collection.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkRestrictions {
    /// Configuration for the `completed_sessions` restriction type.
    pub completed_sessions: CreatePaymentLinkRestrictionsCompletedSessions,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<CreatePaymentLinkShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkShippingOptions {
    /// The ID of the Shipping Rate to use for this shipping option.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreatePaymentLinkSubscriptionDataInvoiceSettings>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    ///
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Integer representing the number of trial period days before the customer is charged for the first time.
    ///
    /// Has to be at least 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,

    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<CreatePaymentLinkSubscriptionDataTrialSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkTaxIdCollection {
    /// Set to `true` to enable tax ID collection.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkTransferData {
    /// The amount that will be transferred automatically when a charge succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// If specified, successful charges will be attributed to the destination
    /// account for tax reporting, and the funds from charges will be transferred
    /// to the destination account.
    ///
    /// The ID of the resulting transfer will be returned on the successful charge's `transfer` field.
    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkAfterCompletion {
    /// Configuration when `type=hosted_confirmation`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation: Option<UpdatePaymentLinkAfterCompletionHostedConfirmation>,

    /// Configuration when `type=redirect`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<UpdatePaymentLinkAfterCompletionRedirect>,

    /// The specified behavior after the purchase is complete.
    ///
    /// Either `redirect` or `hosted_confirmation`.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkAfterCompletionType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkAutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpdatePaymentLinkAutomaticTaxLiability>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomFields {
    /// Configuration for `type=dropdown` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<UpdatePaymentLinkCustomFieldsDropdown>,

    /// String of your choice that your integration can use to reconcile this field.
    ///
    /// Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,

    /// The label for the field, displayed to the customer.
    pub label: UpdatePaymentLinkCustomFieldsLabel,

    /// Configuration for `type=numeric` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<UpdatePaymentLinkCustomFieldsNumeric>,

    /// Whether the customer is required to complete the field before completing the Checkout Session.
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,

    /// Configuration for `type=text` fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<UpdatePaymentLinkCustomFieldsText>,

    /// The type of the field.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkCustomFieldsType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomText {
    /// Custom text that should be displayed after the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_submit: Option<UpdatePaymentLinkCustomTextAfterSubmit>,

    /// Custom text that should be displayed alongside shipping address collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<UpdatePaymentLinkCustomTextShippingAddress>,

    /// Custom text that should be displayed alongside the payment confirmation button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<UpdatePaymentLinkCustomTextSubmit>,

    /// Custom text that should be displayed in place of the default terms of service agreement text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<UpdatePaymentLinkCustomTextTermsOfServiceAcceptance>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkInvoiceCreation {
    /// Whether the feature is enabled.
    pub enabled: bool,

    /// Invoice PDF configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<UpdatePaymentLinkInvoiceCreationInvoiceData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkLineItems {
    /// When set, provides configuration for this item’s quantity to be adjusted by the customer during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustable_quantity: Option<UpdatePaymentLinkLineItemsAdjustableQuantity>,

    /// The ID of an existing line item on the payment link.
    pub id: String,

    /// The quantity of the line item being purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkPaymentIntentData {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Payment Intents](https://stripe.com/docs/api/payment_intents) generated from this payment link.
    ///
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Extra information about the payment.
    ///
    /// This will appear on your customer's statement when this payment succeeds in creating a charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that's set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    /// A string that identifies the resulting payment as part of a group.
    ///
    /// See the PaymentIntents [use case for connected accounts](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkRestrictions {
    /// Configuration for the `completed_sessions` restriction type.
    pub completed_sessions: UpdatePaymentLinkRestrictionsCompletedSessions,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkShippingAddressCollection {
    /// An array of two-letter ISO country codes representing which countries Checkout should provide as options for
    /// shipping locations.
    ///
    /// Unsupported country codes: `AS, CX, CC, CU, HM, IR, KP, MH, FM, NF, MP, PW, SD, SY, UM, VI`.
    pub allowed_countries: Vec<UpdatePaymentLinkShippingAddressCollectionAllowedCountries>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkSubscriptionData {
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdatePaymentLinkSubscriptionDataInvoiceSettings>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will declaratively set metadata on [Subscriptions](https://stripe.com/docs/api/subscriptions) generated from this payment link.
    ///
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Settings related to subscription trials.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_settings: Option<UpdatePaymentLinkSubscriptionDataTrialSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkAfterCompletionHostedConfirmation {
    /// A custom message to display to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkAfterCompletionRedirect {
    /// The URL the customer will be redirected to after the purchase is complete.
    ///
    /// You can embed `{CHECKOUT_SESSION_ID}` into the URL to have the `id` of the completed [checkout session](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-id) included.
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkAutomaticTaxLiabilityType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreement {
    /// Determines the position and visibility of the payment method reuse agreement in the UI.
    ///
    /// When set to `auto`, Stripe's defaults will be used.
    /// When set to `hidden`, the payment method reuse agreement text will always be hidden in the UI.
    pub position: CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomFieldsDropdown {
    /// The options available for the customer to select.
    ///
    /// Up to 200 options allowed.
    pub options: Vec<CreatePaymentLinkCustomFieldsDropdownOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: String,

    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkCustomFieldsLabelType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomTextAfterSubmit {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomTextShippingAddress {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomTextSubmit {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomTextTermsOfServiceAcceptance {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceData {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,

    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreatePaymentLinkInvoiceCreationInvoiceDataCustomFields>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreatePaymentLinkInvoiceCreationInvoiceDataIssuer>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative Integer.
    pub enabled: bool,

    /// The maximum quantity the customer can purchase.
    ///
    /// By default this value is 99.
    /// You can specify a value up to 999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    /// The minimum quantity the customer can purchase.
    ///
    /// By default this value is 0.
    /// If there is only one item in the cart then that item's quantity cannot go down to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkRestrictionsCompletedSessions {
    /// The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met.
    pub limit: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkSubscriptionDataInvoiceSettings {
    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkSubscriptionDataTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: CreatePaymentLinkSubscriptionDataTrialSettingsEndBehavior,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkAfterCompletionHostedConfirmation {
    /// A custom message to display to the customer after the purchase is complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkAfterCompletionRedirect {
    /// The URL the customer will be redirected to after the purchase is complete.
    ///
    /// You can embed `{CHECKOUT_SESSION_ID}` into the URL to have the `id` of the completed [checkout session](https://stripe.com/docs/api/checkout/sessions/object#checkout_session_object-id) included.
    pub url: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkAutomaticTaxLiabilityType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomFieldsDropdown {
    /// The options available for the customer to select.
    ///
    /// Up to 200 options allowed.
    pub options: Vec<UpdatePaymentLinkCustomFieldsDropdownOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomFieldsLabel {
    /// Custom text for the label, displayed to the customer.
    ///
    /// Up to 50 characters.
    pub custom: String,

    /// The type of the label.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkCustomFieldsLabelType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomFieldsNumeric {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomFieldsText {
    /// The maximum character length constraint for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,

    /// The minimum character length requirement for the customer's input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomTextAfterSubmit {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomTextShippingAddress {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomTextSubmit {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomTextTermsOfServiceAcceptance {
    /// Text may be up to 1200 characters in length.
    pub message: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceData {
    /// The account tax IDs associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,

    /// Default custom fields to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<UpdatePaymentLinkInvoiceCreationInvoiceDataCustomFields>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkLineItemsAdjustableQuantity {
    /// Set to true if the quantity can be adjusted to any non-negative Integer.
    pub enabled: bool,

    /// The maximum quantity the customer can purchase.
    ///
    /// By default this value is 99.
    /// You can specify a value up to 999.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,

    /// The minimum quantity the customer can purchase.
    ///
    /// By default this value is 0.
    /// If there is only one item in the cart then that item's quantity cannot go down to 0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkRestrictionsCompletedSessions {
    /// The maximum number of checkout sessions that can be completed for the `completed_sessions` restriction to be met.
    pub limit: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataInvoiceSettings {
    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataTrialSettings {
    /// Defines how the subscription should behave when the user's free trial ends.
    pub end_behavior: UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehavior,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkCustomFieldsDropdownOptions {
    /// The label for the option, displayed to the customer.
    ///
    /// Up to 100 characters.
    pub label: String,

    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    ///
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceDataCustomFields {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: String,

    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceDataIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreatePaymentLinkSubscriptionDataTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method:
        CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkCustomFieldsDropdownOptions {
    /// The label for the option, displayed to the customer.
    ///
    /// Up to 100 characters.
    pub label: String,

    /// The value for this option, not displayed to the customer, used by your integration to reconcile the option selected by the customer.
    ///
    /// Must be unique to this option, alphanumeric, and up to 100 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceDataCustomFields {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: String,

    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehavior {
    /// Indicates how the subscription should change when the trial ends if the user did not provide a payment method.
    pub missing_payment_method:
        UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod,
}

/// An enum representing the possible values of an `CreatePaymentLinkAfterCompletion`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl CreatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            CreatePaymentLinkAfterCompletionType::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkAfterCompletionType {
    fn default() -> Self {
        Self::HostedConfirmation
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkAutomaticTaxLiability`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkAutomaticTaxLiabilityType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreatePaymentLinkAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkAutomaticTaxLiabilityType::Account => "account",
            CreatePaymentLinkAutomaticTaxLiabilityType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkAutomaticTaxLiabilityType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkAutomaticTaxLiabilityType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreement`'s `position` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}

impl CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition::Auto => "auto",
            CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition::Hidden => {
                "hidden"
            }
        }
    }
}

impl AsRef<str> for CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreatePaymentLinkConsentCollectionPaymentMethodReuseAgreementPosition
{
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkConsentCollection`'s `promotions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkConsentCollectionPromotions {
    Auto,
    None,
}

impl CreatePaymentLinkConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkConsentCollectionPromotions::Auto => "auto",
            CreatePaymentLinkConsentCollectionPromotions::None => "none",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkConsentCollectionPromotions {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkConsentCollection`'s `terms_of_service` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkConsentCollectionTermsOfService {
    None,
    Required,
}

impl CreatePaymentLinkConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkConsentCollectionTermsOfService::None => "none",
            CreatePaymentLinkConsentCollectionTermsOfService::Required => "required",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkConsentCollectionTermsOfService {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkCustomFieldsLabel`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkCustomFieldsLabelType {
    Custom,
}

impl CreatePaymentLinkCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkCustomFieldsLabelType::Custom => "custom",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkCustomFieldsLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkCustomFieldsLabelType {
    fn default() -> Self {
        Self::Custom
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkCustomFields`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

impl CreatePaymentLinkCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkCustomFieldsType::Dropdown => "dropdown",
            CreatePaymentLinkCustomFieldsType::Numeric => "numeric",
            CreatePaymentLinkCustomFieldsType::Text => "text",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkCustomFieldsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkCustomFieldsType {
    fn default() -> Self {
        Self::Dropdown
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkInvoiceCreationInvoiceDataIssuer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType::Account => "account",
            CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions`'s `amount_tax_display` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::ExcludeTax => "exclude_tax",
            CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn default() -> Self {
        Self::ExcludeTax
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkPaymentIntentData`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkPaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}

impl CreatePaymentLinkPaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkPaymentIntentDataCaptureMethod::Automatic => "automatic",
            CreatePaymentLinkPaymentIntentDataCaptureMethod::AutomaticAsync => "automatic_async",
            CreatePaymentLinkPaymentIntentDataCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkPaymentIntentDataCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkPaymentIntentData`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}

impl CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkPaymentIntentDataSetupFutureUsage::OffSession => "off_session",
            CreatePaymentLinkPaymentIntentDataSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkPaymentIntentDataSetupFutureUsage {
    fn default() -> Self {
        Self::OffSession
    }
}

/// An enum representing the possible values of an `CreatePaymentLink`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkPaymentMethodTypes {
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
}

impl CreatePaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkPaymentMethodTypes::Affirm => "affirm",
            CreatePaymentLinkPaymentMethodTypes::AfterpayClearpay => "afterpay_clearpay",
            CreatePaymentLinkPaymentMethodTypes::Alipay => "alipay",
            CreatePaymentLinkPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            CreatePaymentLinkPaymentMethodTypes::BacsDebit => "bacs_debit",
            CreatePaymentLinkPaymentMethodTypes::Bancontact => "bancontact",
            CreatePaymentLinkPaymentMethodTypes::Blik => "blik",
            CreatePaymentLinkPaymentMethodTypes::Boleto => "boleto",
            CreatePaymentLinkPaymentMethodTypes::Card => "card",
            CreatePaymentLinkPaymentMethodTypes::Cashapp => "cashapp",
            CreatePaymentLinkPaymentMethodTypes::Eps => "eps",
            CreatePaymentLinkPaymentMethodTypes::Fpx => "fpx",
            CreatePaymentLinkPaymentMethodTypes::Giropay => "giropay",
            CreatePaymentLinkPaymentMethodTypes::Grabpay => "grabpay",
            CreatePaymentLinkPaymentMethodTypes::Ideal => "ideal",
            CreatePaymentLinkPaymentMethodTypes::Klarna => "klarna",
            CreatePaymentLinkPaymentMethodTypes::Konbini => "konbini",
            CreatePaymentLinkPaymentMethodTypes::Link => "link",
            CreatePaymentLinkPaymentMethodTypes::Oxxo => "oxxo",
            CreatePaymentLinkPaymentMethodTypes::P24 => "p24",
            CreatePaymentLinkPaymentMethodTypes::Paynow => "paynow",
            CreatePaymentLinkPaymentMethodTypes::Paypal => "paypal",
            CreatePaymentLinkPaymentMethodTypes::Pix => "pix",
            CreatePaymentLinkPaymentMethodTypes::Promptpay => "promptpay",
            CreatePaymentLinkPaymentMethodTypes::SepaDebit => "sepa_debit",
            CreatePaymentLinkPaymentMethodTypes::Sofort => "sofort",
            CreatePaymentLinkPaymentMethodTypes::Swish => "swish",
            CreatePaymentLinkPaymentMethodTypes::UsBankAccount => "us_bank_account",
            CreatePaymentLinkPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkPaymentMethodTypes {
    fn default() -> Self {
        Self::Affirm
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TA")]
    Ta,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "XK")]
    Xk,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(rename = "ZZ")]
    Zz,
}

impl CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ac => "AC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ad => "AD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ae => "AE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Af => "AF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ag => "AG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ai => "AI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Al => "AL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Am => "AM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ao => "AO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Aq => "AQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ar => "AR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::At => "AT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Au => "AU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Aw => "AW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ax => "AX",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Az => "AZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ba => "BA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bb => "BB",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bd => "BD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Be => "BE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bf => "BF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bg => "BG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bh => "BH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bi => "BI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bj => "BJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bl => "BL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bm => "BM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bn => "BN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bo => "BO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bq => "BQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Br => "BR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bs => "BS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bt => "BT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bv => "BV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bw => "BW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::By => "BY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Bz => "BZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ca => "CA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cd => "CD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cf => "CF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cg => "CG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ch => "CH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ci => "CI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ck => "CK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cl => "CL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cm => "CM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cn => "CN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Co => "CO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cr => "CR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cv => "CV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cw => "CW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cy => "CY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Cz => "CZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::De => "DE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Dj => "DJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Dk => "DK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Dm => "DM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Do => "DO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Dz => "DZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ec => "EC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ee => "EE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Eg => "EG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Eh => "EH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Er => "ER",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Es => "ES",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Et => "ET",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fi => "FI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fj => "FJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fk => "FK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fo => "FO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Fr => "FR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ga => "GA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gb => "GB",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gd => "GD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ge => "GE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gf => "GF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gg => "GG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gh => "GH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gi => "GI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gl => "GL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gm => "GM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gn => "GN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gp => "GP",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gq => "GQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gr => "GR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gs => "GS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gt => "GT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gu => "GU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gw => "GW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Gy => "GY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Hk => "HK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Hn => "HN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Hr => "HR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ht => "HT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Hu => "HU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Id => "ID",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ie => "IE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Il => "IL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Im => "IM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::In => "IN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Io => "IO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Iq => "IQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Is => "IS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::It => "IT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Je => "JE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Jm => "JM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Jo => "JO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Jp => "JP",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ke => "KE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kg => "KG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kh => "KH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ki => "KI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Km => "KM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kn => "KN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kr => "KR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kw => "KW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ky => "KY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Kz => "KZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::La => "LA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lb => "LB",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lc => "LC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Li => "LI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lk => "LK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lr => "LR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ls => "LS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lt => "LT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lu => "LU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Lv => "LV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ly => "LY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ma => "MA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mc => "MC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Md => "MD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Me => "ME",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mf => "MF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mg => "MG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mk => "MK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ml => "ML",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mm => "MM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mn => "MN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mo => "MO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mq => "MQ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mr => "MR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ms => "MS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mt => "MT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mu => "MU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mv => "MV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mw => "MW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mx => "MX",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::My => "MY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Mz => "MZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Na => "NA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nc => "NC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ne => "NE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ng => "NG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ni => "NI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nl => "NL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::No => "NO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Np => "NP",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nr => "NR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nu => "NU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Nz => "NZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Om => "OM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pa => "PA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pe => "PE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pf => "PF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pg => "PG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ph => "PH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pk => "PK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pl => "PL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pm => "PM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pn => "PN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pr => "PR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ps => "PS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Pt => "PT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Py => "PY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Qa => "QA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Re => "RE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ro => "RO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Rs => "RS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ru => "RU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Rw => "RW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sa => "SA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sb => "SB",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sc => "SC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Se => "SE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sg => "SG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sh => "SH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Si => "SI",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sj => "SJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sk => "SK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sl => "SL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sm => "SM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sn => "SN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::So => "SO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sr => "SR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ss => "SS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::St => "ST",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sv => "SV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sx => "SX",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Sz => "SZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ta => "TA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tc => "TC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Td => "TD",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tf => "TF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tg => "TG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Th => "TH",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tj => "TJ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tk => "TK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tl => "TL",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tm => "TM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tn => "TN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::To => "TO",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tr => "TR",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tt => "TT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tv => "TV",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tw => "TW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Tz => "TZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ua => "UA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ug => "UG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Us => "US",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Uy => "UY",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Uz => "UZ",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Va => "VA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Vc => "VC",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ve => "VE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Vg => "VG",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Vn => "VN",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Vu => "VU",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Wf => "WF",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ws => "WS",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Xk => "XK",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Ye => "YE",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Yt => "YT",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Za => "ZA",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Zm => "ZM",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Zw => "ZW",
            CreatePaymentLinkShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn default() -> Self {
        Self::Ac
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType::Account => "account",
            CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreatePaymentLinkSubscriptionDataTrialSettingsEndBehavior`'s `missing_payment_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => "create_invoice",
            CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Pause => "pause",
        }
    }
}

impl AsRef<str> for CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn default() -> Self {
        Self::Cancel
    }
}

/// An enum representing the possible values of an `PaymentLink`'s `billing_address_collection` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkBillingAddressCollection {
    Auto,
    Required,
}

impl PaymentLinkBillingAddressCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinkBillingAddressCollection::Auto => "auto",
            PaymentLinkBillingAddressCollection::Required => "required",
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
impl std::default::Default for PaymentLinkBillingAddressCollection {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `PaymentLink`'s `customer_creation` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkCustomerCreation {
    Always,
    IfRequired,
}

impl PaymentLinkCustomerCreation {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinkCustomerCreation::Always => "always",
            PaymentLinkCustomerCreation::IfRequired => "if_required",
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
impl std::default::Default for PaymentLinkCustomerCreation {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `PaymentLink`'s `payment_method_collection` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinkPaymentMethodCollection {
    Always,
    IfRequired,
}

impl PaymentLinkPaymentMethodCollection {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinkPaymentMethodCollection::Always => "always",
            PaymentLinkPaymentMethodCollection::IfRequired => "if_required",
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
impl std::default::Default for PaymentLinkPaymentMethodCollection {
    fn default() -> Self {
        Self::Always
    }
}

/// An enum representing the possible values of an `PaymentLink`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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
}

impl PaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinkPaymentMethodTypes::Affirm => "affirm",
            PaymentLinkPaymentMethodTypes::AfterpayClearpay => "afterpay_clearpay",
            PaymentLinkPaymentMethodTypes::Alipay => "alipay",
            PaymentLinkPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            PaymentLinkPaymentMethodTypes::BacsDebit => "bacs_debit",
            PaymentLinkPaymentMethodTypes::Bancontact => "bancontact",
            PaymentLinkPaymentMethodTypes::Blik => "blik",
            PaymentLinkPaymentMethodTypes::Boleto => "boleto",
            PaymentLinkPaymentMethodTypes::Card => "card",
            PaymentLinkPaymentMethodTypes::Cashapp => "cashapp",
            PaymentLinkPaymentMethodTypes::Eps => "eps",
            PaymentLinkPaymentMethodTypes::Fpx => "fpx",
            PaymentLinkPaymentMethodTypes::Giropay => "giropay",
            PaymentLinkPaymentMethodTypes::Grabpay => "grabpay",
            PaymentLinkPaymentMethodTypes::Ideal => "ideal",
            PaymentLinkPaymentMethodTypes::Klarna => "klarna",
            PaymentLinkPaymentMethodTypes::Konbini => "konbini",
            PaymentLinkPaymentMethodTypes::Link => "link",
            PaymentLinkPaymentMethodTypes::Oxxo => "oxxo",
            PaymentLinkPaymentMethodTypes::P24 => "p24",
            PaymentLinkPaymentMethodTypes::Paynow => "paynow",
            PaymentLinkPaymentMethodTypes::Paypal => "paypal",
            PaymentLinkPaymentMethodTypes::Pix => "pix",
            PaymentLinkPaymentMethodTypes::Promptpay => "promptpay",
            PaymentLinkPaymentMethodTypes::SepaDebit => "sepa_debit",
            PaymentLinkPaymentMethodTypes::Sofort => "sofort",
            PaymentLinkPaymentMethodTypes::Swish => "swish",
            PaymentLinkPaymentMethodTypes::UsBankAccount => "us_bank_account",
            PaymentLinkPaymentMethodTypes::WechatPay => "wechat_pay",
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
impl std::default::Default for PaymentLinkPaymentMethodTypes {
    fn default() -> Self {
        Self::Affirm
    }
}

/// An enum representing the possible values of an `PaymentLink`'s `submit_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
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
            PaymentLinkSubmitType::Auto => "auto",
            PaymentLinkSubmitType::Book => "book",
            PaymentLinkSubmitType::Donate => "donate",
            PaymentLinkSubmitType::Pay => "pay",
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
impl std::default::Default for PaymentLinkSubmitType {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `PaymentLinksResourceAfterCompletion`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl PaymentLinksResourceAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourceAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            PaymentLinksResourceAfterCompletionType::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for PaymentLinksResourceAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourceAfterCompletionType {
    fn default() -> Self {
        Self::HostedConfirmation
    }
}

/// An enum representing the possible values of an `PaymentLinksResourceConsentCollection`'s `promotions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceConsentCollectionPromotions {
    Auto,
    None,
}

impl PaymentLinksResourceConsentCollectionPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourceConsentCollectionPromotions::Auto => "auto",
            PaymentLinksResourceConsentCollectionPromotions::None => "none",
        }
    }
}

impl AsRef<str> for PaymentLinksResourceConsentCollectionPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceConsentCollectionPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourceConsentCollectionPromotions {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `PaymentLinksResourceConsentCollection`'s `terms_of_service` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceConsentCollectionTermsOfService {
    None,
    Required,
}

impl PaymentLinksResourceConsentCollectionTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourceConsentCollectionTermsOfService::None => "none",
            PaymentLinksResourceConsentCollectionTermsOfService::Required => "required",
        }
    }
}

impl AsRef<str> for PaymentLinksResourceConsentCollectionTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceConsentCollectionTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourceConsentCollectionTermsOfService {
    fn default() -> Self {
        Self::None
    }
}

/// An enum representing the possible values of an `PaymentLinksResourceCustomFieldsLabel`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceCustomFieldsLabelType {
    Custom,
}

impl PaymentLinksResourceCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourceCustomFieldsLabelType::Custom => "custom",
        }
    }
}

impl AsRef<str> for PaymentLinksResourceCustomFieldsLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourceCustomFieldsLabelType {
    fn default() -> Self {
        Self::Custom
    }
}

/// An enum representing the possible values of an `PaymentLinksResourceCustomFields`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

impl PaymentLinksResourceCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourceCustomFieldsType::Dropdown => "dropdown",
            PaymentLinksResourceCustomFieldsType::Numeric => "numeric",
            PaymentLinksResourceCustomFieldsType::Text => "text",
        }
    }
}

impl AsRef<str> for PaymentLinksResourceCustomFieldsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourceCustomFieldsType {
    fn default() -> Self {
        Self::Dropdown
    }
}

/// An enum representing the possible values of an `PaymentLinksResourcePaymentIntentData`'s `capture_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourcePaymentIntentDataCaptureMethod {
    Automatic,
    AutomaticAsync,
    Manual,
}

impl PaymentLinksResourcePaymentIntentDataCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourcePaymentIntentDataCaptureMethod::Automatic => "automatic",
            PaymentLinksResourcePaymentIntentDataCaptureMethod::AutomaticAsync => "automatic_async",
            PaymentLinksResourcePaymentIntentDataCaptureMethod::Manual => "manual",
        }
    }
}

impl AsRef<str> for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourcePaymentIntentDataCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

/// An enum representing the possible values of an `PaymentLinksResourcePaymentIntentData`'s `setup_future_usage` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    OffSession,
    OnSession,
}

impl PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourcePaymentIntentDataSetupFutureUsage::OffSession => "off_session",
            PaymentLinksResourcePaymentIntentDataSetupFutureUsage::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourcePaymentIntentDataSetupFutureUsage {
    fn default() -> Self {
        Self::OffSession
    }
}

/// An enum representing the possible values of an `PaymentLinksResourcePaymentMethodReuseAgreement`'s `position` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    Auto,
    Hidden,
}

impl PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourcePaymentMethodReuseAgreementPosition::Auto => "auto",
            PaymentLinksResourcePaymentMethodReuseAgreementPosition::Hidden => "hidden",
        }
    }
}

impl AsRef<str> for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourcePaymentMethodReuseAgreementPosition {
    fn default() -> Self {
        Self::Auto
    }
}

/// An enum representing the possible values of an `PaymentLinksResourceShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceShippingAddressCollectionAllowedCountries {
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TA")]
    Ta,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "XK")]
    Xk,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(rename = "ZZ")]
    Zz,
}

impl PaymentLinksResourceShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ac => "AC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ad => "AD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ae => "AE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Af => "AF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ag => "AG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ai => "AI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Al => "AL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Am => "AM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ao => "AO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Aq => "AQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ar => "AR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::At => "AT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Au => "AU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Aw => "AW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ax => "AX",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Az => "AZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ba => "BA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bb => "BB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bd => "BD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Be => "BE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bf => "BF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bg => "BG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bh => "BH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bi => "BI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bj => "BJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bl => "BL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bm => "BM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bn => "BN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bo => "BO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bq => "BQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Br => "BR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bs => "BS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bt => "BT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bv => "BV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bw => "BW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::By => "BY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Bz => "BZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ca => "CA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cd => "CD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cf => "CF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cg => "CG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ch => "CH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ci => "CI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ck => "CK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cl => "CL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cm => "CM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cn => "CN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Co => "CO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cr => "CR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cv => "CV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cw => "CW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cy => "CY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Cz => "CZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::De => "DE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Dj => "DJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Dk => "DK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Dm => "DM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Do => "DO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Dz => "DZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ec => "EC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ee => "EE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Eg => "EG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Eh => "EH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Er => "ER",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Es => "ES",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Et => "ET",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fi => "FI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fj => "FJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fk => "FK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fo => "FO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Fr => "FR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ga => "GA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gb => "GB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gd => "GD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ge => "GE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gf => "GF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gg => "GG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gh => "GH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gi => "GI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gl => "GL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gm => "GM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gn => "GN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gp => "GP",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gq => "GQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gr => "GR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gs => "GS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gt => "GT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gu => "GU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gw => "GW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Gy => "GY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Hk => "HK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Hn => "HN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Hr => "HR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ht => "HT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Hu => "HU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Id => "ID",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ie => "IE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Il => "IL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Im => "IM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::In => "IN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Io => "IO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Iq => "IQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Is => "IS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::It => "IT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Je => "JE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Jm => "JM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Jo => "JO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Jp => "JP",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ke => "KE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kg => "KG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kh => "KH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ki => "KI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Km => "KM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kn => "KN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kr => "KR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kw => "KW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ky => "KY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Kz => "KZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::La => "LA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lb => "LB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lc => "LC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Li => "LI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lk => "LK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lr => "LR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ls => "LS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lt => "LT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lu => "LU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Lv => "LV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ly => "LY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ma => "MA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mc => "MC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Md => "MD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Me => "ME",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mf => "MF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mg => "MG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mk => "MK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ml => "ML",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mm => "MM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mn => "MN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mo => "MO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mq => "MQ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mr => "MR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ms => "MS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mt => "MT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mu => "MU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mv => "MV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mw => "MW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mx => "MX",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::My => "MY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Mz => "MZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Na => "NA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nc => "NC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ne => "NE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ng => "NG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ni => "NI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nl => "NL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::No => "NO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Np => "NP",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nr => "NR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nu => "NU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Nz => "NZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Om => "OM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pa => "PA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pe => "PE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pf => "PF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pg => "PG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ph => "PH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pk => "PK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pl => "PL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pm => "PM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pn => "PN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pr => "PR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ps => "PS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Pt => "PT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Py => "PY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Qa => "QA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Re => "RE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ro => "RO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Rs => "RS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ru => "RU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Rw => "RW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sa => "SA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sb => "SB",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sc => "SC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Se => "SE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sg => "SG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sh => "SH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Si => "SI",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sj => "SJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sk => "SK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sl => "SL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sm => "SM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sn => "SN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::So => "SO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sr => "SR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ss => "SS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::St => "ST",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sv => "SV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sx => "SX",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Sz => "SZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ta => "TA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tc => "TC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Td => "TD",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tf => "TF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tg => "TG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Th => "TH",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tj => "TJ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tk => "TK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tl => "TL",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tm => "TM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tn => "TN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::To => "TO",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tr => "TR",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tt => "TT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tv => "TV",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tw => "TW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Tz => "TZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ua => "UA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ug => "UG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Us => "US",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Uy => "UY",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Uz => "UZ",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Va => "VA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Vc => "VC",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ve => "VE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Vg => "VG",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Vn => "VN",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Vu => "VU",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Wf => "WF",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ws => "WS",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Xk => "XK",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Ye => "YE",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Yt => "YT",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Za => "ZA",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Zm => "ZM",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Zw => "ZW",
            PaymentLinksResourceShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for PaymentLinksResourceShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for PaymentLinksResourceShippingAddressCollectionAllowedCountries {
    fn default() -> Self {
        Self::Ac
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkAfterCompletion`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl UpdatePaymentLinkAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkAfterCompletionType::HostedConfirmation => "hosted_confirmation",
            UpdatePaymentLinkAfterCompletionType::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkAfterCompletionType {
    fn default() -> Self {
        Self::HostedConfirmation
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkAutomaticTaxLiability`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkAutomaticTaxLiabilityType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl UpdatePaymentLinkAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkAutomaticTaxLiabilityType::Account => "account",
            UpdatePaymentLinkAutomaticTaxLiabilityType::Self_ => "self",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkAutomaticTaxLiabilityType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkAutomaticTaxLiabilityType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkCustomFieldsLabel`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkCustomFieldsLabelType {
    Custom,
}

impl UpdatePaymentLinkCustomFieldsLabelType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkCustomFieldsLabelType::Custom => "custom",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkCustomFieldsLabelType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkCustomFieldsLabelType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkCustomFieldsLabelType {
    fn default() -> Self {
        Self::Custom
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkCustomFields`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkCustomFieldsType {
    Dropdown,
    Numeric,
    Text,
}

impl UpdatePaymentLinkCustomFieldsType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkCustomFieldsType::Dropdown => "dropdown",
            UpdatePaymentLinkCustomFieldsType::Numeric => "numeric",
            UpdatePaymentLinkCustomFieldsType::Text => "text",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkCustomFieldsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkCustomFieldsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkCustomFieldsType {
    fn default() -> Self {
        Self::Dropdown
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkInvoiceCreationInvoiceDataIssuer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType::Account => "account",
            UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkInvoiceCreationInvoiceDataIssuerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptions`'s `amount_tax_display` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::ExcludeTax => "exclude_tax",
            UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdatePaymentLinkInvoiceCreationInvoiceDataRenderingOptionsAmountTaxDisplay
{
    fn default() -> Self {
        Self::ExcludeTax
    }
}

/// An enum representing the possible values of an `UpdatePaymentLink`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkPaymentMethodTypes {
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
}

impl UpdatePaymentLinkPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkPaymentMethodTypes::Affirm => "affirm",
            UpdatePaymentLinkPaymentMethodTypes::AfterpayClearpay => "afterpay_clearpay",
            UpdatePaymentLinkPaymentMethodTypes::Alipay => "alipay",
            UpdatePaymentLinkPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            UpdatePaymentLinkPaymentMethodTypes::BacsDebit => "bacs_debit",
            UpdatePaymentLinkPaymentMethodTypes::Bancontact => "bancontact",
            UpdatePaymentLinkPaymentMethodTypes::Blik => "blik",
            UpdatePaymentLinkPaymentMethodTypes::Boleto => "boleto",
            UpdatePaymentLinkPaymentMethodTypes::Card => "card",
            UpdatePaymentLinkPaymentMethodTypes::Cashapp => "cashapp",
            UpdatePaymentLinkPaymentMethodTypes::Eps => "eps",
            UpdatePaymentLinkPaymentMethodTypes::Fpx => "fpx",
            UpdatePaymentLinkPaymentMethodTypes::Giropay => "giropay",
            UpdatePaymentLinkPaymentMethodTypes::Grabpay => "grabpay",
            UpdatePaymentLinkPaymentMethodTypes::Ideal => "ideal",
            UpdatePaymentLinkPaymentMethodTypes::Klarna => "klarna",
            UpdatePaymentLinkPaymentMethodTypes::Konbini => "konbini",
            UpdatePaymentLinkPaymentMethodTypes::Link => "link",
            UpdatePaymentLinkPaymentMethodTypes::Oxxo => "oxxo",
            UpdatePaymentLinkPaymentMethodTypes::P24 => "p24",
            UpdatePaymentLinkPaymentMethodTypes::Paynow => "paynow",
            UpdatePaymentLinkPaymentMethodTypes::Paypal => "paypal",
            UpdatePaymentLinkPaymentMethodTypes::Pix => "pix",
            UpdatePaymentLinkPaymentMethodTypes::Promptpay => "promptpay",
            UpdatePaymentLinkPaymentMethodTypes::SepaDebit => "sepa_debit",
            UpdatePaymentLinkPaymentMethodTypes::Sofort => "sofort",
            UpdatePaymentLinkPaymentMethodTypes::Swish => "swish",
            UpdatePaymentLinkPaymentMethodTypes::UsBankAccount => "us_bank_account",
            UpdatePaymentLinkPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkPaymentMethodTypes {
    fn default() -> Self {
        Self::Affirm
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkShippingAddressCollection`'s `allowed_countries` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TA")]
    Ta,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "XK")]
    Xk,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(rename = "ZZ")]
    Zz,
}

impl UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ac => "AC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ad => "AD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ae => "AE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Af => "AF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ag => "AG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ai => "AI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Al => "AL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Am => "AM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ao => "AO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Aq => "AQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ar => "AR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::At => "AT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Au => "AU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Aw => "AW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ax => "AX",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Az => "AZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ba => "BA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bb => "BB",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bd => "BD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Be => "BE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bf => "BF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bg => "BG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bh => "BH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bi => "BI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bj => "BJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bl => "BL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bm => "BM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bn => "BN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bo => "BO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bq => "BQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Br => "BR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bs => "BS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bt => "BT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bv => "BV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bw => "BW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::By => "BY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Bz => "BZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ca => "CA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cd => "CD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cf => "CF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cg => "CG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ch => "CH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ci => "CI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ck => "CK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cl => "CL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cm => "CM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cn => "CN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Co => "CO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cr => "CR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cv => "CV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cw => "CW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cy => "CY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Cz => "CZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::De => "DE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Dj => "DJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Dk => "DK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Dm => "DM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Do => "DO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Dz => "DZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ec => "EC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ee => "EE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Eg => "EG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Eh => "EH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Er => "ER",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Es => "ES",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Et => "ET",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fi => "FI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fj => "FJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fk => "FK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fo => "FO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Fr => "FR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ga => "GA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gb => "GB",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gd => "GD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ge => "GE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gf => "GF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gg => "GG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gh => "GH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gi => "GI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gl => "GL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gm => "GM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gn => "GN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gp => "GP",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gq => "GQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gr => "GR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gs => "GS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gt => "GT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gu => "GU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gw => "GW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Gy => "GY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Hk => "HK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Hn => "HN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Hr => "HR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ht => "HT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Hu => "HU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Id => "ID",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ie => "IE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Il => "IL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Im => "IM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::In => "IN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Io => "IO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Iq => "IQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Is => "IS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::It => "IT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Je => "JE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Jm => "JM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Jo => "JO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Jp => "JP",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ke => "KE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kg => "KG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kh => "KH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ki => "KI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Km => "KM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kn => "KN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kr => "KR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kw => "KW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ky => "KY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Kz => "KZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::La => "LA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lb => "LB",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lc => "LC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Li => "LI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lk => "LK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lr => "LR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ls => "LS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lt => "LT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lu => "LU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Lv => "LV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ly => "LY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ma => "MA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mc => "MC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Md => "MD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Me => "ME",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mf => "MF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mg => "MG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mk => "MK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ml => "ML",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mm => "MM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mn => "MN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mo => "MO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mq => "MQ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mr => "MR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ms => "MS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mt => "MT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mu => "MU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mv => "MV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mw => "MW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mx => "MX",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::My => "MY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Mz => "MZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Na => "NA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nc => "NC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ne => "NE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ng => "NG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ni => "NI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nl => "NL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::No => "NO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Np => "NP",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nr => "NR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nu => "NU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Nz => "NZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Om => "OM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pa => "PA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pe => "PE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pf => "PF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pg => "PG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ph => "PH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pk => "PK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pl => "PL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pm => "PM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pn => "PN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pr => "PR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ps => "PS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Pt => "PT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Py => "PY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Qa => "QA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Re => "RE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ro => "RO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Rs => "RS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ru => "RU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Rw => "RW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sa => "SA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sb => "SB",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sc => "SC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Se => "SE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sg => "SG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sh => "SH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Si => "SI",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sj => "SJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sk => "SK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sl => "SL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sm => "SM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sn => "SN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::So => "SO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sr => "SR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ss => "SS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::St => "ST",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sv => "SV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sx => "SX",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Sz => "SZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ta => "TA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tc => "TC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Td => "TD",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tf => "TF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tg => "TG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Th => "TH",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tj => "TJ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tk => "TK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tl => "TL",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tm => "TM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tn => "TN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::To => "TO",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tr => "TR",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tt => "TT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tv => "TV",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tw => "TW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Tz => "TZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ua => "UA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ug => "UG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Us => "US",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Uy => "UY",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Uz => "UZ",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Va => "VA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Vc => "VC",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ve => "VE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Vg => "VG",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Vn => "VN",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Vu => "VU",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Wf => "WF",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ws => "WS",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Xk => "XK",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Ye => "YE",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Yt => "YT",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Za => "ZA",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Zm => "ZM",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Zw => "ZW",
            UpdatePaymentLinkShippingAddressCollectionAllowedCountries::Zz => "ZZ",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkShippingAddressCollectionAllowedCountries {
    fn default() -> Self {
        Self::Ac
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType::Account => "account",
            UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for UpdatePaymentLinkSubscriptionDataInvoiceSettingsIssuerType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehavior`'s `missing_payment_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    Cancel,
    CreateInvoice,
    Pause,
}

impl UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Cancel => "cancel",
            UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::CreateInvoice => "create_invoice",
            UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod::Pause => "pause",
        }
    }
}

impl AsRef<str> for UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for UpdatePaymentLinkSubscriptionDataTrialSettingsEndBehaviorMissingPaymentMethod
{
    fn default() -> Self {
        Self::Cancel
    }
}
