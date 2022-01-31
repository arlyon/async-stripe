// ======================================
// This file was automatically generated.
// ======================================

use serde_derive::{Deserialize, Serialize};

use crate::ids::CreditNoteLineItemId;
use crate::params::{Expandable, Object};
use crate::resources::{Discount, TaxRate};

/// The resource representing a Stripe "CreditNoteLineItem".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreditNoteLineItem {
    /// Unique identifier for the object.
    pub id: CreditNoteLineItemId,

    /// The integer amount in %s representing the gross amount being credited for this line item, excluding (exclusive) tax and discounts.
    pub amount: i64,

    /// Description of the item being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<String>>,

    /// The integer amount in %s representing the discount being credited for this line item.
    pub discount_amount: i64,

    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,

    /// ID of the invoice line item being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<Box<String>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The number of units of product being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Box<u64>>,

    /// The amount of tax calculated per tax rate for this line item.
    pub tax_amounts: Vec<CreditNoteTaxAmount>,

    /// The tax rates which apply to the line item.
    pub tax_rates: Vec<TaxRate>,

    /// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    ///
    /// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
    #[serde(rename = "type")]
    pub type_: CreditNoteLineItemType,

    /// The cost of each unit of product being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<Box<i64>>,

    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<Box<String>>,
}



#[derive(Clone, Debug, Deserialize, Serialize)]


#[derive(Clone, Debug, Deserialize, Serialize)]


/// An enum representing the possible values of an `CreditNoteLineItem`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteLineItemType {
    CustomLineItem,
    InvoiceLineItem,
}

impl CreditNoteLineItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            CreditNoteLineItemType::CustomLineItem => "custom_line_item",
            CreditNoteLineItemType::InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl AsRef<str> for CreditNoteLineItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Object for CreditNoteLineItem {
    type Id = CreditNoteLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "credit_note_line_item"
    }
}
