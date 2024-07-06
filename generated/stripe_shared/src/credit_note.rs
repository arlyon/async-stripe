/// Issue a credit note to adjust an invoice's amount after the invoice is finalized.
///
/// Related guide: [Credit notes](https://stripe.com/docs/billing/invoices/credit-notes)
///
/// For more details see <<https://stripe.com/docs/api/credit_notes/object>>.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
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
    pub shipping_cost: Option<stripe_shared::InvoicesResourceShippingCost>,
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
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: CreditNoteType,
    /// The time that the credit note was voided.
    pub voided_at: Option<stripe_types::Timestamp>,
}
#[doc(hidden)]
pub struct CreditNoteBuilder {
    amount: Option<i64>,
    amount_shipping: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    customer_balance_transaction:
        Option<Option<stripe_types::Expandable<stripe_shared::CustomerBalanceTransaction>>>,
    discount_amount: Option<i64>,
    discount_amounts: Option<Vec<stripe_shared::DiscountsResourceDiscountAmount>>,
    effective_at: Option<Option<stripe_types::Timestamp>>,
    id: Option<stripe_shared::CreditNoteId>,
    invoice: Option<stripe_types::Expandable<stripe_shared::Invoice>>,
    lines: Option<stripe_types::List<stripe_shared::CreditNoteLineItem>>,
    livemode: Option<bool>,
    memo: Option<Option<String>>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    number: Option<String>,
    out_of_band_amount: Option<Option<i64>>,
    pdf: Option<String>,
    reason: Option<Option<stripe_shared::CreditNoteReason>>,
    refund: Option<Option<stripe_types::Expandable<stripe_shared::Refund>>>,
    shipping_cost: Option<Option<stripe_shared::InvoicesResourceShippingCost>>,
    status: Option<CreditNoteStatus>,
    subtotal: Option<i64>,
    subtotal_excluding_tax: Option<Option<i64>>,
    tax_amounts: Option<Vec<stripe_shared::CreditNoteTaxAmount>>,
    total: Option<i64>,
    total_excluding_tax: Option<Option<i64>>,
    type_: Option<CreditNoteType>,
    voided_at: Option<Option<stripe_types::Timestamp>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for CreditNote {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CreditNote>,
        builder: CreditNoteBuilder,
    }

    impl Visitor for Place<CreditNote> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CreditNoteBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CreditNoteBuilder {
        type Out = CreditNote;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.amount),
                "amount_shipping" => Deserialize::begin(&mut self.amount_shipping),
                "created" => Deserialize::begin(&mut self.created),
                "currency" => Deserialize::begin(&mut self.currency),
                "customer" => Deserialize::begin(&mut self.customer),
                "customer_balance_transaction" => {
                    Deserialize::begin(&mut self.customer_balance_transaction)
                }
                "discount_amount" => Deserialize::begin(&mut self.discount_amount),
                "discount_amounts" => Deserialize::begin(&mut self.discount_amounts),
                "effective_at" => Deserialize::begin(&mut self.effective_at),
                "id" => Deserialize::begin(&mut self.id),
                "invoice" => Deserialize::begin(&mut self.invoice),
                "lines" => Deserialize::begin(&mut self.lines),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "memo" => Deserialize::begin(&mut self.memo),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "number" => Deserialize::begin(&mut self.number),
                "out_of_band_amount" => Deserialize::begin(&mut self.out_of_band_amount),
                "pdf" => Deserialize::begin(&mut self.pdf),
                "reason" => Deserialize::begin(&mut self.reason),
                "refund" => Deserialize::begin(&mut self.refund),
                "shipping_cost" => Deserialize::begin(&mut self.shipping_cost),
                "status" => Deserialize::begin(&mut self.status),
                "subtotal" => Deserialize::begin(&mut self.subtotal),
                "subtotal_excluding_tax" => Deserialize::begin(&mut self.subtotal_excluding_tax),
                "tax_amounts" => Deserialize::begin(&mut self.tax_amounts),
                "total" => Deserialize::begin(&mut self.total),
                "total_excluding_tax" => Deserialize::begin(&mut self.total_excluding_tax),
                "type" => Deserialize::begin(&mut self.type_),
                "voided_at" => Deserialize::begin(&mut self.voided_at),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                amount: Deserialize::default(),
                amount_shipping: Deserialize::default(),
                created: Deserialize::default(),
                currency: Deserialize::default(),
                customer: Deserialize::default(),
                customer_balance_transaction: Deserialize::default(),
                discount_amount: Deserialize::default(),
                discount_amounts: Deserialize::default(),
                effective_at: Deserialize::default(),
                id: Deserialize::default(),
                invoice: Deserialize::default(),
                lines: Deserialize::default(),
                livemode: Deserialize::default(),
                memo: Deserialize::default(),
                metadata: Deserialize::default(),
                number: Deserialize::default(),
                out_of_band_amount: Deserialize::default(),
                pdf: Deserialize::default(),
                reason: Deserialize::default(),
                refund: Deserialize::default(),
                shipping_cost: Deserialize::default(),
                status: Deserialize::default(),
                subtotal: Deserialize::default(),
                subtotal_excluding_tax: Deserialize::default(),
                tax_amounts: Deserialize::default(),
                total: Deserialize::default(),
                total_excluding_tax: Deserialize::default(),
                type_: Deserialize::default(),
                voided_at: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                amount: self.amount?,
                amount_shipping: self.amount_shipping?,
                created: self.created?,
                currency: self.currency?,
                customer: self.customer.take()?,
                customer_balance_transaction: self.customer_balance_transaction.take()?,
                discount_amount: self.discount_amount?,
                discount_amounts: self.discount_amounts.take()?,
                effective_at: self.effective_at?,
                id: self.id.take()?,
                invoice: self.invoice.take()?,
                lines: self.lines.take()?,
                livemode: self.livemode?,
                memo: self.memo.take()?,
                metadata: self.metadata.take()?,
                number: self.number.take()?,
                out_of_band_amount: self.out_of_band_amount?,
                pdf: self.pdf.take()?,
                reason: self.reason?,
                refund: self.refund.take()?,
                shipping_cost: self.shipping_cost.take()?,
                status: self.status?,
                subtotal: self.subtotal?,
                subtotal_excluding_tax: self.subtotal_excluding_tax?,
                tax_amounts: self.tax_amounts.take()?,
                total: self.total?,
                total_excluding_tax: self.total_excluding_tax?,
                type_: self.type_?,
                voided_at: self.voided_at?,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for CreditNote {
        type Builder = CreditNoteBuilder;
    }

    impl FromValueOpt for CreditNote {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CreditNoteBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "amount" => b.amount = Some(FromValueOpt::from_value(v)?),
                    "amount_shipping" => b.amount_shipping = Some(FromValueOpt::from_value(v)?),
                    "created" => b.created = Some(FromValueOpt::from_value(v)?),
                    "currency" => b.currency = Some(FromValueOpt::from_value(v)?),
                    "customer" => b.customer = Some(FromValueOpt::from_value(v)?),
                    "customer_balance_transaction" => {
                        b.customer_balance_transaction = Some(FromValueOpt::from_value(v)?)
                    }
                    "discount_amount" => b.discount_amount = Some(FromValueOpt::from_value(v)?),
                    "discount_amounts" => b.discount_amounts = Some(FromValueOpt::from_value(v)?),
                    "effective_at" => b.effective_at = Some(FromValueOpt::from_value(v)?),
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "invoice" => b.invoice = Some(FromValueOpt::from_value(v)?),
                    "lines" => b.lines = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "memo" => b.memo = Some(FromValueOpt::from_value(v)?),
                    "metadata" => b.metadata = Some(FromValueOpt::from_value(v)?),
                    "number" => b.number = Some(FromValueOpt::from_value(v)?),
                    "out_of_band_amount" => {
                        b.out_of_band_amount = Some(FromValueOpt::from_value(v)?)
                    }
                    "pdf" => b.pdf = Some(FromValueOpt::from_value(v)?),
                    "reason" => b.reason = Some(FromValueOpt::from_value(v)?),
                    "refund" => b.refund = Some(FromValueOpt::from_value(v)?),
                    "shipping_cost" => b.shipping_cost = Some(FromValueOpt::from_value(v)?),
                    "status" => b.status = Some(FromValueOpt::from_value(v)?),
                    "subtotal" => b.subtotal = Some(FromValueOpt::from_value(v)?),
                    "subtotal_excluding_tax" => {
                        b.subtotal_excluding_tax = Some(FromValueOpt::from_value(v)?)
                    }
                    "tax_amounts" => b.tax_amounts = Some(FromValueOpt::from_value(v)?),
                    "total" => b.total = Some(FromValueOpt::from_value(v)?),
                    "total_excluding_tax" => {
                        b.total_excluding_tax = Some(FromValueOpt::from_value(v)?)
                    }
                    "type" => b.type_ = Some(FromValueOpt::from_value(v)?),
                    "voided_at" => b.voided_at = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNote {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CreditNote", 30)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_shipping", &self.amount_shipping)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_balance_transaction", &self.customer_balance_transaction)?;
        s.serialize_field("discount_amount", &self.discount_amount)?;
        s.serialize_field("discount_amounts", &self.discount_amounts)?;
        s.serialize_field("effective_at", &self.effective_at)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("invoice", &self.invoice)?;
        s.serialize_field("lines", &self.lines)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("memo", &self.memo)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("number", &self.number)?;
        s.serialize_field("out_of_band_amount", &self.out_of_band_amount)?;
        s.serialize_field("pdf", &self.pdf)?;
        s.serialize_field("reason", &self.reason)?;
        s.serialize_field("refund", &self.refund)?;
        s.serialize_field("shipping_cost", &self.shipping_cost)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("subtotal", &self.subtotal)?;
        s.serialize_field("subtotal_excluding_tax", &self.subtotal_excluding_tax)?;
        s.serialize_field("tax_amounts", &self.tax_amounts)?;
        s.serialize_field("total", &self.total)?;
        s.serialize_field("total_excluding_tax", &self.total_excluding_tax)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("voided_at", &self.voided_at)?;

        s.serialize_field("object", "credit_note")?;
        s.end()
    }
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteStatus::*;
        match s {
            "issued" => Ok(Issued),
            "void" => Ok(Void),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNoteStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreditNoteStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreditNoteStatus> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteStatus::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreditNoteStatus);
#[cfg(feature = "deserialize")]
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteType::*;
        match s {
            "post_payment" => Ok(PostPayment),
            "pre_payment" => Ok(PrePayment),
            _ => Err(stripe_types::StripeParseError),
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
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNoteType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for CreditNoteType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreditNoteType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteType::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreditNoteType);
#[cfg(feature = "deserialize")]
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

    fn into_id(self) -> Self::Id {
        self.id
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
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "order_change" => Ok(OrderChange),
            "product_unsatisfactory" => Ok(ProductUnsatisfactory),
            _ => Err(stripe_types::StripeParseError),
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
impl miniserde::Deserialize for CreditNoteReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<CreditNoteReason> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteReason::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(CreditNoteReason);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNoteReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreditNoteReason"))
    }
}
