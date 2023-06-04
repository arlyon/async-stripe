#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CreditNoteLineItem {
    /// The integer amount in %s representing the gross amount being credited for this line item, excluding (exclusive) tax and discounts.
    pub amount: i64,
    /// The integer amount in %s representing the amount being credited for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,
    /// Description of the item being credited.
    pub description: Option<String>,
    /// The integer amount in %s representing the discount being credited for this line item.
    pub discount_amount: i64,
    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Vec<crate::discount_amount::DiscountAmount>,
    /// Unique identifier for the object.
    pub id: crate::credit_note_line_item::CreditNoteLineItemId,
    /// ID of the invoice line item being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CreditNoteLineItemObject,
    /// The number of units of product being credited.
    pub quantity: Option<u64>,
    /// The amount of tax calculated per tax rate for this line item.
    pub tax_amounts: Vec<crate::credit_note::tax_amount::TaxAmount>,
    /// The tax rates which apply to the line item.
    pub tax_rates: Vec<crate::tax_rate::TaxRate>,
    /// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    ///
    /// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
    #[serde(rename = "type")]
    pub type_: CreditNoteLineItemType,
    /// The cost of each unit of product being credited.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
    /// The amount in %s representing the unit amount being credited for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditNoteLineItem {
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
pub enum CreditNoteLineItemObject {
    CreditNoteLineItem,
}

impl CreditNoteLineItemObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditNoteLineItem => "credit_note_line_item",
        }
    }
}

impl AsRef<str> for CreditNoteLineItemObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteLineItemObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
///
/// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteLineItemType {
    CustomLineItem,
    InvoiceLineItem,
}

impl CreditNoteLineItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomLineItem => "custom_line_item",
            Self::InvoiceLineItem => "invoice_line_item",
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
impl crate::Object for CreditNoteLineItem {
    type Id = crate::credit_note_line_item::CreditNoteLineItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(CreditNoteLineItemId);
pub mod requests;
