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
    pub currency: stripe_types::Currency,
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,
    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Option<Vec<stripe_core::discount_amount::DiscountAmount>>,
    /// If true, discounts will apply to this line item.
    ///
    /// Always false for prorations.
    pub discountable: bool,
    /// The discounts applied to the invoice line item.
    ///
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<stripe_types::Expandable<stripe_core::discount::Discount>>>,
    /// Unique identifier for the object.
    pub id: stripe_core::invoice_line_item::LineItemId,
    /// The ID of the [invoice item](https://stripe.com/docs/api/invoiceitems) associated with this line item if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription` this will reflect the metadata of the subscription that caused the line item to be created.
    pub metadata: std::collections::HashMap<String, String>,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: InvoiceLineItemObject,
    pub period: stripe_types::invoice_line_item_period::InvoiceLineItemPeriod,
    /// The plan of the subscription, if the line item is a subscription or a proration.
    pub plan: Option<stripe_core::plan::Plan>,
    /// The price of the line item.
    pub price: Option<stripe_core::price::Price>,
    /// Whether this is a proration.
    pub proration: bool,
    /// Additional details for proration line items.
    pub proration_details: Option<stripe_core::proration_details::ProrationDetails>,
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
    pub tax_amounts: Option<Vec<stripe_core::invoice::tax_amount::TaxAmount>>,
    /// The tax rates which apply to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<stripe_core::tax_rate::TaxRate>>,
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
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for InvoiceLineItemObject {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "line_item" => Ok(Self::LineItem),

            _ => Err(()),
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
impl serde::Serialize for InvoiceLineItemObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InvoiceLineItemObject {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceLineItemObject"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceLineItemObject {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<InvoiceLineItemObject> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceLineItemObject::from_str(s)?);
        Ok(())
    }
}
/// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

impl std::str::FromStr for InvoiceLineItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "invoiceitem" => Ok(Self::Invoiceitem),
            "subscription" => Ok(Self::Subscription),

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
        self.as_str().fmt(f)
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
        let s: String = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for InvoiceLineItemType"))
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for InvoiceLineItemType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::Visitor {
        Place::new(out)
    }
}

#[cfg(feature = "min-ser")]
impl miniserde::Visitor for crate::Place<InvoiceLineItemType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(InvoiceLineItemType::from_str(s)?);
        Ok(())
    }
}
impl stripe_types::Object for InvoiceLineItem {
    type Id = stripe_core::invoice_line_item::LineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
stripe_types::def_id!(LineItemId);
pub mod requests;
