/// Issue a credit note to adjust an invoice's amount after the invoice is finalized.
///
/// Related guide: [Credit notes](https://stripe.com/docs/billing/invoices/credit-notes)
///
/// For more details see <<https://stripe.com/docs/api/credit_notes/object>>.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CreditNote {
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax.
    pub amount: i64,
    /// This is the sum of all the shipping amounts.
    pub amount_shipping: i64,
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// ID of the customer.
    pub customer: stripe_types::Expandable<stripe_shared::Customer>,
    /// Customer balance transaction related to this credit note.
    pub customer_balance_transaction:
        Option<stripe_types::Expandable<stripe_shared::CustomerBalanceTransaction>>,
    /// The integer amount in cents (or local equivalent) representing the total amount of discount that was credited.
    pub discount_amount: i64,
    /// The aggregate amounts calculated per discount for all line items.
    pub discount_amounts: Vec<stripe_shared::DiscountsResourceDiscountAmount>,
    /// The date when this credit note is in effect.
    /// Same as `created` unless overwritten.
    /// When defined, this value replaces the system-generated 'Date of issue' printed on the credit note PDF.
    pub effective_at: Option<stripe_types::Timestamp>,
    /// Unique identifier for the object.
    pub id: stripe_shared::CreditNoteId,
    /// ID of the invoice.
    pub invoice: stripe_types::Expandable<stripe_shared::Invoice>,
    /// Line items that make up the credit note
    pub lines: stripe_types::List<stripe_shared::CreditNoteLineItem>,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// Customer-facing text that appears on the credit note PDF.
    pub memo: Option<String>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// A unique number that identifies this particular credit note and appears on the PDF of the credit note and its associated invoice.
    pub number: String,
    /// Amount that was credited outside of Stripe.
    pub out_of_band_amount: Option<i64>,
    /// The link to download the PDF of the credit note.
    pub pdf: String,
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub reason: Option<stripe_shared::CreditNoteReason>,
    /// Refund related to this credit note.
    pub refund: Option<stripe_types::Expandable<stripe_shared::Refund>>,
    /// The details of the cost of shipping, including the ShippingRate applied to the invoice.
    pub shipping_cost: Option<stripe_shared::InvoicesShippingCost>,
    /// Status of this credit note, one of `issued` or `void`.
    /// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub status: CreditNoteStatus,
    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding exclusive tax and invoice level discounts.
    pub subtotal: i64,
    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding all tax and invoice level discounts.
    pub subtotal_excluding_tax: Option<i64>,
    /// The aggregate amounts calculated per tax rate for all line items.
    pub tax_amounts: Vec<stripe_shared::CreditNoteTaxAmount>,
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax and all discount.
    pub total: i64,
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, excluding tax, but including discounts.
    pub total_excluding_tax: Option<i64>,
    /// Type of this credit note, one of `pre_payment` or `post_payment`.
    /// A `pre_payment` credit note means it was issued when the invoice was open.
    /// A `post_payment` credit note means it was issued when the invoice was paid.
    #[serde(rename = "type")]
    pub type_: CreditNoteType,
    /// The time that the credit note was voided.
    pub voided_at: Option<stripe_types::Timestamp>,
}
/// Status of this credit note, one of `issued` or `void`.
/// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreditNoteStatus {
    Issued,
    Void,
}
impl CreditNoteStatus {
    pub fn as_str(self) -> &'static str {
        use CreditNoteStatus::*;
        match self {
            Issued => "issued",
            Void => "void",
        }
    }
}

impl std::str::FromStr for CreditNoteStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteStatus::*;
        match s {
            "issued" => Ok(Issued),
            "void" => Ok(Void),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreditNoteStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditNoteStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteStatus"))
    }
}
/// Type of this credit note, one of `pre_payment` or `post_payment`.
/// A `pre_payment` credit note means it was issued when the invoice was open.
/// A `post_payment` credit note means it was issued when the invoice was paid.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreditNoteType {
    PostPayment,
    PrePayment,
}
impl CreditNoteType {
    pub fn as_str(self) -> &'static str {
        use CreditNoteType::*;
        match self {
            PostPayment => "post_payment",
            PrePayment => "pre_payment",
        }
    }
}

impl std::str::FromStr for CreditNoteType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteType::*;
        match s {
            "post_payment" => Ok(PostPayment),
            "pre_payment" => Ok(PrePayment),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreditNoteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditNoteType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteType"))
    }
}
impl stripe_types::Object for CreditNote {
    type Id = stripe_shared::CreditNoteId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(CreditNoteId);
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}
impl CreditNoteReason {
    pub fn as_str(self) -> &'static str {
        use CreditNoteReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            OrderChange => "order_change",
            ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl std::str::FromStr for CreditNoteReason {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "order_change" => Ok(OrderChange),
            "product_unsatisfactory" => Ok(ProductUnsatisfactory),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreditNoteReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CreditNoteReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteReason"))
    }
}
