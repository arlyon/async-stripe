#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InvoiceLineItem {
    /// The amount, in cents (or local equivalent).
    pub amount: i64,
    /// The integer amount in cents (or local equivalent) representing the amount for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Option<Vec<stripe_types::DiscountsResourceDiscountAmount>>,
    /// If true, discounts will apply to this line item.
    ///
    /// Always false for prorations.
    pub discountable: bool,
    /// The discounts applied to the invoice line item.
    ///
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_types::Discount>>>,
    /// Unique identifier for the object.
    pub id: stripe_types::invoice_line_item::LineItemId,
    /// The ID of the [invoice item](https://stripe.com/docs/api/invoiceitems) associated with this line item if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<stripe_types::Expandable<stripe_types::InvoiceItem>>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription` this will reflect the metadata of the subscription that caused the line item to be created.
    pub metadata: std::collections::HashMap<String, String>,
    pub period: stripe_types::InvoiceLineItemPeriod,
    /// The plan of the subscription, if the line item is a subscription or a proration.
    pub plan: Option<stripe_types::Plan>,
    /// The price of the line item.
    pub price: Option<stripe_types::Price>,
    /// Whether this is a proration.
    pub proration: bool,
    /// Additional details for proration line items.
    pub proration_details: Option<stripe_types::InvoicesResourceLineItemsProrationDetails>,
    /// The quantity of the subscription, if the line item is a subscription or a proration.
    pub quantity: Option<u64>,
    /// The subscription that the invoice item pertains to, if any.
    pub subscription: Option<stripe_types::Expandable<stripe_types::Subscription>>,
    /// The subscription item that generated this line item.
    ///
    /// Left empty if the line item is not an explicit result of a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<stripe_types::Expandable<stripe_types::SubscriptionItem>>,
    /// The amount of tax calculated per tax rate for this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<stripe_types::InvoiceTaxAmount>>,
    /// The tax rates which apply to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<stripe_types::TaxRate>>,
    /// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
    #[serde(rename = "type")]
    pub type_: InvoiceLineItemType,
    /// The amount in cents (or local equivalent) representing the unit amount for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}
/// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum InvoiceLineItemType {
    Invoiceitem,
    Subscription,
}

impl InvoiceLineItemType {
    pub fn as_str(self) -> &'static str {
        use InvoiceLineItemType::*;
        match self {
            Invoiceitem => "invoiceitem",
            Subscription => "subscription",
        }
    }
}

impl std::str::FromStr for InvoiceLineItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use InvoiceLineItemType::*;
        match s {
            "invoiceitem" => Ok(Invoiceitem),
            "subscription" => Ok(Subscription),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for InvoiceLineItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for InvoiceLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for InvoiceLineItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceLineItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceLineItemType"))
    }
}
impl stripe_types::Object for InvoiceLineItem {
    type Id = stripe_types::invoice_line_item::LineItemId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(LineItemId);
