// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::config::{Client, Response};
use crate::ids::{CustomerId, SubscriptionId};
use crate::params::{Deleted, Expand, List, Metadata, Object, RangeQuery, Timestamp};
use crate::Invoice;

impl Invoice {
    /// You can list all invoices, or list the invoices for a specific customer.
    ///
    /// The invoices are returned sorted by creation date, with the most recently created invoices appearing first.
    pub fn list(client: &Client, params: ListInvoices<'_>) -> Response<List<Invoice>> {
        client.get_query("/invoices", &params)
    }

    /// This endpoint creates a draft invoice for a given customer.
    ///
    /// The draft invoice created pulls in all pending invoice items on that customer, including prorations.
    /// The invoice remains a draft until you [finalize](https://stripe.com/docs/api#finalize_invoice) the invoice, which allows you to [pay](https://stripe.com/docs/api#pay_invoice) or [send](https://stripe.com/docs/api#send_invoice) the invoice to your customers.
    pub fn create(client: &Client, params: CreateInvoice<'_>) -> Response<Invoice> {
        client.post_form("/invoices", &params)
    }

    /// Retrieves the invoice with the given ID.
    pub fn retrieve(client: &Client, id: &InvoiceId, expand: &[&str]) -> Response<Invoice> {
        client.get_query(&format!("/invoices/{}", id), &Expand { expand })
    }

    /// Permanently deletes a one-off invoice draft.
    ///
    /// This cannot be undone.
    /// Attempts to delete invoices that are no longer in a draft state will fail; once an invoice has been finalized or if an invoice is for a subscription, it must be [voided](https://stripe.com/docs/api#void_invoice).
    pub fn delete(client: &Client, id: &InvoiceId) -> Response<Deleted<InvoiceId>> {
        client.delete(&format!("/invoices/{}", id))
    }
}

impl Object for Invoice {
    type Id = InvoiceId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "invoice"
    }
}

// written at 597
/// The parameters for `Invoice::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateInvoice<'a> {
    /// The account tax IDs associated with the invoice.
    ///
    /// Only editable when the invoice is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Box<Vec<String>>>,

    /// A fee in %s that will be applied to the invoice and transferred to the application owner's Stripe account.
    ///
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Controls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice.
    ///
    /// When `false`, the invoice's state will not automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    /// Settings for automatic tax lookup for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<Box<CreateInvoiceAutomaticTax>>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// A list of up to 4 custom fields to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Box<Vec<CreateInvoiceCustomFields>>>,

    /// The ID of the customer who will be billed.
    pub customer: CustomerId,

    /// The number of days from when the invoice is created until it is due.
    ///
    /// Valid only for invoices where `collection_method=send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the invoice.
    ///
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<&'a str>,

    /// ID of the default payment source for the invoice.
    ///
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<&'a str>,

    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Box<Vec<String>>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// The coupons to redeem into discounts for the invoice.
    ///
    /// If not specified, inherits the discount from the invoice's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Box<Vec<CreateInvoiceDiscounts>>>,

    /// The date on which payment for this invoice is due.
    ///
    /// Valid only for invoices where `collection_method=send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Timestamp>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Footer to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The account (if any) for which the funds of the invoice payment are intended.
    ///
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,

    /// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<Box<CreateInvoicePaymentSettings>>,

    /// Extra information about a charge for the customer's credit card statement.
    ///
    /// It must contain at least one letter.
    /// If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,

    /// The ID of the subscription to invoice, if any.
    ///
    /// If not set, the created invoice will include all pending invoice items for the customer.
    /// If set, the created invoice will only include pending invoice items for that subscription and pending invoice items not associated with any subscription.
    /// The subscription's billing cycle and regular subscription events won't be affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,

    /// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<Box<CreateInvoiceTransferData>>,
}

impl<'a> CreateInvoice<'a> {
    pub fn new(customer: CustomerId) -> Self {
        CreateInvoice {
            account_tax_ids: Default::default(),
            application_fee_amount: Default::default(),
            auto_advance: Default::default(),
            automatic_tax: Default::default(),
            collection_method: Default::default(),
            custom_fields: Default::default(),
            customer,
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            description: Default::default(),
            discounts: Default::default(),
            due_date: Default::default(),
            expand: Default::default(),
            footer: Default::default(),
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            payment_settings: Default::default(),
            statement_descriptor: Default::default(),
            subscription: Default::default(),
            transfer_data: Default::default(),
        }
    }
}

// written at 597
/// The parameters for `Invoice::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListInvoices<'a> {
    /// The collection method of the invoice to retrieve.
    ///
    /// Either `charge_automatically` or `send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return invoices for the customer specified by this customer ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<RangeQuery<Timestamp>>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<InvoiceId>,

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
    pub starting_after: Option<InvoiceId>,

    /// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
    ///
    /// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,

    /// Only return invoices for the subscription specified by this subscription ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,
}

impl<'a> ListInvoices<'a> {
    pub fn new() -> Self {
        ListInvoices {
            collection_method: Default::default(),
            created: Default::default(),
            customer: Default::default(),
            due_date: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
            subscription: Default::default(),
        }
    }
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoiceAutomaticTax {
    pub enabled: bool,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoiceCustomFields {
    pub name: String,

    pub value: String,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoiceDiscounts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Box<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Box<String>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<Box<CreateInvoicePaymentSettingsPaymentMethodOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Box<Vec<CreateInvoicePaymentSettingsPaymentMethodTypes>>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoiceTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<i64>>,

    pub destination: String,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<Box<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<Box<CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Box<CreateInvoicePaymentSettingsPaymentMethodOptionsCard>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<Box<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<Box<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<Box<CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<Box<CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>>,
}

// written at 1030
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        Box<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
    >,
}

/// An enum representing the possible values of an `CreateInvoice`'s `collection_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl CollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CollectionMethod::ChargeAutomatically => "charge_automatically",
            CollectionMethod::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for CollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions`'s `transaction_type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Business => "business",
            CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Automatic => "automatic",
            CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Instant => "instant",
            CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact`'s `preferred_language` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::De => "de",
            CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::En => "en",
            CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Fr => "fr",
            CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage::Nl => "nl",
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => {
                "automatic"
            }
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Fpx,
    Giropay,
    Ideal,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl CreateInvoicePaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodTypes::AchCreditTransfer => {
                "ach_credit_transfer"
            }
            CreateInvoicePaymentSettingsPaymentMethodTypes::AchDebit => "ach_debit",
            CreateInvoicePaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            CreateInvoicePaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            CreateInvoicePaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Boleto => "boleto",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Card => "card",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            CreateInvoicePaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            CreateInvoicePaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoicePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `ListInvoices`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

impl InvoiceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceStatus::Draft => "draft",
            InvoiceStatus::Open => "open",
            InvoiceStatus::Paid => "paid",
            InvoiceStatus::Uncollectible => "uncollectible",
            InvoiceStatus::Void => "void",
        }
    }
}

impl AsRef<str> for InvoiceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
