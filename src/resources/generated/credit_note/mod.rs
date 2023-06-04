/// Issue a credit note to adjust an invoice's amount after the invoice is finalized.
///
/// Related guide: [Credit Notes](https://stripe.com/docs/billing/invoices/credit-notes).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct CreditNote {
    /// The integer amount in %s representing the total amount of the credit note, including tax.
    pub amount: i64,
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// ID of the customer.
    pub customer: crate::Expandable<crate::customer::Customer>,
    /// Customer balance transaction related to this credit note.
    pub customer_balance_transaction:
        Option<crate::Expandable<crate::customer_balance_transaction::CustomerBalanceTransaction>>,
    /// The integer amount in %s representing the total amount of discount that was credited.
    pub discount_amount: i64,
    /// The aggregate amounts calculated per discount for all line items.
    pub discount_amounts: Vec<crate::discount_amount::DiscountAmount>,
    /// Unique identifier for the object.
    pub id: crate::credit_note::CreditNoteId,
    /// ID of the invoice.
    pub invoice: crate::Expandable<crate::invoice::Invoice>,
    /// Line items that make up the credit note.
    pub lines: crate::List<crate::credit_note_line_item::CreditNoteLineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Customer-facing text that appears on the credit note PDF.
    pub memo: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::Metadata>,
    /// A unique number that identifies this particular credit note and appears on the PDF of the credit note and its associated invoice.
    pub number: String,
    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: CreditNoteObject,
    /// Amount that was credited outside of Stripe.
    pub out_of_band_amount: Option<i64>,
    /// The link to download the PDF of the credit note.
    pub pdf: String,
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub reason: Option<CreditNoteReason>,
    /// Refund related to this credit note.
    pub refund: Option<crate::Expandable<crate::refund::Refund>>,
    /// Status of this credit note, one of `issued` or `void`.
    ///
    /// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub status: CreditNoteStatus,
    /// The integer amount in %s representing the amount of the credit note, excluding exclusive tax and invoice level discounts.
    pub subtotal: i64,
    /// The integer amount in %s representing the amount of the credit note, excluding all tax and invoice level discounts.
    pub subtotal_excluding_tax: Option<i64>,
    /// The aggregate amounts calculated per tax rate for all line items.
    pub tax_amounts: Vec<crate::credit_note::tax_amount::TaxAmount>,
    /// The integer amount in %s representing the total amount of the credit note, including tax and all discount.
    pub total: i64,
    /// The integer amount in %s representing the total amount of the credit note, excluding tax, but including discounts.
    pub total_excluding_tax: Option<i64>,
    /// Type of this credit note, one of `pre_payment` or `post_payment`.
    ///
    /// A `pre_payment` credit note means it was issued when the invoice was open.
    /// A `post_payment` credit note means it was issued when the invoice was paid.
    #[serde(rename = "type")]
    pub type_: CreditNoteType,
    /// The time that the credit note was voided.
    pub voided_at: Option<crate::Timestamp>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for CreditNote {
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
pub enum CreditNoteObject {
    CreditNote,
}

impl CreditNoteObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditNote => "credit_note",
        }
    }
}

impl AsRef<str> for CreditNoteObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl CreditNoteReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::OrderChange => "order_change",
            Self::ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl AsRef<str> for CreditNoteReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Status of this credit note, one of `issued` or `void`.
///
/// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteStatus {
    Issued,
    Void,
}

impl CreditNoteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Issued => "issued",
            Self::Void => "void",
        }
    }
}

impl AsRef<str> for CreditNoteStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Type of this credit note, one of `pre_payment` or `post_payment`.
///
/// A `pre_payment` credit note means it was issued when the invoice was open.
/// A `post_payment` credit note means it was issued when the invoice was paid.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
#[cfg_attr(feature = "min-ser", derive(miniserde::Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteType {
    PostPayment,
    PrePayment,
}

impl CreditNoteType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PostPayment => "post_payment",
            Self::PrePayment => "pre_payment",
        }
    }
}

impl AsRef<str> for CreditNoteType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl crate::Object for CreditNote {
    type Id = crate::credit_note::CreditNoteId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
}
crate::def_id!(CreditNoteId);
pub mod requests;
pub mod tax_amount;
pub use tax_amount::TaxAmount;
