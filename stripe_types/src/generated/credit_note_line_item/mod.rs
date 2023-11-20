/// The credit note line item object.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreditNoteLineItem {
    /// The integer amount in cents (or local equivalent) representing the gross amount being credited for this line item, excluding (exclusive) tax and discounts.
    pub amount: i64,
    /// The integer amount in cents (or local equivalent) representing the amount being credited for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,
    /// Description of the item being credited.
    pub description: Option<String>,
    /// The integer amount in cents (or local equivalent) representing the discount being credited for this line item.
    pub discount_amount: i64,
    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Vec<stripe_types::DiscountsResourceDiscountAmount>,
    /// Unique identifier for the object.
    pub id: stripe_types::credit_note_line_item::CreditNoteLineItemId,
    /// ID of the invoice line item being credited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The number of units of product being credited.
    pub quantity: Option<u64>,
    /// The amount of tax calculated per tax rate for this line item.
    pub tax_amounts: Vec<stripe_types::CreditNoteTaxAmount>,
    /// The tax rates which apply to the line item.
    pub tax_rates: Vec<stripe_types::TaxRate>,
    /// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    ///
    /// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
    #[serde(rename = "type")]
    pub type_: CreditNoteLineItemType,
    /// The cost of each unit of product being credited.
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
    /// The amount in cents (or local equivalent) representing the unit amount being credited for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}
/// The type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
///
/// When the type is `invoice_line_item` there is an additional `invoice_line_item` property on the resource the value of which is the id of the credited line item on the invoice.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreditNoteLineItemType {
    CustomLineItem,
    InvoiceLineItem,
}

impl CreditNoteLineItemType {
    pub fn as_str(self) -> &'static str {
        use CreditNoteLineItemType::*;
        match self {
            CustomLineItem => "custom_line_item",
            InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl std::str::FromStr for CreditNoteLineItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteLineItemType::*;
        match s {
            "custom_line_item" => Ok(CustomLineItem),
            "invoice_line_item" => Ok(InvoiceLineItem),
            _ => Err(()),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteLineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreditNoteLineItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditNoteLineItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteLineItemType"))
    }
}
impl stripe_types::Object for CreditNoteLineItem {
    type Id = stripe_types::credit_note_line_item::CreditNoteLineItemId;
    fn id(&self) -> Option<&str> {
        Some(self.id.as_str())
    }
}
stripe_types::def_id!(CreditNoteLineItemId, "cnli_");
