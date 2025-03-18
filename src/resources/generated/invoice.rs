// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CustomerId, InvoiceId, SubscriptionId};
use crate::params::{
    CurrencyMap, Deleted, Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery,
    Timestamp,
};
use crate::resources::{
    Account, Address, ApiErrors, Application, Charge, ConnectAccountReference, Currency, Customer,
    Discount, InvoiceLineItem, InvoicePaymentMethodOptionsAcssDebit,
    InvoicePaymentMethodOptionsBancontact, InvoicePaymentMethodOptionsCustomerBalance,
    InvoicePaymentMethodOptionsKonbini, InvoicePaymentMethodOptionsUsBankAccount,
    InvoiceSettingRenderingOptions, InvoicesShippingCost, PaymentIntent, PaymentMethod,
    PaymentSource, Quote, Shipping, Subscription, TaxId, TaxRate, TestHelpersTestClock,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "Invoice".
///
/// For more details see <https://stripe.com/docs/api/invoices/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Invoice {
    /// Unique identifier for the object.
    ///
    /// This property is always present unless the invoice is an upcoming invoice.
    /// See [Retrieve an upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) for more details.
    #[serde(default = "InvoiceId::none")]
    pub id: InvoiceId,

    /// The country of the business associated with this invoice, most often the business creating the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_country: Option<String>,

    /// The public name of the business associated with this invoice, most often the business creating the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_name: Option<String>,

    /// The account tax IDs associated with the invoice.
    ///
    /// Only editable when the invoice is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<Expandable<TaxId>>>,

    /// Final amount due at this time for this invoice.
    ///
    /// If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0.
    /// If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account.
    /// The charge that gets generated for the invoice will be for the amount specified in `amount_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_due: Option<i64>,

    /// The amount, in cents (or local equivalent), that was paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_paid: Option<i64>,

    /// The difference between amount_due and amount_paid, in cents (or local equivalent).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_remaining: Option<i64>,

    /// This is the sum of all the shipping amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_shipping: Option<i64>,

    /// ID of the Connect Application that created the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<Expandable<Application>>,

    /// The fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account when the invoice is paid.
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

    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    ///
    /// If `false`, the invoice's state doesn't automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<AutomaticTax>,

    /// Indicates the reason why the invoice was created.
    ///
    /// * `manual`: Unrelated to a subscription, for example, created via the invoice editor.
    /// * `subscription`: No longer in use.
    ///
    /// Applies to subscriptions from before May 2018 where no distinction was made between updates, cycles, and thresholds. * `subscription_create`: A new subscription was created. * `subscription_cycle`: A subscription advanced into a new period. * `subscription_threshold`: A subscription reached a billing threshold. * `subscription_update`: A subscription was updated. * `upcoming`: Reserved for simulated invoices, per the upcoming invoice endpoint.
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
    ///
    /// Not populated if there are multiple discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,

    /// The discounts applied to the invoice.
    ///
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<Expandable<Discount>>>,

    /// The date on which payment for this invoice is due.
    ///
    /// This value will be `null` for invoices where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Timestamp>,

    /// The date when this invoice is in effect.
    ///
    /// Same as `finalized_at` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the invoice PDF and receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<Timestamp>,

    /// Ending customer balance after the invoice is finalized.
    ///
    /// Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice.
    /// If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_balance: Option<i64>,

    /// Footer displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// Details of the invoice that was cloned.
    ///
    /// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_invoice: Option<InvoicesFromInvoice>,

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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<ConnectAccountReference>,

    /// The error encountered during the previous attempt to finalize the invoice.
    ///
    /// This field is cleared when the invoice is successfully finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_finalization_error: Option<Box<ApiErrors>>,

    /// The ID of the most recent non-draft revision of this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<Expandable<Invoice>>,

    /// The individual line items that make up the invoice.
    ///
    /// `lines` is sorted as follows: (1) pending invoice items (including prorations) in reverse chronological order, (2) subscription items in reverse chronological order, and (3) invoice items added after invoice creation in chronological order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<List<InvoiceLineItem>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

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

    /// The account (if any) for which the funds of the invoice payment are intended.
    ///
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<Expandable<Account>>,

    /// Whether payment was successfully collected for this invoice.
    ///
    /// An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<bool>,

    /// Returns true if the invoice was manually marked paid, returns false if the invoice hasn't been paid yet or was paid on Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_out_of_band: Option<bool>,

    /// The PaymentIntent associated with this invoice.
    ///
    /// The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice.
    /// Note that voiding an invoice will cancel the PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<InvoicesPaymentSettings>,

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

    /// The quote this invoice was generated from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<Expandable<Quote>>,

    /// This is the transaction number that appears on email receipts sent for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,

    /// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<InvoicesInvoiceRendering>,

    /// This is a legacy field that will be removed soon.
    ///
    /// For details about `rendering_options`, refer to `rendering` instead.
    /// Options for invoice PDF rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<InvoiceSettingRenderingOptions>,

    /// The details of the cost of shipping, including the ShippingRate applied on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<InvoicesShippingCost>,

    /// Shipping details for the invoice.
    ///
    /// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<Shipping>,

    /// Starting customer balance before the invoice is finalized.
    ///
    /// If the invoice has not been finalized yet, this will be the current customer balance.
    /// For revision invoices, this also includes any customer balance that was applied to the original invoice.
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

    /// Details about the subscription that created this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_details: Option<SubscriptionDetailsData>,

    /// Only set for upcoming invoices that preview prorations.
    ///
    /// The time used to calculate prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<Timestamp>,

    /// Total of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusive tax is applied.
    ///
    /// Item discounts are already incorporated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<i64>,

    /// The integer amount in cents (or local equivalent) representing the subtotal of the invoice before any invoice level discount or tax is applied.
    ///
    /// Item discounts are already incorporated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_excluding_tax: Option<i64>,

    /// The amount of tax on this invoice.
    ///
    /// This is the sum of all the tax amounts on this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<i64>,

    /// ID of the test clock this invoice belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<Expandable<TestHelpersTestClock>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_reason: Option<InvoiceThresholdReason>,

    /// Total after discounts and taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,

    /// The aggregate amounts calculated per discount across all line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_discount_amounts: Option<Vec<DiscountsResourceDiscountAmount>>,

    /// The integer amount in cents (or local equivalent) representing the total amount of the invoice including all discounts but excluding all tax.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_excluding_tax: Option<i64>,

    /// The aggregate amounts calculated per tax rate for all line items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax_amounts: Option<Vec<TaxAmount>>,

    /// The account (if any) the payment will be attributed to for tax reporting, and where funds from the payment will be transferred to for the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<InvoiceTransferData>,

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
    pub fn list(client: &Client, params: &ListInvoices<'_>) -> Response<List<Invoice>> {
        client.get_query("/invoices", params)
    }

    /// This endpoint creates a draft invoice for a given customer.
    ///
    /// The invoice remains a draft until you [finalize](https://stripe.com/docs/api#finalize_invoice) the invoice, which allows you to [pay](https://stripe.com/docs/api#pay_invoice) or [send](https://stripe.com/docs/api#send_invoice) the invoice to your customers.
    pub fn create(client: &Client, params: CreateInvoice<'_>) -> Response<Invoice> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/invoices", &params)
    }

    /// Retrieves the invoice with the given ID.
    pub fn retrieve(client: &Client, id: &InvoiceId, expand: &[&str]) -> Response<Invoice> {
        client.get_query(&format!("/invoices/{}", id), Expand { expand })
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    pub liability: Option<ConnectAccountReference>,

    /// The status of the most recent automated tax calculation for this invoice.
    pub status: Option<AutomaticTaxStatus>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DiscountsResourceDiscountAmount {
    /// The amount, in cents (or local equivalent), of the discount.
    pub amount: i64,

    /// The discount that was applied to get this discount amount.
    pub discount: Expandable<Discount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceSettingCustomField {
    /// The name of the custom field.
    pub name: String,

    /// The value of the custom field.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TaxAmount {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,

    /// Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,

    /// The tax rate that was applied to get this tax amount.
    pub tax_rate: Expandable<TaxRate>,

    /// The reasoning behind this tax, for example, if the product is tax exempt.
    ///
    /// The possible values for this field may be extended as new tax rules are supported.
    pub taxability_reason: Option<TaxAmountTaxabilityReason>,

    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceThresholdReason {
    /// The total invoice amount threshold boundary if it triggered the threshold invoice.
    pub amount_gte: Option<i64>,

    /// Indicates which line items triggered a threshold invoice.
    pub item_reasons: Vec<InvoiceItemThresholdReason>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceItemThresholdReason {
    /// The IDs of the line items that triggered the threshold invoice.
    pub line_item_ids: Vec<String>,

    /// The quantity threshold boundary that applied to the given line item.
    pub usage_gte: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceTransferData {
    /// The amount in cents (or local equivalent) that will be transferred to the destination account when the invoice is paid.
    ///
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,

    /// The account where funds from the payment will be transferred to upon payment success.
    pub destination: Expandable<Account>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesFromInvoice {
    /// The relation between this invoice and the cloned invoice.
    pub action: String,

    /// The invoice that was cloned.
    pub invoice: Expandable<Invoice>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesInvoiceRendering {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    pub amount_tax_display: Option<String>,

    /// Invoice pdf rendering options.
    pub pdf: Option<InvoiceRenderingPdf>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceRenderingPdf {
    /// Page size of invoice pdf.
    ///
    /// Options include a4, letter, and auto.
    /// If set to auto, page size will be switched to a4 or letter based on customer locale.
    pub page_size: Option<InvoiceRenderingPdfPageSize>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesPaymentSettings {
    /// ID of the mandate to be used for this invoice.
    ///
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    pub default_mandate: Option<String>,

    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    pub payment_method_options: Option<InvoicesPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    pub payment_method_types: Option<Vec<InvoicesPaymentSettingsPaymentMethodTypes>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    pub acss_debit: Option<InvoicePaymentMethodOptionsAcssDebit>,

    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    pub bancontact: Option<InvoicePaymentMethodOptionsBancontact>,

    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    pub card: Option<InvoicePaymentMethodOptionsCard>,

    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    pub customer_balance: Option<InvoicePaymentMethodOptionsCustomerBalance>,

    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    pub konbini: Option<InvoicePaymentMethodOptionsKonbini>,

    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    pub us_bank_account: Option<InvoicePaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicePaymentMethodOptionsCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<InvoiceInstallmentsCard>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    pub request_three_d_secure: Option<InvoicePaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceInstallmentsCard {
    /// Whether Installments are enabled for this Invoice.
    pub enabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesResourceInvoiceTaxId {
    /// The type of the tax ID, one of `ad_nrt`, `ar_cuit`, `eu_vat`, `bo_tin`, `br_cnpj`, `br_cpf`, `cn_tin`, `co_nit`, `cr_tin`, `do_rcn`, `ec_ruc`, `eu_oss_vat`, `pe_ruc`, `ro_tin`, `rs_pib`, `sv_nit`, `uy_ruc`, `ve_rif`, `vn_tin`, `gb_vat`, `nz_gst`, `au_abn`, `au_arn`, `in_gst`, `no_vat`, `za_vat`, `ch_vat`, `mx_rfc`, `sg_uen`, `ru_inn`, `ru_kpp`, `ca_bn`, `hk_br`, `es_cif`, `tw_vat`, `th_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `li_uid`, `my_itn`, `us_ein`, `kr_brn`, `ca_qst`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `my_sst`, `sg_gst`, `ae_trn`, `cl_tin`, `sa_vat`, `id_npwp`, `my_frp`, `il_vat`, `ge_vat`, `ua_vat`, `is_vat`, `bg_uic`, `hu_tin`, `si_tin`, `ke_pin`, `tr_tin`, `eg_tin`, `ph_tin`, or `unknown`.
    #[serde(rename = "type")]
    pub type_: TaxIdType,

    /// The value of the tax ID.
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesStatusTransitions {
    /// The time that the invoice draft was finalized.
    pub finalized_at: Option<Timestamp>,

    /// The time that the invoice was marked uncollectible.
    pub marked_uncollectible_at: Option<Timestamp>,

    /// The time that the invoice was paid.
    pub paid_at: Option<Timestamp>,

    /// The time that the invoice was voided.
    pub voided_at: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct SubscriptionDetailsData {
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will reflect the metadata of the subscription at the time of invoice creation.
    ///
    /// *Note: This attribute is populated only for invoices created on or after June 29, 2023.*.
    pub metadata: Option<Metadata>,
}

/// The parameters for `Invoice::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateInvoice<'a> {
    /// The account tax IDs associated with the invoice.
    ///
    /// Only editable when the invoice is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,

    /// A fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account.
    ///
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    ///
    /// If `false`, the invoice's state doesn't automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    /// Settings for automatic tax lookup for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateInvoiceAutomaticTax>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// The currency to create this invoice in.
    ///
    /// Defaults to that of `customer` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// A list of up to 4 custom fields to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CreateInvoiceCustomFields>>,

    /// The ID of the customer who will be billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

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

    /// The coupons to redeem into discounts for the invoice.
    ///
    /// If not specified, inherits the discount from the invoice's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<CreateInvoiceDiscounts>>,

    /// The date on which payment for this invoice is due.
    ///
    /// Valid only for invoices where `collection_method=send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Timestamp>,

    /// The date when this invoice is in effect.
    ///
    /// Same as `finalized_at` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the invoice PDF and receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<Timestamp>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Footer to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,

    /// Revise an existing invoice.
    ///
    /// The new invoice will be created in `status=draft`.
    /// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_invoice: Option<CreateInvoiceFromInvoice>,

    /// The connected account that issues the invoice.
    ///
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateInvoiceIssuer>,

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
    pub payment_settings: Option<CreateInvoicePaymentSettings>,

    /// How to handle pending invoice items on invoice creation.
    ///
    /// One of `include` or `exclude`.
    /// `include` will include any pending invoice items, and will create an empty draft invoice if no pending invoice items exist.
    /// `exclude` will always create an empty invoice draft regardless if there are pending invoice items or not.
    /// Defaults to `exclude` if the parameter is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_items_behavior: Option<InvoicePendingInvoiceItemsBehavior>,

    /// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<CreateInvoiceRendering>,

    /// This is a legacy field that will be removed soon.
    ///
    /// For details about `rendering_options`, refer to `rendering` instead.
    /// Options for invoice PDF rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<CreateInvoiceRenderingOptions>,

    /// Settings for the cost of shipping for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<CreateInvoiceShippingCost>,

    /// Shipping details for the invoice.
    ///
    /// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<CreateInvoiceShippingDetails>,

    /// Extra information about a charge for the customer's credit card statement.
    ///
    /// It must contain at least one letter.
    /// If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<&'a str>,

    /// The ID of the subscription to invoice, if any.
    ///
    /// If set, the created invoice will only include pending invoice items for that subscription.
    /// The subscription's billing cycle and regular subscription events won't be affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,

    /// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateInvoiceTransferData>,
}

impl<'a> CreateInvoice<'a> {
    pub fn new() -> Self {
        CreateInvoice {
            account_tax_ids: Default::default(),
            application_fee_amount: Default::default(),
            auto_advance: Default::default(),
            automatic_tax: Default::default(),
            collection_method: Default::default(),
            currency: Default::default(),
            custom_fields: Default::default(),
            customer: Default::default(),
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            description: Default::default(),
            discounts: Default::default(),
            due_date: Default::default(),
            effective_at: Default::default(),
            expand: Default::default(),
            footer: Default::default(),
            from_invoice: Default::default(),
            issuer: Default::default(),
            metadata: Default::default(),
            on_behalf_of: Default::default(),
            payment_settings: Default::default(),
            pending_invoice_items_behavior: Default::default(),
            rendering: Default::default(),
            rendering_options: Default::default(),
            shipping_cost: Default::default(),
            shipping_details: Default::default(),
            statement_descriptor: Default::default(),
            subscription: Default::default(),
            transfer_data: Default::default(),
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
impl Paginable for ListInvoices<'_> {
    type O = Invoice;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,

    /// The account that's liable for tax.
    ///
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateInvoiceAutomaticTaxLiability>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceCustomFields {
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
pub struct CreateInvoiceDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceFromInvoice {
    /// The relation between the new invoice and the original invoice.
    ///
    /// Currently, only 'revision' is permitted.
    pub action: CreateInvoiceFromInvoiceAction,

    /// The `id` of the invoice that will be cloned.
    pub invoice: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateInvoiceIssuerType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettings {
    /// ID of the mandate to be used for this invoice.
    ///
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mandate: Option<String>,

    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<CreateInvoicePaymentSettingsPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<CreateInvoicePaymentSettingsPaymentMethodTypes>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceRendering {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<CreateInvoiceRenderingAmountTaxDisplay>,

    /// Invoice pdf rendering options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf: Option<CreateInvoiceRenderingPdf>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<CreateInvoiceRenderingOptionsAmountTaxDisplay>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceShippingCost {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,

    /// Parameters to create a new ad-hoc shipping rate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<CreateInvoiceShippingCostShippingRateData>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceShippingDetails {
    /// Shipping address.
    pub address: CreateInvoiceShippingDetailsAddress,

    /// Recipient name.
    pub name: String,

    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceTransferData {
    /// The amount that will be transferred automatically when the invoice is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateInvoiceAutomaticTaxLiabilityType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit>,

    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact>,

    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCard>,

    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalance>,

    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsKonbini>,

    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceRenderingPdf {
    /// Page size for invoice PDF.
    ///
    /// Can be set to `a4`, `letter`, or `auto`.  If set to `auto`, invoice PDF page size defaults to `a4` for customers with  Japanese locale and `letter` for customers with other locales.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<CreateInvoiceRenderingPdfPageSize>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceShippingCostShippingRateData {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<CreateInvoiceShippingCostShippingRateDataDeliveryEstimate>,

    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: String,

    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateInvoiceShippingCostShippingRateDataFixedAmount>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateInvoiceShippingCostShippingRateDataTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateInvoiceShippingCostShippingRateDataType>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceShippingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCard {
    /// Installment configuration for payments attempted on this invoice (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsKonbini {}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum>,

    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<CurrencyMap<CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this invoice.
    /// Setting to false will prevent any selected plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The selected installment plan to use for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer: Option<
        CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer,
    >,

    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, `mx_bank_transfer`, or `us_bank_transfer`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {

    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>>,

    /// List of data features that you would like to retrieve upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: u64,

    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,

    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoicePaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `BE`, `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}

/// An enum representing the possible values of an `AutomaticTax`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl AutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            AutomaticTaxStatus::Complete => "complete",
            AutomaticTaxStatus::Failed => "failed",
            AutomaticTaxStatus::RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl AsRef<str> for AutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for AutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for AutomaticTaxStatus {
    fn default() -> Self {
        Self::Complete
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
impl std::default::Default for CollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}

/// An enum representing the possible values of an `CreateInvoiceAutomaticTaxLiability`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceAutomaticTaxLiabilityType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreateInvoiceAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceAutomaticTaxLiabilityType::Account => "account",
            CreateInvoiceAutomaticTaxLiabilityType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreateInvoiceAutomaticTaxLiabilityType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateInvoiceAutomaticTaxLiabilityType {
    fn default() -> Self {
        Self::Account
    }
}

/// An enum representing the possible values of an `CreateInvoiceFromInvoice`'s `action` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceFromInvoiceAction {
    Revision,
}

impl CreateInvoiceFromInvoiceAction {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceFromInvoiceAction::Revision => "revision",
        }
    }
}

impl AsRef<str> for CreateInvoiceFromInvoiceAction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceFromInvoiceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateInvoiceFromInvoiceAction {
    fn default() -> Self {
        Self::Revision
    }
}

/// An enum representing the possible values of an `CreateInvoiceIssuer`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceIssuerType {
    Account,
    #[serde(rename = "self")]
    Self_,
}

impl CreateInvoiceIssuerType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceIssuerType::Account => "account",
            CreateInvoiceIssuerType::Self_ => "self",
        }
    }
}

impl AsRef<str> for CreateInvoiceIssuerType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateInvoiceIssuerType {
    fn default() -> Self {
        Self::Account
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
impl std::default::Default
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
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
impl std::default::Default
    for CreateInvoicePaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
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
impl std::default::Default
    for CreateInvoicePaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn default() -> Self {
        Self::De
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan`'s `interval` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval::Month => {
                "month"
            }
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn default() -> Self {
        Self::Month
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlan`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    FixedCount,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType::FixedCount => "fixed_count",
        }
    }
}

impl AsRef<str> for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn default() -> Self {
        Self::FixedCount
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Automatic => {
                "automatic"
            }
            CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure::Challenge => {
                "challenge"
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
impl std::default::Default
    for CreateInvoicePaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn default() -> Self {
        Self::Any
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections`'s `permissions` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Balances => "balances",
            CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Ownership => "ownership",
            CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::PaymentMethod => "payment_method",
            CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections`'s `prefetch` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    Balances,
    Transactions,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Balances => "balances",
            CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch::Transactions => "transactions",
        }
    }
}

impl AsRef<str>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPrefetch
{
    fn default() -> Self {
        Self::Balances
    }
}

/// An enum representing the possible values of an `CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccount`'s `verification_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::Automatic => "automatic",
            CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::Instant => "instant",
            CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateInvoicePaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
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
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    P24,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
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
            CreateInvoicePaymentSettingsPaymentMethodTypes::Cashapp => "cashapp",
            CreateInvoicePaymentSettingsPaymentMethodTypes::CustomerBalance => "customer_balance",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Eps => "eps",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Grabpay => "grabpay",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Konbini => "konbini",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Link => "link",
            CreateInvoicePaymentSettingsPaymentMethodTypes::P24 => "p24",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Paynow => "paynow",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Paypal => "paypal",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Promptpay => "promptpay",
            CreateInvoicePaymentSettingsPaymentMethodTypes::SepaCreditTransfer => {
                "sepa_credit_transfer"
            }
            CreateInvoicePaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            CreateInvoicePaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            CreateInvoicePaymentSettingsPaymentMethodTypes::UsBankAccount => "us_bank_account",
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
impl std::default::Default for CreateInvoicePaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}

/// An enum representing the possible values of an `CreateInvoiceRendering`'s `amount_tax_display` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceRenderingAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl CreateInvoiceRenderingAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceRenderingAmountTaxDisplay::ExcludeTax => "exclude_tax",
            CreateInvoiceRenderingAmountTaxDisplay::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str> for CreateInvoiceRenderingAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceRenderingAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateInvoiceRenderingAmountTaxDisplay {
    fn default() -> Self {
        Self::ExcludeTax
    }
}

/// An enum representing the possible values of an `CreateInvoiceRenderingOptions`'s `amount_tax_display` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl CreateInvoiceRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceRenderingOptionsAmountTaxDisplay::ExcludeTax => "exclude_tax",
            CreateInvoiceRenderingOptionsAmountTaxDisplay::IncludeInclusiveTax => {
                "include_inclusive_tax"
            }
        }
    }
}

impl AsRef<str> for CreateInvoiceRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateInvoiceRenderingOptionsAmountTaxDisplay {
    fn default() -> Self {
        Self::ExcludeTax
    }
}

/// An enum representing the possible values of an `CreateInvoiceRenderingPdf`'s `page_size` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceRenderingPdfPageSize {
    A4,
    Auto,
    Letter,
}

impl CreateInvoiceRenderingPdfPageSize {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceRenderingPdfPageSize::A4 => "a4",
            CreateInvoiceRenderingPdfPageSize::Auto => "auto",
            CreateInvoiceRenderingPdfPageSize::Letter => "letter",
        }
    }
}

impl AsRef<str> for CreateInvoiceRenderingPdfPageSize {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateInvoiceRenderingPdfPageSize {
    fn default() -> Self {
        Self::A4
    }
}

/// An enum representing the possible values of an `CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::BusinessDay => {
                "business_day"
            }
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Day => "day",
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Hour => "hour",
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Month => "month",
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit::Week => "week",
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMaximumUnit
{
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::BusinessDay => {
                "business_day"
            }
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Day => "day",
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Hour => "hour",
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Month => "month",
            CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit::Week => "week",
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateInvoiceShippingCostShippingRateDataDeliveryEstimateMinimumUnit
{
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptions`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Exclusive => "exclusive",
            CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Inclusive => "inclusive",
            CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default
    for CreateInvoiceShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateInvoiceShippingCostShippingRateData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceShippingCostShippingRateDataTaxBehavior::Exclusive => "exclusive",
            CreateInvoiceShippingCostShippingRateDataTaxBehavior::Inclusive => "inclusive",
            CreateInvoiceShippingCostShippingRateDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateInvoiceShippingCostShippingRateDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateInvoiceShippingCostShippingRateData`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceShippingCostShippingRateDataType {
    FixedAmount,
}

impl CreateInvoiceShippingCostShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateInvoiceShippingCostShippingRateDataType::FixedAmount => "fixed_amount",
        }
    }
}

impl AsRef<str> for CreateInvoiceShippingCostShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceShippingCostShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for CreateInvoiceShippingCostShippingRateDataType {
    fn default() -> Self {
        Self::FixedAmount
    }
}

/// An enum representing the possible values of an `Invoice`'s `billing_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceBillingReason {
    AutomaticPendingInvoiceItemInvoice,
    Manual,
    QuoteAccept,
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
            InvoiceBillingReason::QuoteAccept => "quote_accept",
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
impl std::default::Default for InvoiceBillingReason {
    fn default() -> Self {
        Self::AutomaticPendingInvoiceItemInvoice
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
impl std::default::Default for InvoiceCustomerTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}

/// An enum representing the possible values of an `InvoicePaymentMethodOptionsCard`'s `request_three_d_secure` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
    Challenge,
}

impl InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentMethodOptionsCardRequestThreeDSecure::Any => "any",
            InvoicePaymentMethodOptionsCardRequestThreeDSecure::Automatic => "automatic",
            InvoicePaymentMethodOptionsCardRequestThreeDSecure::Challenge => "challenge",
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoicePaymentMethodOptionsCardRequestThreeDSecure {
    fn default() -> Self {
        Self::Any
    }
}

/// An enum representing the possible values of an `CreateInvoice`'s `pending_invoice_items_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePendingInvoiceItemsBehavior {
    Exclude,
    Include,
    IncludeAndRequire,
}

impl InvoicePendingInvoiceItemsBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePendingInvoiceItemsBehavior::Exclude => "exclude",
            InvoicePendingInvoiceItemsBehavior::Include => "include",
            InvoicePendingInvoiceItemsBehavior::IncludeAndRequire => "include_and_require",
        }
    }
}

impl AsRef<str> for InvoicePendingInvoiceItemsBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePendingInvoiceItemsBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoicePendingInvoiceItemsBehavior {
    fn default() -> Self {
        Self::Exclude
    }
}

/// An enum representing the possible values of an `InvoiceRenderingPdf`'s `page_size` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceRenderingPdfPageSize {
    A4,
    Auto,
    Letter,
}

impl InvoiceRenderingPdfPageSize {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceRenderingPdfPageSize::A4 => "a4",
            InvoiceRenderingPdfPageSize::Auto => "auto",
            InvoiceRenderingPdfPageSize::Letter => "letter",
        }
    }
}

impl AsRef<str> for InvoiceRenderingPdfPageSize {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceRenderingPdfPageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoiceRenderingPdfPageSize {
    fn default() -> Self {
        Self::A4
    }
}

/// An enum representing the possible values of an `Invoice`'s `status` field.
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
impl std::default::Default for InvoiceStatus {
    fn default() -> Self {
        Self::Draft
    }
}

/// An enum representing the possible values of an `InvoicesPaymentSettings`'s `payment_method_types` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicesPaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    Cashapp,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    P24,
    Paynow,
    Paypal,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl InvoicesPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicesPaymentSettingsPaymentMethodTypes::AchCreditTransfer => "ach_credit_transfer",
            InvoicesPaymentSettingsPaymentMethodTypes::AchDebit => "ach_debit",
            InvoicesPaymentSettingsPaymentMethodTypes::AcssDebit => "acss_debit",
            InvoicesPaymentSettingsPaymentMethodTypes::AuBecsDebit => "au_becs_debit",
            InvoicesPaymentSettingsPaymentMethodTypes::BacsDebit => "bacs_debit",
            InvoicesPaymentSettingsPaymentMethodTypes::Bancontact => "bancontact",
            InvoicesPaymentSettingsPaymentMethodTypes::Boleto => "boleto",
            InvoicesPaymentSettingsPaymentMethodTypes::Card => "card",
            InvoicesPaymentSettingsPaymentMethodTypes::Cashapp => "cashapp",
            InvoicesPaymentSettingsPaymentMethodTypes::CustomerBalance => "customer_balance",
            InvoicesPaymentSettingsPaymentMethodTypes::Eps => "eps",
            InvoicesPaymentSettingsPaymentMethodTypes::Fpx => "fpx",
            InvoicesPaymentSettingsPaymentMethodTypes::Giropay => "giropay",
            InvoicesPaymentSettingsPaymentMethodTypes::Grabpay => "grabpay",
            InvoicesPaymentSettingsPaymentMethodTypes::Ideal => "ideal",
            InvoicesPaymentSettingsPaymentMethodTypes::Konbini => "konbini",
            InvoicesPaymentSettingsPaymentMethodTypes::Link => "link",
            InvoicesPaymentSettingsPaymentMethodTypes::P24 => "p24",
            InvoicesPaymentSettingsPaymentMethodTypes::Paynow => "paynow",
            InvoicesPaymentSettingsPaymentMethodTypes::Paypal => "paypal",
            InvoicesPaymentSettingsPaymentMethodTypes::Promptpay => "promptpay",
            InvoicesPaymentSettingsPaymentMethodTypes::SepaCreditTransfer => "sepa_credit_transfer",
            InvoicesPaymentSettingsPaymentMethodTypes::SepaDebit => "sepa_debit",
            InvoicesPaymentSettingsPaymentMethodTypes::Sofort => "sofort",
            InvoicesPaymentSettingsPaymentMethodTypes::UsBankAccount => "us_bank_account",
            InvoicesPaymentSettingsPaymentMethodTypes::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for InvoicesPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicesPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoicesPaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}

/// An enum representing the possible values of an `TaxAmount`'s `taxability_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxAmountTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
}

impl TaxAmountTaxabilityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxAmountTaxabilityReason::CustomerExempt => "customer_exempt",
            TaxAmountTaxabilityReason::NotCollecting => "not_collecting",
            TaxAmountTaxabilityReason::NotSubjectToTax => "not_subject_to_tax",
            TaxAmountTaxabilityReason::NotSupported => "not_supported",
            TaxAmountTaxabilityReason::PortionProductExempt => "portion_product_exempt",
            TaxAmountTaxabilityReason::PortionReducedRated => "portion_reduced_rated",
            TaxAmountTaxabilityReason::PortionStandardRated => "portion_standard_rated",
            TaxAmountTaxabilityReason::ProductExempt => "product_exempt",
            TaxAmountTaxabilityReason::ProductExemptHoliday => "product_exempt_holiday",
            TaxAmountTaxabilityReason::ProportionallyRated => "proportionally_rated",
            TaxAmountTaxabilityReason::ReducedRated => "reduced_rated",
            TaxAmountTaxabilityReason::ReverseCharge => "reverse_charge",
            TaxAmountTaxabilityReason::StandardRated => "standard_rated",
            TaxAmountTaxabilityReason::TaxableBasisReduced => "taxable_basis_reduced",
            TaxAmountTaxabilityReason::ZeroRated => "zero_rated",
        }
    }
}

impl AsRef<str> for TaxAmountTaxabilityReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxAmountTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for TaxAmountTaxabilityReason {
    fn default() -> Self {
        Self::CustomerExempt
    }
}

/// An enum representing the possible values of an `InvoicesResourceInvoiceTaxId`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaxIdType {
    AdNrt,
    AeTrn,
    ArCuit,
    AuAbn,
    AuArn,
    BgUic,
    BoTin,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    CnTin,
    CoNit,
    CrTin,
    DoRcn,
    EcRuc,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    PeRuc,
    PhTin,
    RoTin,
    RsPib,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    SvNit,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    Unknown,
    UsEin,
    UyRuc,
    VeRif,
    VnTin,
    ZaVat,
}

impl TaxIdType {
    pub fn as_str(self) -> &'static str {
        match self {
            TaxIdType::AdNrt => "ad_nrt",
            TaxIdType::AeTrn => "ae_trn",
            TaxIdType::ArCuit => "ar_cuit",
            TaxIdType::AuAbn => "au_abn",
            TaxIdType::AuArn => "au_arn",
            TaxIdType::BgUic => "bg_uic",
            TaxIdType::BoTin => "bo_tin",
            TaxIdType::BrCnpj => "br_cnpj",
            TaxIdType::BrCpf => "br_cpf",
            TaxIdType::CaBn => "ca_bn",
            TaxIdType::CaGstHst => "ca_gst_hst",
            TaxIdType::CaPstBc => "ca_pst_bc",
            TaxIdType::CaPstMb => "ca_pst_mb",
            TaxIdType::CaPstSk => "ca_pst_sk",
            TaxIdType::CaQst => "ca_qst",
            TaxIdType::ChVat => "ch_vat",
            TaxIdType::ClTin => "cl_tin",
            TaxIdType::CnTin => "cn_tin",
            TaxIdType::CoNit => "co_nit",
            TaxIdType::CrTin => "cr_tin",
            TaxIdType::DoRcn => "do_rcn",
            TaxIdType::EcRuc => "ec_ruc",
            TaxIdType::EgTin => "eg_tin",
            TaxIdType::EsCif => "es_cif",
            TaxIdType::EuOssVat => "eu_oss_vat",
            TaxIdType::EuVat => "eu_vat",
            TaxIdType::GbVat => "gb_vat",
            TaxIdType::GeVat => "ge_vat",
            TaxIdType::HkBr => "hk_br",
            TaxIdType::HuTin => "hu_tin",
            TaxIdType::IdNpwp => "id_npwp",
            TaxIdType::IlVat => "il_vat",
            TaxIdType::InGst => "in_gst",
            TaxIdType::IsVat => "is_vat",
            TaxIdType::JpCn => "jp_cn",
            TaxIdType::JpRn => "jp_rn",
            TaxIdType::JpTrn => "jp_trn",
            TaxIdType::KePin => "ke_pin",
            TaxIdType::KrBrn => "kr_brn",
            TaxIdType::LiUid => "li_uid",
            TaxIdType::MxRfc => "mx_rfc",
            TaxIdType::MyFrp => "my_frp",
            TaxIdType::MyItn => "my_itn",
            TaxIdType::MySst => "my_sst",
            TaxIdType::NoVat => "no_vat",
            TaxIdType::NzGst => "nz_gst",
            TaxIdType::PeRuc => "pe_ruc",
            TaxIdType::PhTin => "ph_tin",
            TaxIdType::RoTin => "ro_tin",
            TaxIdType::RsPib => "rs_pib",
            TaxIdType::RuInn => "ru_inn",
            TaxIdType::RuKpp => "ru_kpp",
            TaxIdType::SaVat => "sa_vat",
            TaxIdType::SgGst => "sg_gst",
            TaxIdType::SgUen => "sg_uen",
            TaxIdType::SiTin => "si_tin",
            TaxIdType::SvNit => "sv_nit",
            TaxIdType::ThVat => "th_vat",
            TaxIdType::TrTin => "tr_tin",
            TaxIdType::TwVat => "tw_vat",
            TaxIdType::UaVat => "ua_vat",
            TaxIdType::Unknown => "unknown",
            TaxIdType::UsEin => "us_ein",
            TaxIdType::UyRuc => "uy_ruc",
            TaxIdType::VeRif => "ve_rif",
            TaxIdType::VnTin => "vn_tin",
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
impl std::default::Default for TaxIdType {
    fn default() -> Self {
        Self::AdNrt
    }
}
