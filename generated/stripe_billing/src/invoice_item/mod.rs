/// Sometimes you want to add a charge or credit to a customer, but actually
/// charge or credit the customer's card only at the end of a regular billing
/// cycle.
///
/// This is useful for combining several charges (to minimize per-transaction fees), or for having Stripe tabulate your usage-based billing totals.  Related guide: [Subscription Invoices](https://stripe.com/docs/billing/invoices/subscription#adding-upcoming-invoice-items).
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
    pub customer: stripe_types::Expandable<stripe_types::customer::Customer>,
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
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_types::discount::Discount>>>,
    /// Unique identifier for the object.
    pub id: stripe_billing::invoice_item::InvoiceitemId,
    /// The ID of the invoice this invoice item belongs to.
    pub invoice: Option<stripe_types::Expandable<stripe_types::invoice::Invoice>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: InvoiceItemObject,
    pub period: stripe_types::invoice_line_item_period::InvoiceLineItemPeriod,
    /// If the invoice item is a proration, the plan of the subscription that the proration was computed for.
    pub plan: Option<stripe_types::plan::Plan>,
    /// The price of the invoice item.
    pub price: Option<stripe_types::price::Price>,
    /// Whether the invoice item was created automatically as a proration adjustment when the customer switched plans.
    pub proration: bool,
    /// Quantity of units for the invoice item.
    ///
    /// If the invoice item is a proration, the quantity of the subscription that the proration was computed for.
    pub quantity: u64,
    /// The subscription that this invoice item has been created for, if any.
    pub subscription: Option<stripe_types::Expandable<stripe_types::subscription::Subscription>>,
    /// The subscription item that this invoice item has been created for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,
    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    pub tax_rates: Option<Vec<stripe_types::tax_rate::TaxRate>>,
    /// ID of the test clock this invoice item belongs to.
    pub test_clock:
        Option<stripe_types::Expandable<stripe_types::test_helpers::test_clock::TestClock>>,
    /// Unit amount (in the `currency` specified) of the invoice item.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
}
/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InvoiceItemObject {
    Invoiceitem,
}

impl InvoiceItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoiceitem => "invoiceitem",
        }
    }
}

impl std::str::FromStr for InvoiceItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "invoiceitem" => Ok(Self::Invoiceitem),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for InvoiceItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for InvoiceItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceItemObject"))
    }
}
impl stripe_types::Object for InvoiceItem {
    type Id = stripe_billing::invoice_item::InvoiceitemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(InvoiceitemId, "ii_");
pub mod deleted;
pub use deleted::DeletedInvoiceItem;
