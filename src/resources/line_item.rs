// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::InvoiceLineItemId;
use crate::params::{Expandable, Metadata, Object};
use crate::resources::{Currency, Period, Plan, TaxRate};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoiceLineItem".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceLineItem {
    /// Unique identifier for the object.
    pub id: InvoiceLineItemId,

    /// The amount, in %s.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// If true, discounts will apply to this line item.
    ///
    /// Always false for prorations.
    pub discountable: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,

    /// Whether this is a test line item.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription` this will reflect the metadata of the subscription that caused the line item to be created.
    pub metadata: Metadata,

    pub period: Option<Period>,

    /// The plan of the subscription, if the line item is a subscription or a proration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,

    /// Whether this is a proration.
    pub proration: bool,

    /// The quantity of the subscription, if the line item is a subscription or a proration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The subscription that the invoice item pertains to, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    /// The subscription item that generated this invoice item.
    ///
    /// Left empty if the line item is not an explicit result of a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,

    /// The amount of tax calculated per tax rate for this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<TaxAmount>>,

    /// The tax rates which apply to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,

    /// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
    #[serde(rename = "type")]
    pub type_: InvoiceLineItemType,
}

impl Object for InvoiceLineItem {
    type Id = InvoiceLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "line_item"
    }
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

/// An enum representing the possible values of an `InvoiceLineItem`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceLineItemType {
    #[serde(rename = "invoiceitem")]
    InvoiceItem,
    Subscription,
}
