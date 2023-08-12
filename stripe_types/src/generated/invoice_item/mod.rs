/// Invoice Items represent the component lines of an [invoice](https://stripe.com/docs/api/invoices).
///
/// An invoice item is added to an invoice by creating or updating it with an `invoice` field, at which point it will be included as [an invoice line item](https://stripe.com/docs/api/invoices/line_item) within [invoice.lines](https://stripe.com/docs/api/invoices/object#invoice_object-lines).  Invoice Items can be created before you are ready to actually send the invoice.
/// This can be particularly useful when combined with a [subscription](https://stripe.com/docs/api/subscriptions).
/// Sometimes you want to add a charge or credit to a customer, but actually charge or credit the customer’s card only at the end of a regular billing cycle.
/// This is useful for combining several charges (to minimize per-transaction fees), or for having Stripe tabulate your usage-based billing totals.  Related guides: [Integrate with the Invoicing API](https://stripe.com/docs/invoicing/integration), [Subscription Invoices](https://stripe.com/docs/billing/invoices/subscription#adding-upcoming-invoice-items).
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceItem {
    /// Amount (in the `currency` specified) of the invoice item.
    ///
    /// This should always be equal to `unit_amount * quantity`.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the customer who will be billed when this invoice item is billed.
    pub customer: stripe_types::Expandable<stripe_types::Customer>,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub date: stripe_types::Timestamp,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// If true, discounts will apply to this invoice item.
    ///
    /// Always false for prorations.
    pub discountable: bool,
    /// The discounts which apply to the invoice item.
    ///
    /// Item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_types::Discount>>>,
    /// Unique identifier for the object.
    pub id: stripe_types::invoice_item::InvoiceitemId,
    /// The ID of the invoice this invoice item belongs to.
    pub invoice: Option<stripe_types::Expandable<stripe_types::Invoice>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    pub period: stripe_types::InvoiceLineItemPeriod,
    /// If the invoice item is a proration, the plan of the subscription that the proration was computed for.
    pub plan: Option<stripe_types::Plan>,
    /// The price of the invoice item.
    pub price: Option<stripe_types::Price>,
    /// Whether the invoice item was created automatically as a proration adjustment when the customer switched plans.
    pub proration: bool,
    /// Quantity of units for the invoice item.
    ///
    /// If the invoice item is a proration, the quantity of the subscription that the proration was computed for.
    pub quantity: u64,
    /// The subscription that this invoice item has been created for, if any.
    pub subscription: Option<stripe_types::Expandable<stripe_types::Subscription>>,
    /// The subscription item that this invoice item has been created for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,
    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    pub tax_rates: Option<Vec<stripe_types::TaxRate>>,
    /// ID of the test clock this invoice item belongs to.
    pub test_clock: Option<stripe_types::Expandable<stripe_types::TestClock>>,
    /// Unit amount (in the `currency` specified) of the invoice item.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
}
impl stripe_types::Object for InvoiceItem {
    type Id = stripe_types::invoice_item::InvoiceitemId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(InvoiceitemId, "ii_");
