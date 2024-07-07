/// Invoices are statements of amounts owed by a customer, and are either
/// generated one-off, or generated periodically from a subscription.
///
/// They contain [invoice items](https://stripe.com/docs/api#invoiceitems), and proration adjustments
/// that may be caused by subscription upgrades/downgrades (if necessary).
///
/// If your invoice is configured to be billed through automatic charges,
/// Stripe automatically finalizes your invoice and attempts payment. Note
/// that finalizing the invoice,
/// [when automatic](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection), does.
/// not happen immediately as the invoice is created. Stripe waits
/// until one hour after the last webhook was successfully sent (or the last
/// webhook timed out after failing). If you (and the platforms you may have
/// connected to) have no webhooks configured, Stripe waits one hour after
/// creation to finalize the invoice.
///
/// If your invoice is configured to be billed by sending an email, then based on your
/// [email settings](https://dashboard.stripe.com/account/billing/automatic),
/// Stripe will email the invoice to your customer and await payment. These
/// emails can contain a link to a hosted page to pay the invoice.
///
/// Stripe applies any customer credit on the account before determining the
/// amount due for the invoice (i.e., the amount that will be actually
/// charged). If the amount due for the invoice is less than Stripe's [minimum allowed charge
/// per currency](/docs/currencies#minimum-and-maximum-charge-amounts), the
/// invoice is automatically marked paid, and we add the amount due to the
/// customer's credit balance which is applied to the next invoice.
///
/// More details on the customer's credit balance are
/// [here](https://stripe.com/docs/billing/customer/balance).
///
/// Related guide: [Send invoices to customers](https://stripe.com/docs/billing/invoices/sending)
///
/// For more details see <<https://stripe.com/docs/api/invoices/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct Invoice {
    /// The country of the business associated with this invoice, most often the business creating the invoice.
    pub account_country: Option<String>,
    /// The public name of the business associated with this invoice, most often the business creating the invoice.
    pub account_name: Option<String>,
    /// The account tax IDs associated with the invoice. Only editable when the invoice is a draft.
    pub account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>,
    /// Final amount due at this time for this invoice.
    /// If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0.
    /// If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account.
    /// The charge that gets generated for the invoice will be for the amount specified in `amount_due`.
    pub amount_due: i64,
    /// The amount, in cents (or local equivalent), that was paid.
    pub amount_paid: i64,
    /// The difference between amount_due and amount_paid, in cents (or local equivalent).
    pub amount_remaining: i64,
    /// This is the sum of all the shipping amounts.
    pub amount_shipping: i64,
    /// ID of the Connect Application that created the invoice.
    pub application: Option<stripe_types::Expandable<stripe_shared::Application>>,
    /// The fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account when the invoice is paid.
    pub application_fee_amount: Option<i64>,
    /// Number of payment attempts made for this invoice, from the perspective of the payment retry schedule.
    /// Any payment attempt counts as the first attempt, and subsequently only automatic retries increment the attempt count.
    /// In other words, manual payment attempts after the first attempt do not affect the retry schedule.
    pub attempt_count: u64,
    /// Whether an attempt has been made to pay the invoice.
    /// An invoice is not attempted until 1 hour after the `invoice.created` webhook, for example, so you might not want to display that invoice as unpaid to your users.
    pub attempted: bool,
    /// Controls whether Stripe performs [automatic collection](https://stripe.com/docs/invoicing/integration/automatic-advancement-collection) of the invoice.
    /// If `false`, the invoice's state doesn't automatically advance without an explicit action.
    pub auto_advance: Option<bool>,
    pub automatic_tax: stripe_shared::AutomaticTax,
    /// Indicates the reason why the invoice was created.
    ///
    /// * `manual`: Unrelated to a subscription, for example, created via the invoice editor.
    /// * `subscription`: No longer in use.
    /// Applies to subscriptions from before May 2018 where no distinction was made between updates, cycles, and thresholds.
    /// * `subscription_create`: A new subscription was created.
    /// * `subscription_cycle`: A subscription advanced into a new period.
    /// * `subscription_threshold`: A subscription reached a billing threshold.
    /// * `subscription_update`: A subscription was updated.
    /// * `upcoming`: Reserved for simulated invoices, per the upcoming invoice endpoint.
    pub billing_reason: Option<InvoiceBillingReason>,
    /// ID of the latest charge generated for this invoice, if any.
    pub charge: Option<stripe_types::Expandable<stripe_shared::Charge>>,
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    pub collection_method: stripe_shared::InvoiceCollectionMethod,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Custom fields displayed on the invoice.
    pub custom_fields: Option<Vec<stripe_shared::InvoiceSettingCustomField>>,
    /// The ID of the customer who will be billed.
    pub customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    /// The customer's address.
    /// Until the invoice is finalized, this field will equal `customer.address`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_address: Option<stripe_shared::Address>,
    /// The customer's email.
    /// Until the invoice is finalized, this field will equal `customer.email`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_email: Option<String>,
    /// The customer's name.
    /// Until the invoice is finalized, this field will equal `customer.name`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_name: Option<String>,
    /// The customer's phone number.
    /// Until the invoice is finalized, this field will equal `customer.phone`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_phone: Option<String>,
    /// The customer's shipping information.
    /// Until the invoice is finalized, this field will equal `customer.shipping`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_shipping: Option<stripe_shared::Shipping>,
    /// The customer's tax exempt status.
    /// Until the invoice is finalized, this field will equal `customer.tax_exempt`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_tax_exempt: Option<InvoiceCustomerTaxExempt>,
    /// The customer's tax IDs.
    /// Until the invoice is finalized, this field will contain the same tax IDs as `customer.tax_ids`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_tax_ids: Option<Vec<stripe_shared::InvoicesResourceInvoiceTaxId>>,
    /// ID of the default payment method for the invoice.
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>,
    /// ID of the default payment source for the invoice.
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    pub default_source: Option<stripe_types::Expandable<stripe_shared::PaymentSource>>,
    /// The tax rates applied to this invoice, if any.
    pub default_tax_rates: Vec<stripe_shared::TaxRate>,
    /// An arbitrary string attached to the object.
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    pub description: Option<String>,
    /// Describes the current discount applied to this invoice, if there is one.
    /// Not populated if there are multiple discounts.
    pub discount: Option<stripe_shared::Discount>,
    /// The discounts applied to the invoice.
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Vec<stripe_types::Expandable<stripe_shared::Discount>>,
    /// The date on which payment for this invoice is due.
    /// This value will be `null` for invoices where `collection_method=charge_automatically`.
    pub due_date: Option<stripe_types::Timestamp>,
    /// The date when this invoice is in effect.
    /// Same as `finalized_at` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the invoice PDF and receipt.
    pub effective_at: Option<stripe_types::Timestamp>,
    /// Ending customer balance after the invoice is finalized.
    /// Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice.
    /// If the invoice has not been finalized yet, this will be null.
    pub ending_balance: Option<i64>,
    /// Footer displayed on the invoice.
    pub footer: Option<String>,
    /// Details of the invoice that was cloned.
    /// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    pub from_invoice: Option<stripe_shared::InvoicesResourceFromInvoice>,
    /// The URL for the hosted invoice page, which allows customers to view and pay an invoice.
    /// If the invoice has not been finalized yet, this will be null.
    pub hosted_invoice_url: Option<String>,
    /// Unique identifier for the object.
    /// This property is always present unless the invoice is an upcoming invoice.
    /// See [Retrieve an upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) for more details.
    pub id: Option<stripe_shared::InvoiceId>,
    /// The link to download the PDF for the invoice.
    /// If the invoice has not been finalized yet, this will be null.
    pub invoice_pdf: Option<String>,
    pub issuer: stripe_shared::ConnectAccountReference,
    /// The error encountered during the previous attempt to finalize the invoice.
    /// This field is cleared when the invoice is successfully finalized.
    pub last_finalization_error: Option<Box<stripe_shared::ApiErrors>>,
    /// The ID of the most recent non-draft revision of this invoice
    pub latest_revision: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    /// The individual line items that make up the invoice.
    /// `lines` is sorted as follows: (1) pending invoice items (including prorations) in reverse chronological order, (2) subscription items in reverse chronological order, and (3) invoice items added after invoice creation in chronological order.
    pub lines: stripe_types::List<stripe_shared::InvoiceLineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The time at which payment will next be attempted.
    /// This value will be `null` for invoices where `collection_method=send_invoice`.
    pub next_payment_attempt: Option<stripe_types::Timestamp>,
    /// A unique, identifying string that appears on emails sent to the customer for this invoice.
    /// This starts with the customer's unique invoice_prefix if it is specified.
    pub number: Option<String>,
    /// The account (if any) for which the funds of the invoice payment are intended.
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_shared::Account>>,
    /// Whether payment was successfully collected for this invoice.
    /// An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance.
    pub paid: bool,
    /// Returns true if the invoice was manually marked paid, returns false if the invoice hasn't been paid yet or was paid on Stripe.
    pub paid_out_of_band: bool,
    /// The PaymentIntent associated with this invoice.
    /// The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice.
    /// Note that voiding an invoice will cancel the PaymentIntent.
    pub payment_intent: Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>,
    pub payment_settings: stripe_shared::InvoicesPaymentSettings,
    /// End of the usage period during which invoice items were added to this invoice.
    /// This looks back one period for a subscription invoice.
    /// Use the [line item period](/api/invoices/line_item#invoice_line_item_object-period) to get the service period for each price.
    pub period_end: stripe_types::Timestamp,
    /// Start of the usage period during which invoice items were added to this invoice.
    /// This looks back one period for a subscription invoice.
    /// Use the [line item period](/api/invoices/line_item#invoice_line_item_object-period) to get the service period for each price.
    pub period_start: stripe_types::Timestamp,
    /// Total amount of all post-payment credit notes issued for this invoice.
    pub post_payment_credit_notes_amount: i64,
    /// Total amount of all pre-payment credit notes issued for this invoice.
    pub pre_payment_credit_notes_amount: i64,
    /// The quote this invoice was generated from.
    pub quote: Option<stripe_types::Expandable<stripe_shared::Quote>>,
    /// This is the transaction number that appears on email receipts sent for this invoice.
    pub receipt_number: Option<String>,
    /// The rendering-related settings that control how the invoice is displayed on customer-facing surfaces such as PDF and Hosted Invoice Page.
    pub rendering: Option<stripe_shared::InvoicesResourceInvoiceRendering>,
    /// The details of the cost of shipping, including the ShippingRate applied on the invoice.
    pub shipping_cost: Option<stripe_shared::InvoicesResourceShippingCost>,
    /// Shipping details for the invoice.
    /// The Invoice PDF will use the `shipping_details` value if it is set, otherwise the PDF will render the shipping address from the customer.
    pub shipping_details: Option<stripe_shared::Shipping>,
    /// Starting customer balance before the invoice is finalized.
    /// If the invoice has not been finalized yet, this will be the current customer balance.
    /// For revision invoices, this also includes any customer balance that was applied to the original invoice.
    pub starting_balance: i64,
    /// Extra information about an invoice for the customer's credit card statement.
    pub statement_descriptor: Option<String>,
    /// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
    /// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    pub status: Option<stripe_shared::InvoiceStatus>,
    pub status_transitions: stripe_shared::InvoicesResourceStatusTransitions,
    /// The subscription that this invoice was prepared for, if any.
    pub subscription: Option<stripe_types::Expandable<stripe_shared::Subscription>>,
    /// Details about the subscription that created this invoice.
    pub subscription_details: Option<stripe_shared::SubscriptionDetailsData>,
    /// Only set for upcoming invoices that preview prorations. The time used to calculate prorations.
    pub subscription_proration_date: Option<stripe_types::Timestamp>,
    /// Total of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusive tax is applied.
    /// Item discounts are already incorporated.
    pub subtotal: i64,
    /// The integer amount in cents (or local equivalent) representing the subtotal of the invoice before any invoice level discount or tax is applied.
    /// Item discounts are already incorporated.
    pub subtotal_excluding_tax: Option<i64>,
    /// The amount of tax on this invoice. This is the sum of all the tax amounts on this invoice.
    pub tax: Option<i64>,
    /// ID of the test clock this invoice belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>,
    pub threshold_reason: Option<stripe_shared::InvoiceThresholdReason>,
    /// Total after discounts and taxes.
    pub total: i64,
    /// The aggregate amounts calculated per discount across all line items.
    pub total_discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
    /// The integer amount in cents (or local equivalent) representing the total amount of the invoice including all discounts but excluding all tax.
    pub total_excluding_tax: Option<i64>,
    /// The aggregate amounts calculated per tax rate for all line items.
    pub total_tax_amounts: Vec<stripe_shared::InvoiceTaxAmount>,
    /// The account (if any) the payment will be attributed to for tax reporting, and where funds from the payment will be transferred to for the invoice.
    pub transfer_data: Option<stripe_shared::InvoiceTransferData>,
    /// Invoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts have [been exhausted](https://stripe.com/docs/billing/webhooks#understand).
    /// This field tracks the time when webhooks for this invoice were successfully delivered.
    /// If the invoice had no webhooks to deliver, this will be set while the invoice is being created.
    pub webhooks_delivered_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct InvoiceBuilder {
    account_country: Option<Option<String>>,
    account_name: Option<Option<String>>,
    account_tax_ids: Option<Option<Vec<stripe_types::Expandable<stripe_shared::TaxId>>>>,
    amount_due: Option<i64>,
    amount_paid: Option<i64>,
    amount_remaining: Option<i64>,
    amount_shipping: Option<i64>,
    application: Option<Option<stripe_types::Expandable<stripe_shared::Application>>>,
    application_fee_amount: Option<Option<i64>>,
    attempt_count: Option<u64>,
    attempted: Option<bool>,
    auto_advance: Option<Option<bool>>,
    automatic_tax: Option<stripe_shared::AutomaticTax>,
    billing_reason: Option<Option<InvoiceBillingReason>>,
    charge: Option<Option<stripe_types::Expandable<stripe_shared::Charge>>>,
    collection_method: Option<stripe_shared::InvoiceCollectionMethod>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    custom_fields: Option<Option<Vec<stripe_shared::InvoiceSettingCustomField>>>,
    customer: Option<Option<stripe_types::Expandable<stripe_shared::Customer>>>,
    customer_address: Option<Option<stripe_shared::Address>>,
    customer_email: Option<Option<String>>,
    customer_name: Option<Option<String>>,
    customer_phone: Option<Option<String>>,
    customer_shipping: Option<Option<stripe_shared::Shipping>>,
    customer_tax_exempt: Option<Option<InvoiceCustomerTaxExempt>>,
    customer_tax_ids: Option<Option<Vec<stripe_shared::InvoicesResourceInvoiceTaxId>>>,
    default_payment_method: Option<Option<stripe_types::Expandable<stripe_shared::PaymentMethod>>>,
    default_source: Option<Option<stripe_types::Expandable<stripe_shared::PaymentSource>>>,
    default_tax_rates: Option<Vec<stripe_shared::TaxRate>>,
    description: Option<Option<String>>,
    discount: Option<Option<stripe_shared::Discount>>,
    discounts: Option<Vec<stripe_types::Expandable<stripe_shared::Discount>>>,
    due_date: Option<Option<stripe_types::Timestamp>>,
    effective_at: Option<Option<stripe_types::Timestamp>>,
    ending_balance: Option<Option<i64>>,
    footer: Option<Option<String>>,
    from_invoice: Option<Option<stripe_shared::InvoicesResourceFromInvoice>>,
    hosted_invoice_url: Option<Option<String>>,
    id: Option<Option<stripe_shared::InvoiceId>>,
    invoice_pdf: Option<Option<String>>,
    issuer: Option<stripe_shared::ConnectAccountReference>,
    last_finalization_error: Option<Option<Box<stripe_shared::ApiErrors>>>,
    latest_revision: Option<Option<stripe_types::Expandable<stripe_shared::Invoice>>>,
    lines: Option<stripe_types::List<stripe_shared::InvoiceLineItem>>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    next_payment_attempt: Option<Option<stripe_types::Timestamp>>,
    number: Option<Option<String>>,
    on_behalf_of: Option<Option<stripe_types::Expandable<stripe_shared::Account>>>,
    paid: Option<bool>,
    paid_out_of_band: Option<bool>,
    payment_intent: Option<Option<stripe_types::Expandable<stripe_shared::PaymentIntent>>>,
    payment_settings: Option<stripe_shared::InvoicesPaymentSettings>,
    period_end: Option<stripe_types::Timestamp>,
    period_start: Option<stripe_types::Timestamp>,
    post_payment_credit_notes_amount: Option<i64>,
    pre_payment_credit_notes_amount: Option<i64>,
    quote: Option<Option<stripe_types::Expandable<stripe_shared::Quote>>>,
    receipt_number: Option<Option<String>>,
    rendering: Option<Option<stripe_shared::InvoicesResourceInvoiceRendering>>,
    shipping_cost: Option<Option<stripe_shared::InvoicesResourceShippingCost>>,
    shipping_details: Option<Option<stripe_shared::Shipping>>,
    starting_balance: Option<i64>,
    statement_descriptor: Option<Option<String>>,
    status: Option<Option<stripe_shared::InvoiceStatus>>,
    status_transitions: Option<stripe_shared::InvoicesResourceStatusTransitions>,
    subscription: Option<Option<stripe_types::Expandable<stripe_shared::Subscription>>>,
    subscription_details: Option<Option<stripe_shared::SubscriptionDetailsData>>,
    subscription_proration_date: Option<Option<stripe_types::Timestamp>>,
    subtotal: Option<i64>,
    subtotal_excluding_tax: Option<Option<i64>>,
    tax: Option<Option<i64>>,
    test_clock: Option<Option<stripe_types::Expandable<stripe_shared::TestHelpersTestClock>>>,
    threshold_reason: Option<Option<stripe_shared::InvoiceThresholdReason>>,
    total: Option<i64>,
    total_discount_amounts: Option<Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>>,
    total_excluding_tax: Option<Option<i64>>,
    total_tax_amounts: Option<Vec<stripe_shared::InvoiceTaxAmount>>,
    transfer_data: Option<Option<stripe_shared::InvoiceTransferData>>,
    webhooks_delivered_at: Option<Option<stripe_types::Timestamp>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for Invoice {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<Invoice>,
        builder: InvoiceBuilder,
    }

    impl Visitor for Place<Invoice> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder { out: &mut self.out, builder: InvoiceBuilder::deser_default() }))
        }
    }

    impl MapBuilder for InvoiceBuilder {
        type Out = Invoice;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "account_country" => Deserialize::begin(&mut self.account_country),
                "account_name" => Deserialize::begin(&mut self.account_name),
                "account_tax_ids" => Deserialize::begin(&mut self.account_tax_ids),
                "amount_due" => Deserialize::begin(&mut self.amount_due),
                "amount_paid" => Deserialize::begin(&mut self.amount_paid),
                "amount_remaining" => Deserialize::begin(&mut self.amount_remaining),
                "amount_shipping" => Deserialize::begin(&mut self.amount_shipping),
                "application" => Deserialize::begin(&mut self.application),
                "application_fee_amount" => Deserialize::begin(&mut self.application_fee_amount),
                "attempt_count" => Deserialize::begin(&mut self.attempt_count),
                "attempted" => Deserialize::begin(&mut self.attempted),
                "auto_advance" => Deserialize::begin(&mut self.auto_advance),
                "automatic_tax" => Deserialize::begin(&mut self.automatic_tax),
                "billing_reason" => Deserialize::begin(&mut self.billing_reason),
                "charge" => Deserialize::begin(&mut self.charge),
                "collection_method" => Deserialize::begin(&mut self.collection_method),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "custom_fields" => Deserialize::begin(&mut self.custom_fields),
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_address" => Deserialize::begin(&mut self.customer_address),
                "customer_email" => Deserialize::begin(&mut self.customer_email),
                "customer_name" => Deserialize::begin(&mut self.customer_name),
                "customer_phone" => Deserialize::begin(&mut self.customer_phone),
                "customer_shipping" => Deserialize::begin(&mut self.customer_shipping),
                "customer_tax_exempt" => Deserialize::begin(&mut self.customer_tax_exempt),
                "customer_tax_ids" => Deserialize::begin(&mut self.customer_tax_ids),
                "default_payment_method" => Deserialize::begin(&mut self.default_payment_method),
                "default_source" => Deserialize::begin(&mut self.default_source),
                "default_tax_rates" => Deserialize::begin(&mut self.default_tax_rates),
                "description" => Deserialize::begin(&mut self.description),
                "discount" => Deserialize::begin(&mut self.discount),
                "discounts" => Deserialize::begin(&mut self.discounts),
                "due_date" => Deserialize::begin(&mut self.due_date),
                "effective_at" => Deserialize::begin(&mut self.effective_at),
                "ending_balance" => Deserialize::begin(&mut self.ending_balance),
                "footer" => Deserialize::begin(&mut self.footer),
                "from_invoice" => Deserialize::begin(&mut self.from_invoice),
                "hosted_invoice_url" => Deserialize::begin(&mut self.hosted_invoice_url),
                "id" => Deserialize::begin(&mut self.id),
                "invoice_pdf" => Deserialize::begin(&mut self.invoice_pdf),
                "issuer" => Deserialize::begin(&mut self.issuer),
                "last_finalization_error" => Deserialize::begin(&mut self.last_finalization_error),
                "latest_revision" => Deserialize::begin(&mut self.latest_revision),
                "lines" => Deserialize::begin(&mut self.lines),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "next_payment_attempt" => Deserialize::begin(&mut self.next_payment_attempt),
                "number" => Deserialize::begin(&mut self.number),
                "on_behalf_of" => Deserialize::begin(&mut self.on_behalf_of),
                "paid" => Deserialize::begin(&mut self.paid),
                "paid_out_of_band" => Deserialize::begin(&mut self.paid_out_of_band),
                "payment_intent" => Deserialize::begin(&mut self.payment_intent),
                "payment_settings" => Deserialize::begin(&mut self.payment_settings),
                "period_end" => Deserialize::begin(&mut self.period_end),
                "period_start" => Deserialize::begin(&mut self.period_start),
                "post_payment_credit_notes_amount" => {
                    Deserialize::begin(&mut self.post_payment_credit_notes_amount)
                }
                "pre_payment_credit_notes_amount" => {
                    Deserialize::begin(&mut self.pre_payment_credit_notes_amount)
                }
                "quote" => Deserialize::begin(&mut self.quote),
                "receipt_number" => Deserialize::begin(&mut self.receipt_number),
                "rendering" => Deserialize::begin(&mut self.rendering),
                "shipping_cost" => Deserialize::begin(&mut self.shipping_cost),
                "shipping_details" => Deserialize::begin(&mut self.shipping_details),
                "starting_balance" => Deserialize::begin(&mut self.starting_balance),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),
                "status" => Deserialize::begin(&mut self.status),
                "status_transitions" => Deserialize::begin(&mut self.status_transitions),
                "subscription" => Deserialize::begin(&mut self.subscription),
                "subscription_details" => Deserialize::begin(&mut self.subscription_details),
                "subscription_proration_date" => {
                    Deserialize::begin(&mut self.subscription_proration_date)
                }
                "subtotal" => Deserialize::begin(&mut self.subtotal),
                "subtotal_excluding_tax" => Deserialize::begin(&mut self.subtotal_excluding_tax),
                "tax" => Deserialize::begin(&mut self.tax),
                "test_clock" => Deserialize::begin(&mut self.test_clock),
                "threshold_reason" => Deserialize::begin(&mut self.threshold_reason),
                "total" => Deserialize::begin(&mut self.total),
                "total_discount_amounts" => Deserialize::begin(&mut self.total_discount_amounts),
                "total_excluding_tax" => Deserialize::begin(&mut self.total_excluding_tax),
                "total_tax_amounts" => Deserialize::begin(&mut self.total_tax_amounts),
                "transfer_data" => Deserialize::begin(&mut self.transfer_data),
                "webhooks_delivered_at" => Deserialize::begin(&mut self.webhooks_delivered_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                account_country: Deserialize::default(),
                account_name: Deserialize::default(),
                account_tax_ids: Deserialize::default(),
                amount_due: Deserialize::default(),
                amount_paid: Deserialize::default(),
                amount_remaining: Deserialize::default(),
                amount_shipping: Deserialize::default(),
                application: Deserialize::default(),
                application_fee_amount: Deserialize::default(),
                attempt_count: Deserialize::default(),
                attempted: Deserialize::default(),
                auto_advance: Deserialize::default(),
                automatic_tax: Deserialize::default(),
                billing_reason: Deserialize::default(),
                charge: Deserialize::default(),
                collection_method: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                custom_fields: Deserialize::default(),
                customer: Deserialize::default(),
                customer_address: Deserialize::default(),
                customer_email: Deserialize::default(),
                customer_name: Deserialize::default(),
                customer_phone: Deserialize::default(),
                customer_shipping: Deserialize::default(),
                customer_tax_exempt: Deserialize::default(),
                customer_tax_ids: Deserialize::default(),
                default_payment_method: Deserialize::default(),
                default_source: Deserialize::default(),
                default_tax_rates: Deserialize::default(),
                description: Deserialize::default(),
                discount: Deserialize::default(),
                discounts: Deserialize::default(),
                due_date: Deserialize::default(),
                effective_at: Deserialize::default(),
                ending_balance: Deserialize::default(),
                footer: Deserialize::default(),
                from_invoice: Deserialize::default(),
                hosted_invoice_url: Deserialize::default(),
                id: Deserialize::default(),
                invoice_pdf: Deserialize::default(),
                issuer: Deserialize::default(),
                last_finalization_error: Deserialize::default(),
                latest_revision: Deserialize::default(),
                lines: Deserialize::default(),
                livemode: Deserialize::default(),
                metadata: Deserialize::default(),
                next_payment_attempt: Deserialize::default(),
                number: Deserialize::default(),
                on_behalf_of: Deserialize::default(),
                paid: Deserialize::default(),
                paid_out_of_band: Deserialize::default(),
                payment_intent: Deserialize::default(),
                payment_settings: Deserialize::default(),
                period_end: Deserialize::default(),
                period_start: Deserialize::default(),
                post_payment_credit_notes_amount: Deserialize::default(),
                pre_payment_credit_notes_amount: Deserialize::default(),
                quote: Deserialize::default(),
                receipt_number: Deserialize::default(),
                rendering: Deserialize::default(),
                shipping_cost: Deserialize::default(),
                shipping_details: Deserialize::default(),
                starting_balance: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
                status: Deserialize::default(),
                status_transitions: Deserialize::default(),
                subscription: Deserialize::default(),
                subscription_details: Deserialize::default(),
                subscription_proration_date: Deserialize::default(),
                subtotal: Deserialize::default(),
                subtotal_excluding_tax: Deserialize::default(),
                tax: Deserialize::default(),
                test_clock: Deserialize::default(),
                threshold_reason: Deserialize::default(),
                total: Deserialize::default(),
                total_discount_amounts: Deserialize::default(),
                total_excluding_tax: Deserialize::default(),
                total_tax_amounts: Deserialize::default(),
                transfer_data: Deserialize::default(),
                webhooks_delivered_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                account_country: self.account_country.take()?,
                account_name: self.account_name.take()?,
                account_tax_ids: self.account_tax_ids.take()?,
                amount_due: self.amount_due?,
                amount_paid: self.amount_paid?,
                amount_remaining: self.amount_remaining?,
                amount_shipping: self.amount_shipping?,
                application: self.application.take()?,
                application_fee_amount: self.application_fee_amount?,
                attempt_count: self.attempt_count?,
                attempted: self.attempted?,
                auto_advance: self.auto_advance?,
                automatic_tax: self.automatic_tax.take()?,
                billing_reason: self.billing_reason?,
                charge: self.charge.take()?,
                collection_method: self.collection_method?,
                created: self.created?,
                currency: self.currency?,
                custom_fields: self.custom_fields.take()?,
                customer: self.customer.take()?,
                customer_address: self.customer_address.take()?,
                customer_email: self.customer_email.take()?,
                customer_name: self.customer_name.take()?,
                customer_phone: self.customer_phone.take()?,
                customer_shipping: self.customer_shipping.take()?,
                customer_tax_exempt: self.customer_tax_exempt?,
                customer_tax_ids: self.customer_tax_ids.take()?,
                default_payment_method: self.default_payment_method.take()?,
                default_source: self.default_source.take()?,
                default_tax_rates: self.default_tax_rates.take()?,
                description: self.description.take()?,
                discount: self.discount.take()?,
                discounts: self.discounts.take()?,
                due_date: self.due_date?,
                effective_at: self.effective_at?,
                ending_balance: self.ending_balance?,
                footer: self.footer.take()?,
                from_invoice: self.from_invoice.take()?,
                hosted_invoice_url: self.hosted_invoice_url.take()?,
                id: self.id.take()?,
                invoice_pdf: self.invoice_pdf.take()?,
                issuer: self.issuer.take()?,
                last_finalization_error: self.last_finalization_error.take()?,
                latest_revision: self.latest_revision.take()?,
                lines: self.lines.take()?,
                livemode: self.livemode?,
                metadata: self.metadata.take()?,
                next_payment_attempt: self.next_payment_attempt?,
                number: self.number.take()?,
                on_behalf_of: self.on_behalf_of.take()?,
                paid: self.paid?,
                paid_out_of_band: self.paid_out_of_band?,
                payment_intent: self.payment_intent.take()?,
                payment_settings: self.payment_settings.take()?,
                period_end: self.period_end?,
                period_start: self.period_start?,
                post_payment_credit_notes_amount: self.post_payment_credit_notes_amount?,
                pre_payment_credit_notes_amount: self.pre_payment_credit_notes_amount?,
                quote: self.quote.take()?,
                receipt_number: self.receipt_number.take()?,
                rendering: self.rendering.take()?,
                shipping_cost: self.shipping_cost.take()?,
                shipping_details: self.shipping_details.take()?,
                starting_balance: self.starting_balance?,
                statement_descriptor: self.statement_descriptor.take()?,
                status: self.status?,
                status_transitions: self.status_transitions?,
                subscription: self.subscription.take()?,
                subscription_details: self.subscription_details.take()?,
                subscription_proration_date: self.subscription_proration_date?,
                subtotal: self.subtotal?,
                subtotal_excluding_tax: self.subtotal_excluding_tax?,
                tax: self.tax?,
                test_clock: self.test_clock.take()?,
                threshold_reason: self.threshold_reason.take()?,
                total: self.total?,
                total_discount_amounts: self.total_discount_amounts.take()?,
                total_excluding_tax: self.total_excluding_tax?,
                total_tax_amounts: self.total_tax_amounts.take()?,
                transfer_data: self.transfer_data.take()?,
                webhooks_delivered_at: self.webhooks_delivered_at?,
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

    impl ObjectDeser for Invoice {
        type Builder = InvoiceBuilder;
    }

    impl FromValueOpt for Invoice {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = InvoiceBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "account_country" => b.account_country = Some(FromValueOpt::from_value(v)?),
                    "account_name" => b.account_name = Some(FromValueOpt::from_value(v)?),
                    "account_tax_ids" => b.account_tax_ids = Some(FromValueOpt::from_value(v)?),
                    "amount_due" => b.amount_due = Some(FromValueOpt::from_value(v)?),
                    "amount_paid" => b.amount_paid = Some(FromValueOpt::from_value(v)?),
                    "amount_remaining" => b.amount_remaining = Some(FromValueOpt::from_value(v)?),
                    "amount_shipping" => b.amount_shipping = Some(FromValueOpt::from_value(v)?),
                    "application" => b.application = Some(FromValueOpt::from_value(v)?),
                    "application_fee_amount" => {
                        b.application_fee_amount = Some(FromValueOpt::from_value(v)?)
                    }
                    "attempt_count" => b.attempt_count = Some(FromValueOpt::from_value(v)?),
                    "attempted" => b.attempted = Some(FromValueOpt::from_value(v)?),
                    "auto_advance" => b.auto_advance = Some(FromValueOpt::from_value(v)?),
                    "automatic_tax" => b.automatic_tax = Some(FromValueOpt::from_value(v)?),
                    "billing_reason" => b.billing_reason = Some(FromValueOpt::from_value(v)?),
                    "charge" => b.charge = Some(FromValueOpt::from_value(v)?),
                    "collection_method" => b.collection_method = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "custom_fields" => b.custom_fields = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "customer_address" => b.customer_address = Some(FromValueOpt::from_value(v)?),
                    "customer_email" => b.customer_email = Some(FromValueOpt::from_value(v)?),
                    "customer_name" => b.customer_name = Some(FromValueOpt::from_value(v)?),
                    "customer_phone" => b.customer_phone = Some(FromValueOpt::from_value(v)?),
                    "customer_shipping" => b.customer_shipping = Some(FromValueOpt::from_value(v)?),
                    "customer_tax_exempt" => {
                        b.customer_tax_exempt = Some(FromValueOpt::from_value(v)?)
                    }
                    "customer_tax_ids" => b.customer_tax_ids = Some(FromValueOpt::from_value(v)?),
                    "default_payment_method" => {
                        b.default_payment_method = Some(FromValueOpt::from_value(v)?)
                    }
                    "default_source" => b.default_source = Some(FromValueOpt::from_value(v)?),
                    "default_tax_rates" => b.default_tax_rates = Some(FromValueOpt::from_value(v)?),
                    "description" => b.description = Some(FromValueOpt::from_value(v)?),
                    "discount" => b.discount = Some(FromValueOpt::from_value(v)?),
                    "discounts" => b.discounts = Some(FromValueOpt::from_value(v)?),
                    "due_date" => b.due_date = Some(FromValueOpt::from_value(v)?),
                    "effective_at" => b.effective_at = Some(FromValueOpt::from_value(v)?),
                    "ending_balance" => b.ending_balance = Some(FromValueOpt::from_value(v)?),
                    "footer" => b.footer = Some(FromValueOpt::from_value(v)?),
                    "from_invoice" => b.from_invoice = Some(FromValueOpt::from_value(v)?),
                    "hosted_invoice_url" => {
                        b.hosted_invoice_url = Some(FromValueOpt::from_value(v)?)
                    }
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "invoice_pdf" => b.invoice_pdf = Some(FromValueOpt::from_value(v)?),
                    "issuer" => b.issuer = Some(FromValueOpt::from_value(v)?),
                    "last_finalization_error" => {
                        b.last_finalization_error = Some(FromValueOpt::from_value(v)?)
                    }
                    "latest_revision" => b.latest_revision = Some(FromValueOpt::from_value(v)?),
                    "lines" => b.lines = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "next_payment_attempt" => {
                        b.next_payment_attempt = Some(FromValueOpt::from_value(v)?)
                    }
                    "number" => b.number = Some(FromValueOpt::from_value(v)?),
                    "on_behalf_of" => b.on_behalf_of = Some(FromValueOpt::from_value(v)?),
                    "paid" => b.paid = Some(FromValueOpt::from_value(v)?),
                    "paid_out_of_band" => b.paid_out_of_band = Some(FromValueOpt::from_value(v)?),
                    "payment_intent" => b.payment_intent = Some(FromValueOpt::from_value(v)?),
                    "payment_settings" => b.payment_settings = Some(FromValueOpt::from_value(v)?),
                    "period_end" => b.period_end = Some(FromValueOpt::from_value(v)?),
                    "period_start" => b.period_start = Some(FromValueOpt::from_value(v)?),
                    "post_payment_credit_notes_amount" => {
                        b.post_payment_credit_notes_amount = Some(FromValueOpt::from_value(v)?)
                    }
                    "pre_payment_credit_notes_amount" => {
                        b.pre_payment_credit_notes_amount = Some(FromValueOpt::from_value(v)?)
                    }
                    "quote" => b.quote = Some(FromValueOpt::from_value(v)?),
                    "receipt_number" => b.receipt_number = Some(FromValueOpt::from_value(v)?),
                    "rendering" => b.rendering = Some(FromValueOpt::from_value(v)?),
                    "shipping_cost" => b.shipping_cost = Some(FromValueOpt::from_value(v)?),
                    "shipping_details" => b.shipping_details = Some(FromValueOpt::from_value(v)?),
                    "starting_balance" => b.starting_balance = Some(FromValueOpt::from_value(v)?),
                    "statement_descriptor" => {
                        b.statement_descriptor = Some(FromValueOpt::from_value(v)?)
                    }
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "status_transitions" => {
                        b.status_transitions = Some(FromValueOpt::from_value(v)?)
                    }
                    "subscription" => b.subscription = Some(FromValueOpt::from_value(v)?),
                    "subscription_details" => {
                        b.subscription_details = Some(FromValueOpt::from_value(v)?)
                    }
                    "subscription_proration_date" => {
                        b.subscription_proration_date = Some(FromValueOpt::from_value(v)?)
                    }
                    "subtotal" => b.subtotal = Some(FromValueOpt::from_value(v)?),
                    "subtotal_excluding_tax" => {
                        b.subtotal_excluding_tax = Some(FromValueOpt::from_value(v)?)
                    }
                    "tax" => b.tax = Some(FromValueOpt::from_value(v)?),
                    "test_clock" => b.test_clock = Some(FromValueOpt::from_value(v)?),
                    "threshold_reason" => b.threshold_reason = Some(FromValueOpt::from_value(v)?),
                    "total" => b.total = Some(FromValueOpt::from_value(v)?),
                    "total_discount_amounts" => {
                        b.total_discount_amounts = Some(FromValueOpt::from_value(v)?)
                    }
                    "total_excluding_tax" => {
                        b.total_excluding_tax = Some(FromValueOpt::from_value(v)?)
                    }
                    "total_tax_amounts" => b.total_tax_amounts = Some(FromValueOpt::from_value(v)?),
                    "transfer_data" => b.transfer_data = Some(FromValueOpt::from_value(v)?),
                    "webhooks_delivered_at" => {
                        b.webhooks_delivered_at = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for Invoice {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("Invoice", 82)?;
        s.serialize_field("account_country", &self.account_country)?;
        s.serialize_field("account_name", &self.account_name)?;
        s.serialize_field("account_tax_ids", &self.account_tax_ids)?;
        s.serialize_field("amount_due", &self.amount_due)?;
        s.serialize_field("amount_paid", &self.amount_paid)?;
        s.serialize_field("amount_remaining", &self.amount_remaining)?;
        s.serialize_field("amount_shipping", &self.amount_shipping)?;
        s.serialize_field("application", &self.application)?;
        s.serialize_field("application_fee_amount", &self.application_fee_amount)?;
        s.serialize_field("attempt_count", &self.attempt_count)?;
        s.serialize_field("attempted", &self.attempted)?;
        s.serialize_field("auto_advance", &self.auto_advance)?;
        s.serialize_field("automatic_tax", &self.automatic_tax)?;
        s.serialize_field("billing_reason", &self.billing_reason)?;
        s.serialize_field("charge", &self.charge)?;
        s.serialize_field("collection_method", &self.collection_method)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("custom_fields", &self.custom_fields)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_address", &self.customer_address)?;
        s.serialize_field("customer_email", &self.customer_email)?;
        s.serialize_field("customer_name", &self.customer_name)?;
        s.serialize_field("customer_phone", &self.customer_phone)?;
        s.serialize_field("customer_shipping", &self.customer_shipping)?;
        s.serialize_field("customer_tax_exempt", &self.customer_tax_exempt)?;
        s.serialize_field("customer_tax_ids", &self.customer_tax_ids)?;
        s.serialize_field("default_payment_method", &self.default_payment_method)?;
        s.serialize_field("default_source", &self.default_source)?;
        s.serialize_field("default_tax_rates", &self.default_tax_rates)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("discount", &self.discount)?;
        s.serialize_field("discounts", &self.discounts)?;
        s.serialize_field("due_date", &self.due_date)?;
        s.serialize_field("effective_at", &self.effective_at)?;
        s.serialize_field("ending_balance", &self.ending_balance)?;
        s.serialize_field("footer", &self.footer)?;
        s.serialize_field("from_invoice", &self.from_invoice)?;
        s.serialize_field("hosted_invoice_url", &self.hosted_invoice_url)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice_pdf", &self.invoice_pdf)?;
        s.serialize_field("issuer", &self.issuer)?;
        s.serialize_field("last_finalization_error", &self.last_finalization_error)?;
        s.serialize_field("latest_revision", &self.latest_revision)?;
        s.serialize_field("lines", &self.lines)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("next_payment_attempt", &self.next_payment_attempt)?;
        s.serialize_field("number", &self.number)?;
        s.serialize_field("on_behalf_of", &self.on_behalf_of)?;
        s.serialize_field("paid", &self.paid)?;
        s.serialize_field("paid_out_of_band", &self.paid_out_of_band)?;
        s.serialize_field("payment_intent", &self.payment_intent)?;
        s.serialize_field("payment_settings", &self.payment_settings)?;
        s.serialize_field("period_end", &self.period_end)?;
        s.serialize_field("period_start", &self.period_start)?;
        s.serialize_field(
            "post_payment_credit_notes_amount",
            &self.post_payment_credit_notes_amount,
        )?;
        s.serialize_field(
            "pre_payment_credit_notes_amount",
            &self.pre_payment_credit_notes_amount,
        )?;
        s.serialize_field("quote", &self.quote)?;
        s.serialize_field("receipt_number", &self.receipt_number)?;
        s.serialize_field("rendering", &self.rendering)?;
        s.serialize_field("shipping_cost", &self.shipping_cost)?;
        s.serialize_field("shipping_details", &self.shipping_details)?;
        s.serialize_field("starting_balance", &self.starting_balance)?;
        s.serialize_field("statement_descriptor", &self.statement_descriptor)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("status_transitions", &self.status_transitions)?;
        s.serialize_field("subscription", &self.subscription)?;
        s.serialize_field("subscription_details", &self.subscription_details)?;
        s.serialize_field("subscription_proration_date", &self.subscription_proration_date)?;
        s.serialize_field("subtotal", &self.subtotal)?;
        s.serialize_field("subtotal_excluding_tax", &self.subtotal_excluding_tax)?;
        s.serialize_field("tax", &self.tax)?;
        s.serialize_field("test_clock", &self.test_clock)?;
        s.serialize_field("threshold_reason", &self.threshold_reason)?;
        s.serialize_field("total", &self.total)?;
        s.serialize_field("total_discount_amounts", &self.total_discount_amounts)?;
        s.serialize_field("total_excluding_tax", &self.total_excluding_tax)?;
        s.serialize_field("total_tax_amounts", &self.total_tax_amounts)?;
        s.serialize_field("transfer_data", &self.transfer_data)?;
        s.serialize_field("webhooks_delivered_at", &self.webhooks_delivered_at)?;

        s.serialize_field("object", "invoice")?;
        s.end()
    }
}
/// Indicates the reason why the invoice was created.
///
/// * `manual`: Unrelated to a subscription, for example, created via the invoice editor.
/// * `subscription`: No longer in use.
/// Applies to subscriptions from before May 2018 where no distinction was made between updates, cycles, and thresholds.
/// * `subscription_create`: A new subscription was created.
/// * `subscription_cycle`: A subscription advanced into a new period.
/// * `subscription_threshold`: A subscription reached a billing threshold.
/// * `subscription_update`: A subscription was updated.
/// * `upcoming`: Reserved for simulated invoices, per the upcoming invoice endpoint.
#[derive(Copy, Clone, Eq, PartialEq)]
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
        use InvoiceBillingReason::*;
        match self {
            AutomaticPendingInvoiceItemInvoice => "automatic_pending_invoice_item_invoice",
            Manual => "manual",
            QuoteAccept => "quote_accept",
            Subscription => "subscription",
            SubscriptionCreate => "subscription_create",
            SubscriptionCycle => "subscription_cycle",
            SubscriptionThreshold => "subscription_threshold",
            SubscriptionUpdate => "subscription_update",
            Upcoming => "upcoming",
        }
    }
}

impl std::str::FromStr for InvoiceBillingReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceBillingReason::*;
        match s {
            "automatic_pending_invoice_item_invoice" => Ok(AutomaticPendingInvoiceItemInvoice),
            "manual" => Ok(Manual),
            "quote_accept" => Ok(QuoteAccept),
            "subscription" => Ok(Subscription),
            "subscription_create" => Ok(SubscriptionCreate),
            "subscription_cycle" => Ok(SubscriptionCycle),
            "subscription_threshold" => Ok(SubscriptionThreshold),
            "subscription_update" => Ok(SubscriptionUpdate),
            "upcoming" => Ok(Upcoming),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for InvoiceBillingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceBillingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceBillingReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceBillingReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceBillingReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceBillingReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceBillingReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceBillingReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceBillingReason"))
    }
}
/// The customer's tax exempt status.
/// Until the invoice is finalized, this field will equal `customer.tax_exempt`.
/// Once the invoice is finalized, this field will no longer be updated.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceCustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}
impl InvoiceCustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        use InvoiceCustomerTaxExempt::*;
        match self {
            Exempt => "exempt",
            None => "none",
            Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for InvoiceCustomerTaxExempt {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceCustomerTaxExempt::*;
        match s {
            "exempt" => Ok(Exempt),
            "none" => Ok(None),
            "reverse" => Ok(Reverse),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for InvoiceCustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceCustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for InvoiceCustomerTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceCustomerTaxExempt {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceCustomerTaxExempt> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceCustomerTaxExempt::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceCustomerTaxExempt);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceCustomerTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceCustomerTaxExempt"))
    }
}
impl stripe_types::Object for Invoice {
    type Id = Option<stripe_shared::InvoiceId>;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(InvoiceId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}
impl InvoiceCollectionMethod {
    pub fn as_str(self) -> &'static str {
        use InvoiceCollectionMethod::*;
        match self {
            ChargeAutomatically => "charge_automatically",
            SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for InvoiceCollectionMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceCollectionMethod::*;
        match s {
            "charge_automatically" => Ok(ChargeAutomatically),
            "send_invoice" => Ok(SendInvoice),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for InvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceCollectionMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceCollectionMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceCollectionMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceCollectionMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceCollectionMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceCollectionMethod"))
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}
impl InvoiceStatus {
    pub fn as_str(self) -> &'static str {
        use InvoiceStatus::*;
        match self {
            Draft => "draft",
            Open => "open",
            Paid => "paid",
            Uncollectible => "uncollectible",
            Void => "void",
        }
    }
}

impl std::str::FromStr for InvoiceStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceStatus::*;
        match s {
            "draft" => Ok(Draft),
            "open" => Ok(Open),
            "paid" => Ok(Paid),
            "uncollectible" => Ok(Uncollectible),
            "void" => Ok(Void),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for InvoiceStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<InvoiceStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(InvoiceStatus);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for InvoiceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoiceStatus"))
    }
}
