#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct InvoiceLineItem {
    /// The amount, in %s.
    pub amount: i64,
    /// The integer amount in %s representing the amount for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Option<Vec<crate::discount_amount::DiscountAmount>>,
    /// If true, discounts will apply to this line item.
    ///
    /// Always false for prorations.
    pub discountable: bool,
    /// The discounts applied to the invoice line item.
    ///
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<crate::Expandable<crate::discount::Discount>>>,
    /// Unique identifier for the object.
    pub id: crate::invoice_line_item::LineItemId,
    /// The ID of the [invoice item](https://stripe.com/docs/api/invoiceitems) associated with this line item if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription` this will reflect the metadata of the subscription that caused the line item to be created.
    pub metadata: crate::Metadata,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: InvoiceLineItemObject,
    pub period: crate::invoice_line_item_period::InvoiceLineItemPeriod,
    /// The plan of the subscription, if the line item is a subscription or a proration.
    pub plan: Option<crate::plan::Plan>,
    /// The price of the line item.
    pub price: Option<crate::price::Price>,
    /// Whether this is a proration.
    pub proration: bool,
    /// Additional details for proration line items.
    pub proration_details: Option<crate::proration_details::ProrationDetails>,
    /// The quantity of the subscription, if the line item is a subscription or a proration.
    pub quantity: Option<u64>,
    /// The subscription that the invoice item pertains to, if any.
    pub subscription: Option<String>,
    /// The subscription item that generated this invoice item.
    ///
    /// Left empty if the line item is not an explicit result of a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,
    /// The amount of tax calculated per tax rate for this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<crate::invoice::tax_amount::TaxAmount>>,
    /// The tax rates which apply to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<crate::tax_rate::TaxRate>>,
    /// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
    #[serde(rename = "type")]
    pub type_: InvoiceLineItemType,
    /// The amount in %s representing the unit amount for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceLineItem {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

/// String representing the object's type.
///
/// Objects of the same type share the same value.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum InvoiceLineItemObject {
    LineItem,
}

impl InvoiceLineItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LineItem => "line_item",
        }
    }
}

impl AsRef<str> for InvoiceLineItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceLineItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum InvoiceLineItemType {
    Invoiceitem,
    Subscription,
}

impl InvoiceLineItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoiceitem => "invoiceitem",
            Self::Subscription => "subscription",
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
        self.as_str().fmt(f)
    }
}
impl crate::Object for InvoiceLineItem {
    type Id = crate::invoice_line_item::LineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(LineItemId);
pub mod requests;
