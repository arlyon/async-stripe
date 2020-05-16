// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CustomerId, InvoiceId, SubscriptionId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    Address, Charge, Currency, CustomField, Customer, Discount, InvoiceLineItem, PaymentIntent,
    PaymentMethod, PaymentSource, Shipping, Subscription, TaxRate,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Invoice".
///
/// For more details see [https://stripe.com/docs/api/invoices/object](https://stripe.com/docs/api/invoices/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Invoice {
    /// Unique identifier for the object.
    #[serde(default = "InvoiceId::none")]
    pub id: InvoiceId,

    /// The country of the business associated with this invoice, most often the business creating the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_country: Option<String>,

    /// The public name of the business associated with this invoice, most often the business creating the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,

    /// Final amount due at this time for this invoice.
    ///
    /// If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0.
    /// If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account.
    /// The charge that gets generated for the invoice will be for the amount specified in `amount_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_due: Option<i64>,

    /// The amount, in %s, that was paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_paid: Option<i64>,

    /// The amount remaining, in %s, that is due.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_remaining: Option<i64>,

    /// The fee in %s that will be applied to the invoice and transferred to the application owner's Stripe account when the invoice is paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Number of payment attempts made for this invoice, from the perspective of the payment retry schedule.
    ///
    /// Any payment attempt counts as the first attempt, and subsequently only automatic retries increment the attempt count.
    /// In other words, manual payment attempts after the first attempt do not affect the retry schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_count: Option<u64>,

    /// Whether an attempt has been made to pay the invoice.
    ///
    /// An invoice is not attempted until 1 hour after the `invoice.created` webhook, for example, so you might not want to display that invoice as unpaid to your users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempted: Option<bool>,

    /// Controls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice.
    ///
    /// When `false`, the invoice's state will not automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    /// Indicates the reason why the invoice was created.
    ///
    /// `subscription_cycle` indicates an invoice created by a subscription advancing into a new period.
    /// `subscription_create` indicates an invoice created due to creating a subscription.
    /// `subscription_update` indicates an invoice created due to updating a subscription.
    /// `subscription` is set for all old invoices to indicate either a change to a subscription or a period advancement.
    /// `manual` is set for all invoices unrelated to a subscription (for example: created via the invoice editor).
    /// The `upcoming` value is reserved for simulated invoices per the upcoming invoice endpoint.
    /// `subscription_threshold` indicates an invoice created due to a billing threshold being reached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_reason: Option<InvoiceBillingReason>,

    /// ID of the latest charge generated for this invoice, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Expandable<Charge>>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// Custom fields displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,

    /// The ID of the customer who will be billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    /// The customer's address.
    ///
    /// Until the invoice is finalized, this field will equal `customer.address`.
    /// Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_address: Option<Address>,

    /// The customer's email.
    ///
    /// Until the invoice is finalized, this field will equal `customer.email`.
    /// Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,

    /// The customer's name.
    ///
    /// Until the invoice is finalized, this field will equal `customer.name`.
    /// Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,

    /// The customer's phone number.
    ///
    /// Until the invoice is finalized, this field will equal `customer.phone`.
    /// Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_phone: Option<String>,

    /// The customer's shipping information.
    ///
    /// Until the invoice is finalized, this field will equal `customer.shipping`.
    /// Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_shipping: Option<Shipping>,

    /// The customer's tax exempt status.
    ///
    /// Until the invoice is finalized, this field will equal `customer.tax_exempt`.
    /// Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tax_exempt: Option<InvoiceCustomerTaxExempt>,

    /// The customer's tax IDs.
    ///
    /// Until the invoice is finalized, this field will contain the same tax IDs as `customer.tax_ids`.
    /// Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tax_ids: Option<Vec<InvoicesResourceInvoiceTaxId>>,

    /// ID of the default payment method for the invoice.
    ///
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    /// ID of the default payment source for the invoice.
    ///
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<Expandable<PaymentSource>>,

    /// The tax rates applied to this invoice, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Describes the current discount applied to this invoice, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,

    /// The date on which payment for this invoice is due.
    ///
    /// This value will be `null` for invoices where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Timestamp>,

    /// Ending customer balance after the invoice is finalized.
    ///
    /// Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice.
    /// If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<i64>,

    /// Footer displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// The URL for the hosted invoice page, which allows customers to view and pay an invoice.
    ///
    /// If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_invoice_url: Option<String>,

    /// The link to download the PDF for the invoice.
    ///
    /// If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_pdf: Option<String>,

    /// The individual line items that make up the invoice.
    ///
    /// `lines` is sorted as follows: invoice items in reverse chronological order, followed by the subscription, if any.
    #[serde(default)]
    pub lines: List<InvoiceLineItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// The time at which payment will next be attempted.
    ///
    /// This value will be `null` for invoices where `collection_method=send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_payment_attempt: Option<Timestamp>,

    /// A unique, identifying string that appears on emails sent to the customer for this invoice.
    ///
    /// This starts with the customer's unique invoice_prefix if it is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// Whether payment was successfully collected for this invoice.
    ///
    /// An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,

    /// The PaymentIntent associated with this invoice.
    ///
    /// The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice.
    /// Note that voiding an invoice will cancel the PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// End of the usage period during which invoice items were added to this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_end: Option<Timestamp>,

    /// Start of the usage period during which invoice items were added to this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period_start: Option<Timestamp>,

    /// Total amount of all post-payment credit notes issued for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_payment_credit_notes_amount: Option<i64>,

    /// Total amount of all pre-payment credit notes issued for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_payment_credit_notes_amount: Option<i64>,

    /// This is the transaction number that appears on email receipts sent for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,

    /// Starting customer balance before the invoice is finalized.
    ///
    /// If the invoice has not been finalized yet, this will be the current customer balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_balance: Option<i64>,

    /// Extra information about an invoice for the customer's credit card statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
    ///
    /// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoiceStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<InvoicesStatusTransitions>,

    /// The subscription that this invoice was prepared for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,

    /// Only set for upcoming invoices that preview prorations.
    ///
    /// The time used to calculate prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<Timestamp>,

    /// Total of all subscriptions, invoice items, and prorations on the invoice before any discount or tax is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<i64>,

    /// The amount of tax on this invoice.
    ///
    /// This is the sum of all the tax amounts on this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<i64>,

    /// This percentage of the subtotal has been added to the total amount of the invoice, including invoice line items and discounts.
    ///
    /// This field is inherited from the subscription's `tax_percent` field, but can be changed before the invoice is paid.
    /// This field defaults to null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_reason: Option<InvoiceThresholdReason>,

    /// Total after discounts and taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    /// The aggregate amounts calculated per tax rate for all line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amounts: Option<Vec<TaxAmount>>,

    /// Invoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts have [been exhausted](https://stripe.com/docs/billing/webhooks#understand).
    ///
    /// This field tracks the time when webhooks for this invoice were successfully delivered.
    /// If the invoice had no webhooks to deliver, this will be set while the invoice is being created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhooks_delivered_at: Option<Timestamp>,
}

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
    pub fn create(client: &Client, params: CreateInvoice<'_>) -> Response<Invoice> {
        client.post_form("/invoices", &params)
    }

    /// Retrieves the invoice with the given ID.
    pub fn retrieve(client: &Client, id: &InvoiceId, expand: &[&str]) -> Response<Invoice> {
        client.get_query(&format!("/invoices/{}", id), &Expand { expand })
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceSettingCustomField {
    /// The name of the custom field.
    pub name: String,

    /// The value of the custom field.
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaxAmount {
    /// The amount, in %s, of the tax.
    pub amount: i64,

    /// Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,

    /// The tax rate that was applied to get this tax amount.
    pub tax_rate: Expandable<TaxRate>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceThresholdReason {
    /// The total invoice amount threshold boundary if it triggered the threshold invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,

    /// Indicates which line items triggered a threshold invoice.
    pub item_reasons: Vec<InvoiceItemThresholdReason>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceItemThresholdReason {
    /// The IDs of the line items that triggered the threshold invoice.
    pub line_item_ids: Vec<String>,

    /// The quantity threshold boundary that applied to the given line item.
    pub usage_gte: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoicesResourceInvoiceTaxId {
    /// The type of the tax ID, one of `eu_vat`, `br_cnpj`, `br_cpf`, `nz_gst`, `au_abn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `my_sst`, `sg_gst`, or `unknown`.
    #[serde(rename = "type")]
    pub type_: TaxIdType,

    /// The value of the tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoicesStatusTransitions {
    /// The time that the invoice draft was finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<Timestamp>,

    /// The time that the invoice was marked uncollectible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marked_uncollectible_at: Option<Timestamp>,

    /// The time that the invoice was paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_at: Option<Timestamp>,

    /// The time that the invoice was voided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<Timestamp>,
}

/// The parameters for `Invoice::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateInvoice<'a> {
    /// A fee in %s that will be applied to the invoice and transferred to the application owner's Stripe account.
    ///
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#invoices).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Controls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice.
    ///
    /// When `false`, the invoice's state will not automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// A list of up to 4 custom fields to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,

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
    pub default_tax_rates: Option<Vec<String>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

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

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

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

    /// The percent tax rate applied to the invoice, represented as a decimal number.
    ///
    /// This field has been deprecated and will be removed in a future API version, for further information view the [migration docs](https://stripe.com/docs/billing/migration/taxes) for `tax_rates`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,
}

impl<'a> CreateInvoice<'a> {
    pub fn new(customer: CustomerId) -> Self {
        CreateInvoice {
            application_fee_amount: Default::default(),
            auto_advance: Default::default(),
            collection_method: Default::default(),
            custom_fields: Default::default(),
            customer,
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            description: Default::default(),
            due_date: Default::default(),
            expand: Default::default(),
            footer: Default::default(),
            metadata: Default::default(),
            statement_descriptor: Default::default(),
            subscription: Default::default(),
            tax_percent: Default::default(),
        }
    }
}

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
    pub status: Option<InvoiceStatusFilter>,

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

/// An enum representing the possible values of an `Invoice`'s `collection_method` field.
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

/// An enum representing the possible values of an `Invoice`'s `billing_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceBillingReason {
    AutomaticPendingInvoiceItemInvoice,
    Manual,
    Subscription,
    SubscriptionCreate,
    SubscriptionCycle,
    SubscriptionThreshold,
    SubscriptionUpdate,
    Upcoming,
}

impl InvoiceBillingReason {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceBillingReason::AutomaticPendingInvoiceItemInvoice => {
                "automatic_pending_invoice_item_invoice"
            }
            InvoiceBillingReason::Manual => "manual",
            InvoiceBillingReason::Subscription => "subscription",
            InvoiceBillingReason::SubscriptionCreate => "subscription_create",
            InvoiceBillingReason::SubscriptionCycle => "subscription_cycle",
            InvoiceBillingReason::SubscriptionThreshold => "subscription_threshold",
            InvoiceBillingReason::SubscriptionUpdate => "subscription_update",
            InvoiceBillingReason::Upcoming => "upcoming",
        }
    }
}

impl AsRef<str> for InvoiceBillingReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceBillingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `Invoice`'s `customer_tax_exempt` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceCustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl InvoiceCustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceCustomerTaxExempt::Exempt => "exempt",
            InvoiceCustomerTaxExempt::None => "none",
            InvoiceCustomerTaxExempt::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for InvoiceCustomerTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceCustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `Invoice`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Deleted,
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

impl InvoiceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceStatus::Deleted => "deleted",
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

/// An enum representing the possible values of an `ListInvoices`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatusFilter {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

impl InvoiceStatusFilter {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceStatusFilter::Draft => "draft",
            InvoiceStatusFilter::Open => "open",
            InvoiceStatusFilter::Paid => "paid",
            InvoiceStatusFilter::Uncollectible => "uncollectible",
            InvoiceStatusFilter::Void => "void",
        }
    }
}

impl AsRef<str> for InvoiceStatusFilter {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceStatusFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `InvoicesResourceInvoiceTaxId`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    AuAbn,
    BrCnpj,
    BrCpf,
    CaBn,
    CaQst,
    ChVat,
    EsCif,
    EuVat,
    HkBr,
    InGst,
    JpCn,
    KrBrn,
    LiUid,
    MxRfc,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    RuInn,
    SgGst,
    SgUen,
    ThVat,
    TwVat,
    Unknown,
    UsEin,
    ZaVat,
}

impl TaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxIdType::AuAbn => "au_abn",
            TaxIdType::BrCnpj => "br_cnpj",
            TaxIdType::BrCpf => "br_cpf",
            TaxIdType::CaBn => "ca_bn",
            TaxIdType::CaQst => "ca_qst",
            TaxIdType::ChVat => "ch_vat",
            TaxIdType::EsCif => "es_cif",
            TaxIdType::EuVat => "eu_vat",
            TaxIdType::HkBr => "hk_br",
            TaxIdType::InGst => "in_gst",
            TaxIdType::JpCn => "jp_cn",
            TaxIdType::KrBrn => "kr_brn",
            TaxIdType::LiUid => "li_uid",
            TaxIdType::MxRfc => "mx_rfc",
            TaxIdType::MyItn => "my_itn",
            TaxIdType::MySst => "my_sst",
            TaxIdType::NoVat => "no_vat",
            TaxIdType::NzGst => "nz_gst",
            TaxIdType::RuInn => "ru_inn",
            TaxIdType::SgGst => "sg_gst",
            TaxIdType::SgUen => "sg_uen",
            TaxIdType::ThVat => "th_vat",
            TaxIdType::TwVat => "tw_vat",
            TaxIdType::Unknown => "unknown",
            TaxIdType::UsEin => "us_ein",
            TaxIdType::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for TaxIdType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxIdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
