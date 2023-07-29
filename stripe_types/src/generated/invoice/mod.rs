/// Invoices are statements of amounts owed by a customer, and are either
/// generated one-off, or generated periodically from a subscription.
///
/// They contain [invoice items](https://stripe.com/docs/api#invoiceitems), and proration adjustments
/// that may be caused by subscription upgrades/downgrades (if necessary).
///
/// If your invoice is configured to be billed through automatic charges,
/// Stripe automatically finalizes your invoice and attempts payment.
///
/// Note that finalizing the invoice, [when automatic](https://stripe.com/docs/billing/invoices/workflow/#auto_advance), does not happen immediately as the invoice is created.
/// Stripe waits until one hour after the last webhook was successfully sent (or the last webhook timed out after failing).
/// If you (and the platforms you may have connected to) have no webhooks configured, Stripe waits one hour after creation to finalize the invoice.  If your invoice is configured to be billed by sending an email, then based on your [email settings](https://dashboard.stripe.com/account/billing/automatic), Stripe will email the invoice to your customer and await payment.
/// These emails can contain a link to a hosted page to pay the invoice.  Stripe applies any customer credit on the account before determining the amount due for the invoice (i.e., the amount that will be actually charged).
/// If the amount due for the invoice is less than Stripe's [minimum allowed charge per currency](/docs/currencies#minimum-and-maximum-charge-amounts), the invoice is automatically marked paid, and we add the amount due to the customer's credit balance which is applied to the next invoice.  More details on the customer's credit balance are [here](https://stripe.com/docs/billing/customer/balance).  Related guide: [Send Invoices to Customers](https://stripe.com/docs/billing/invoices/sending).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Invoice {
    /// The country of the business associated with this invoice, most often the business creating the invoice.
    pub account_country: Option<String>,
    /// The public name of the business associated with this invoice, most often the business creating the invoice.
    pub account_name: Option<String>,
    /// The account tax IDs associated with the invoice.
    ///
    /// Only editable when the invoice is a draft.
    pub account_tax_ids: Option<Vec<stripe_types::Expandable<stripe_types::tax_id::TaxId>>>,
    /// Final amount due at this time for this invoice.
    ///
    /// If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0.
    /// If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account.
    /// The charge that gets generated for the invoice will be for the amount specified in `amount_due`.
    pub amount_due: i64,
    /// The amount, in %s, that was paid.
    pub amount_paid: i64,
    /// The difference between amount_due and amount_paid, in %s.
    pub amount_remaining: i64,
    /// ID of the Connect Application that created the invoice.
    pub application: Option<stripe_types::Expandable<stripe_types::application::Application>>,
    /// The fee in %s that will be applied to the invoice and transferred to the application owner's Stripe account when the invoice is paid.
    pub application_fee_amount: Option<i64>,
    /// Number of payment attempts made for this invoice, from the perspective of the payment retry schedule.
    ///
    /// Any payment attempt counts as the first attempt, and subsequently only automatic retries increment the attempt count.
    /// In other words, manual payment attempts after the first attempt do not affect the retry schedule.
    pub attempt_count: u64,
    /// Whether an attempt has been made to pay the invoice.
    ///
    /// An invoice is not attempted until 1 hour after the `invoice.created` webhook, for example, so you might not want to display that invoice as unpaid to your users.
    pub attempted: bool,
    /// Controls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice.
    ///
    /// When `false`, the invoice's state will not automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,
    pub automatic_tax: stripe_types::invoice::automatic_tax::AutomaticTax,
    /// Indicates the reason why the invoice was created.
    ///
    /// `subscription_cycle` indicates an invoice created by a subscription advancing into a new period.
    /// `subscription_create` indicates an invoice created due to creating a subscription.
    /// `subscription_update` indicates an invoice created due to updating a subscription.
    /// `subscription` is set for all old invoices to indicate either a change to a subscription or a period advancement.
    /// `manual` is set for all invoices unrelated to a subscription (for example: created via the invoice editor).
    /// The `upcoming` value is reserved for simulated invoices per the upcoming invoice endpoint.
    /// `subscription_threshold` indicates an invoice created due to a billing threshold being reached.
    pub billing_reason: Option<InvoiceBillingReason>,
    /// ID of the latest charge generated for this invoice, if any.
    pub charge: Option<stripe_types::Expandable<stripe_types::charge::Charge>>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    pub collection_method: InvoiceCollectionMethod,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Custom fields displayed on the invoice.
    pub custom_fields: Option<Vec<stripe_types::invoice::custom_field::CustomField>>,
    /// The ID of the customer who will be billed.
    pub customer: Option<stripe_types::Expandable<stripe_types::customer::Customer>>,
    /// The customer's address.
    ///
    /// Until the invoice is finalized, this field will equal `customer.address`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_address: Option<stripe_types::address::Address>,
    /// The customer's email.
    ///
    /// Until the invoice is finalized, this field will equal `customer.email`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_email: Option<String>,
    /// The customer's name.
    ///
    /// Until the invoice is finalized, this field will equal `customer.name`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_name: Option<String>,
    /// The customer's phone number.
    ///
    /// Until the invoice is finalized, this field will equal `customer.phone`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_phone: Option<String>,
    /// The customer's shipping information.
    ///
    /// Until the invoice is finalized, this field will equal `customer.shipping`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_shipping: Option<stripe_types::shipping_details::ShippingDetails>,
    /// The customer's tax exempt status.
    ///
    /// Until the invoice is finalized, this field will equal `customer.tax_exempt`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_tax_exempt: Option<InvoiceCustomerTaxExempt>,
    /// The customer's tax IDs.
    ///
    /// Until the invoice is finalized, this field will contain the same tax IDs as `customer.tax_ids`.
    /// Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tax_ids: Option<Vec<stripe_types::invoice::customer_tax_id::CustomerTaxId>>,
    /// ID of the default payment method for the invoice.
    ///
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    pub default_payment_method:
        Option<stripe_types::Expandable<stripe_types::payment_method::PaymentMethod>>,
    /// ID of the default payment source for the invoice.
    ///
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    pub default_source:
        Option<stripe_types::Expandable<stripe_types::payment_source::PaymentSource>>,
    /// The tax rates applied to this invoice, if any.
    pub default_tax_rates: Vec<stripe_types::tax_rate::TaxRate>,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    pub description: Option<String>,
    /// Describes the current discount applied to this invoice, if there is one.
    ///
    /// Not populated if there are multiple discounts.
    pub discount: Option<stripe_types::discount::Discount>,
    /// The discounts applied to the invoice.
    ///
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_types::discount::Discount>>>,
    /// The date on which payment for this invoice is due.
    ///
    /// This value will be `null` for invoices where `collection_method=charge_automatically`.
    pub due_date: Option<stripe_types::Timestamp>,
    /// Ending customer balance after the invoice is finalized.
    ///
    /// Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice.
    /// If the invoice has not been finalized yet, this will be null.
    pub ending_balance: Option<i64>,
    /// Footer displayed on the invoice.
    pub footer: Option<String>,
    /// Details of the invoice that was cloned.
    ///
    /// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    pub from_invoice: Option<stripe_types::invoice::from_invoice::FromInvoice>,
    /// The URL for the hosted invoice page, which allows customers to view and pay an invoice.
    ///
    /// If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_invoice_url: Option<String>,
    /// Unique identifier for the object.
    ///
    /// This property is always present unless the invoice is an upcoming invoice.
    /// See [Retrieve an upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<stripe_types::invoice::InvoiceId>,
    /// The link to download the PDF for the invoice.
    ///
    /// If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_pdf: Option<String>,
    /// The error encountered during the previous attempt to finalize the invoice.
    ///
    /// This field is cleared when the invoice is successfully finalized.
    pub last_finalization_error: Option<Box<stripe_types::api_errors::ApiErrors>>,
    /// The ID of the most recent non-draft revision of this invoice.
    pub latest_revision: Option<stripe_types::Expandable<stripe_types::invoice::Invoice>>,
    /// The individual line items that make up the invoice.
    ///
    /// `lines` is sorted as follows: invoice items in reverse chronological order, followed by the subscription, if any.
    pub lines: stripe_types::List<stripe_types::invoice_line_item::InvoiceLineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The time at which payment will next be attempted.
    ///
    /// This value will be `null` for invoices where `collection_method=send_invoice`.
    pub next_payment_attempt: Option<stripe_types::Timestamp>,
    /// A unique, identifying string that appears on emails sent to the customer for this invoice.
    ///
    /// This starts with the customer's unique invoice_prefix if it is specified.
    pub number: Option<String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: InvoiceObject,
    /// The account (if any) for which the funds of the invoice payment are intended.
    ///
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub on_behalf_of: Option<stripe_types::Expandable<stripe_types::account::Account>>,
    /// Whether payment was successfully collected for this invoice.
    ///
    /// An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance.
    pub paid: bool,
    /// Returns true if the invoice was manually marked paid, returns false if the invoice hasn't been paid yet or was paid on Stripe.
    pub paid_out_of_band: bool,
    /// The PaymentIntent associated with this invoice.
    ///
    /// The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice.
    /// Note that voiding an invoice will cancel the PaymentIntent.
    pub payment_intent:
        Option<stripe_types::Expandable<stripe_types::payment_intent::PaymentIntent>>,
    pub payment_settings: stripe_types::invoice::payment_settings::PaymentSettings,
    /// End of the usage period during which invoice items were added to this invoice.
    pub period_end: stripe_types::Timestamp,
    /// Start of the usage period during which invoice items were added to this invoice.
    pub period_start: stripe_types::Timestamp,
    /// Total amount of all post-payment credit notes issued for this invoice.
    pub post_payment_credit_notes_amount: i64,
    /// Total amount of all pre-payment credit notes issued for this invoice.
    pub pre_payment_credit_notes_amount: i64,
    /// The quote this invoice was generated from.
    pub quote: Option<stripe_types::Expandable<stripe_types::quote::Quote>>,
    /// This is the transaction number that appears on email receipts sent for this invoice.
    pub receipt_number: Option<String>,
    /// Options for invoice PDF rendering.
    pub rendering_options: Option<stripe_types::invoice::rendering_options::RenderingOptions>,
    /// Starting customer balance before the invoice is finalized.
    ///
    /// If the invoice has not been finalized yet, this will be the current customer balance.
    /// For revision invoices, this also includes any customer balance that was applied to the original invoice.
    pub starting_balance: i64,
    /// Extra information about an invoice for the customer's credit card statement.
    pub statement_descriptor: Option<String>,
    /// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
    ///
    /// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    pub status: Option<InvoiceStatus>,
    pub status_transitions: stripe_types::invoice::status_transitions::StatusTransitions,
    /// The subscription that this invoice was prepared for, if any.
    pub subscription: Option<stripe_types::Expandable<stripe_types::subscription::Subscription>>,
    /// Only set for upcoming invoices that preview prorations.
    ///
    /// The time used to calculate prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<stripe_types::Timestamp>,
    /// Total of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusive tax is applied.
    ///
    /// Item discounts are already incorporated.
    pub subtotal: i64,
    /// The integer amount in %s representing the subtotal of the invoice before any invoice level discount or tax is applied.
    ///
    /// Item discounts are already incorporated.
    pub subtotal_excluding_tax: Option<i64>,
    /// The amount of tax on this invoice.
    ///
    /// This is the sum of all the tax amounts on this invoice.
    pub tax: Option<i64>,
    /// ID of the test clock this invoice belongs to.
    pub test_clock:
        Option<stripe_types::Expandable<stripe_types::test_helpers::test_clock::TestClock>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_reason: Option<stripe_types::invoice::threshold_reason::ThresholdReason>,
    /// Total after discounts and taxes.
    pub total: i64,
    /// The aggregate amounts calculated per discount across all line items.
    pub total_discount_amounts: Option<Vec<stripe_types::discount_amount::DiscountAmount>>,
    /// The integer amount in %s representing the total amount of the invoice including all discounts but excluding all tax.
    pub total_excluding_tax: Option<i64>,
    /// The aggregate amounts calculated per tax rate for all line items.
    pub total_tax_amounts: Vec<stripe_types::invoice::tax_amount::TaxAmount>,
    /// The account (if any) the payment will be attributed to for tax reporting, and where funds from the payment will be transferred to for the invoice.
    pub transfer_data: Option<stripe_types::invoice::transfer_data::TransferData>,
    /// Invoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts have [been exhausted](https://stripe.com/docs/billing/webhooks#understand).
    ///
    /// This field tracks the time when webhooks for this invoice were successfully delivered.
    /// If the invoice had no webhooks to deliver, this will be set while the invoice is being created.
    pub webhooks_delivered_at: Option<stripe_types::Timestamp>,
}
/// Indicates the reason why the invoice was created.
///
/// `subscription_cycle` indicates an invoice created by a subscription advancing into a new period.
/// `subscription_create` indicates an invoice created due to creating a subscription.
/// `subscription_update` indicates an invoice created due to updating a subscription.
/// `subscription` is set for all old invoices to indicate either a change to a subscription or a period advancement.
/// `manual` is set for all invoices unrelated to a subscription (for example: created via the invoice editor).
/// The `upcoming` value is reserved for simulated invoices per the upcoming invoice endpoint.
/// `subscription_threshold` indicates an invoice created due to a billing threshold being reached.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::AutomaticPendingInvoiceItemInvoice => "automatic_pending_invoice_item_invoice",
            Self::Manual => "manual",
            Self::QuoteAccept => "quote_accept",
            Self::Subscription => "subscription",
            Self::SubscriptionCreate => "subscription_create",
            Self::SubscriptionCycle => "subscription_cycle",
            Self::SubscriptionThreshold => "subscription_threshold",
            Self::SubscriptionUpdate => "subscription_update",
            Self::Upcoming => "upcoming",
        }
    }
}

impl std::str::FromStr for InvoiceBillingReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "automatic_pending_invoice_item_invoice" => {
                Ok(Self::AutomaticPendingInvoiceItemInvoice)
            }
            "manual" => Ok(Self::Manual),
            "quote_accept" => Ok(Self::QuoteAccept),
            "subscription" => Ok(Self::Subscription),
            "subscription_create" => Ok(Self::SubscriptionCreate),
            "subscription_cycle" => Ok(Self::SubscriptionCycle),
            "subscription_threshold" => Ok(Self::SubscriptionThreshold),
            "subscription_update" => Ok(Self::SubscriptionUpdate),
            "upcoming" => Ok(Self::Upcoming),

            _ => Err(()),
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
impl serde::Serialize for InvoiceBillingReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceBillingReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceBillingReason"))
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
/// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InvoiceCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl InvoiceCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl std::str::FromStr for InvoiceCollectionMethod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "charge_automatically" => Ok(Self::ChargeAutomatically),
            "send_invoice" => Ok(Self::SendInvoice),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for InvoiceCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
impl<'de> serde::Deserialize<'de> for InvoiceCollectionMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceCollectionMethod"))
    }
}
/// The customer's tax exempt status.
///
/// Until the invoice is finalized, this field will equal `customer.tax_exempt`.
/// Once the invoice is finalized, this field will no longer be updated.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InvoiceCustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl InvoiceCustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl std::str::FromStr for InvoiceCustomerTaxExempt {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exempt" => Ok(Self::Exempt),
            "none" => Ok(Self::None),
            "reverse" => Ok(Self::Reverse),

            _ => Err(()),
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
impl serde::Serialize for InvoiceCustomerTaxExempt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceCustomerTaxExempt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceCustomerTaxExempt"))
    }
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InvoiceObject {
    Invoice,
}

impl InvoiceObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoice => "invoice",
        }
    }
}

impl std::str::FromStr for InvoiceObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "invoice" => Ok(Self::Invoice),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for InvoiceObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for InvoiceObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoiceObject"))
    }
}
/// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
///
/// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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
            Self::Deleted => "deleted",
            Self::Draft => "draft",
            Self::Open => "open",
            Self::Paid => "paid",
            Self::Uncollectible => "uncollectible",
            Self::Void => "void",
        }
    }
}

impl std::str::FromStr for InvoiceStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deleted" => Ok(Self::Deleted),
            "draft" => Ok(Self::Draft),
            "open" => Ok(Self::Open),
            "paid" => Ok(Self::Paid),
            "uncollectible" => Ok(Self::Uncollectible),
            "void" => Ok(Self::Void),

            _ => Err(()),
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
impl serde::Serialize for InvoiceStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for InvoiceStatus"))
    }
}
impl stripe_types::Object for Invoice {
    type Id = Option<stripe_types::invoice::InvoiceId>;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(InvoiceId, "in_");
pub mod deleted;
pub use deleted::DeletedInvoice;
pub mod automatic_tax;
pub use automatic_tax::AutomaticTax;
pub mod threshold_item_reason;
pub use threshold_item_reason::ThresholdItemReason;
pub mod custom_field;
pub use custom_field::CustomField;
pub mod rendering_options;
pub use rendering_options::RenderingOptions;
pub mod tax_amount;
pub use tax_amount::TaxAmount;
pub mod threshold_reason;
pub use threshold_reason::ThresholdReason;
pub mod transfer_data;
pub use transfer_data::TransferData;
pub mod from_invoice;
pub use from_invoice::FromInvoice;
pub mod payment_method_options;
pub use payment_method_options::PaymentMethodOptions;
pub mod payment_settings;
pub use payment_settings::PaymentSettings;
pub mod customer_tax_id;
pub use customer_tax_id::CustomerTaxId;
pub mod status_transitions;
pub use status_transitions::StatusTransitions;
