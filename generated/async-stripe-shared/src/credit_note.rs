/// Issue a credit note to adjust an invoice's amount after the invoice is finalized.
///
/// Related guide: [Credit notes](https://docs.stripe.com/billing/invoices/credit-notes)
///
/// For more details see <<https://stripe.com/docs/api/credit_notes/object>>.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
    /// ID of the account representing the customer.
    pub customer_account: Option<String>,
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
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Customer-facing text that appears on the credit note PDF.
    pub memo: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// A unique number that identifies this particular credit note and appears on the PDF of the credit note and its associated invoice.
    pub number: String,
    /// Amount that was credited outside of Stripe.
    pub out_of_band_amount: Option<i64>,
    /// The link to download the PDF of the credit note.
    pub pdf: String,
    /// The amount of the credit note that was refunded to the customer, credited to the customer's balance, credited outside of Stripe, or any combination thereof.
    pub post_payment_amount: i64,
    /// The amount of the credit note by which the invoice's `amount_remaining` and `amount_due` were reduced.
    pub pre_payment_amount: i64,
    /// The pretax credit amounts (ex: discount, credit grants, etc) for all line items.
    pub pretax_credit_amounts: Vec<stripe_shared::CreditNotesPretaxCreditAmount>,
    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub reason: Option<stripe_shared::CreditNoteReason>,
    /// Refunds related to this credit note.
    pub refunds: Vec<stripe_shared::CreditNoteRefund>,
    /// The details of the cost of shipping, including the ShippingRate applied to the invoice.
    pub shipping_cost: Option<stripe_shared::InvoicesResourceShippingCost>,
    /// Status of this credit note, one of `issued` or `void`.
    /// Learn more about [voiding credit notes](https://docs.stripe.com/billing/invoices/credit-notes#voiding).
    pub status: CreditNoteStatus,
    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding exclusive tax and invoice level discounts.
    pub subtotal: i64,
    /// The integer amount in cents (or local equivalent) representing the amount of the credit note, excluding all tax and invoice level discounts.
    pub subtotal_excluding_tax: Option<i64>,
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, including tax and all discount.
    pub total: i64,
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note, excluding tax, but including discounts.
    pub total_excluding_tax: Option<i64>,
    /// The aggregate tax information for all line items.
    pub total_taxes: Option<Vec<stripe_shared::BillingBillResourceInvoicingTaxesTax>>,
    /// Type of this credit note, one of `pre_payment` or `post_payment`.
    /// A `pre_payment` credit note means it was issued when the invoice was open.
    /// A `post_payment` credit note means it was issued when the invoice was paid.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: CreditNoteType,
    /// The time that the credit note was voided.
    pub voided_at: Option<stripe_types::Timestamp>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreditNote").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct CreditNoteBuilder {
    amount: Option<i64>,
    amount_shipping: Option<i64>,
    created: Option<stripe_types::Timestamp>,
    currency: Option<stripe_types::Currency>,
    customer: Option<stripe_types::Expandable<stripe_shared::Customer>>,
    customer_account: Option<Option<String>>,
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
    post_payment_amount: Option<i64>,
    pre_payment_amount: Option<i64>,
    pretax_credit_amounts: Option<Vec<stripe_shared::CreditNotesPretaxCreditAmount>>,
    reason: Option<Option<stripe_shared::CreditNoteReason>>,
    refunds: Option<Vec<stripe_shared::CreditNoteRefund>>,
    shipping_cost: Option<Option<stripe_shared::InvoicesResourceShippingCost>>,
    status: Option<CreditNoteStatus>,
    subtotal: Option<i64>,
    subtotal_excluding_tax: Option<Option<i64>>,
    total: Option<i64>,
    total_excluding_tax: Option<Option<i64>>,
    total_taxes: Option<Option<Vec<stripe_shared::BillingBillResourceInvoicingTaxesTax>>>,
    type_: Option<CreditNoteType>,
    voided_at: Option<Option<stripe_types::Timestamp>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: CreditNoteBuilder {
                    amount: Deserialize::default(),
                    amount_shipping: Deserialize::default(),
                    created: Deserialize::default(),
                    currency: Deserialize::default(),
                    customer: Deserialize::default(),
                    customer_account: Deserialize::default(),
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
                    post_payment_amount: Deserialize::default(),
                    pre_payment_amount: Deserialize::default(),
                    pretax_credit_amounts: Deserialize::default(),
                    reason: Deserialize::default(),
                    refunds: Deserialize::default(),
                    shipping_cost: Deserialize::default(),
                    status: Deserialize::default(),
                    subtotal: Deserialize::default(),
                    subtotal_excluding_tax: Deserialize::default(),
                    total: Deserialize::default(),
                    total_excluding_tax: Deserialize::default(),
                    total_taxes: Deserialize::default(),
                    type_: Deserialize::default(),
                    voided_at: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "amount" => Deserialize::begin(&mut self.builder.amount),
                "amount_shipping" => Deserialize::begin(&mut self.builder.amount_shipping),
                "created" => Deserialize::begin(&mut self.builder.created),
                "currency" => Deserialize::begin(&mut self.builder.currency),
                "customer" => Deserialize::begin(&mut self.builder.customer),
                "customer_account" => Deserialize::begin(&mut self.builder.customer_account),
                "customer_balance_transaction" => {
                    Deserialize::begin(&mut self.builder.customer_balance_transaction)
                }
                "discount_amount" => Deserialize::begin(&mut self.builder.discount_amount),
                "discount_amounts" => Deserialize::begin(&mut self.builder.discount_amounts),
                "effective_at" => Deserialize::begin(&mut self.builder.effective_at),
                "id" => Deserialize::begin(&mut self.builder.id),
                "invoice" => Deserialize::begin(&mut self.builder.invoice),
                "lines" => Deserialize::begin(&mut self.builder.lines),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "memo" => Deserialize::begin(&mut self.builder.memo),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "number" => Deserialize::begin(&mut self.builder.number),
                "out_of_band_amount" => Deserialize::begin(&mut self.builder.out_of_band_amount),
                "pdf" => Deserialize::begin(&mut self.builder.pdf),
                "post_payment_amount" => Deserialize::begin(&mut self.builder.post_payment_amount),
                "pre_payment_amount" => Deserialize::begin(&mut self.builder.pre_payment_amount),
                "pretax_credit_amounts" => {
                    Deserialize::begin(&mut self.builder.pretax_credit_amounts)
                }
                "reason" => Deserialize::begin(&mut self.builder.reason),
                "refunds" => Deserialize::begin(&mut self.builder.refunds),
                "shipping_cost" => Deserialize::begin(&mut self.builder.shipping_cost),
                "status" => Deserialize::begin(&mut self.builder.status),
                "subtotal" => Deserialize::begin(&mut self.builder.subtotal),
                "subtotal_excluding_tax" => {
                    Deserialize::begin(&mut self.builder.subtotal_excluding_tax)
                }
                "total" => Deserialize::begin(&mut self.builder.total),
                "total_excluding_tax" => Deserialize::begin(&mut self.builder.total_excluding_tax),
                "total_taxes" => Deserialize::begin(&mut self.builder.total_taxes),
                "type" => Deserialize::begin(&mut self.builder.type_),
                "voided_at" => Deserialize::begin(&mut self.builder.voided_at),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(amount),
                Some(amount_shipping),
                Some(created),
                Some(currency),
                Some(customer),
                Some(customer_account),
                Some(customer_balance_transaction),
                Some(discount_amount),
                Some(discount_amounts),
                Some(effective_at),
                Some(id),
                Some(invoice),
                Some(lines),
                Some(livemode),
                Some(memo),
                Some(metadata),
                Some(number),
                Some(out_of_band_amount),
                Some(pdf),
                Some(post_payment_amount),
                Some(pre_payment_amount),
                Some(pretax_credit_amounts),
                Some(reason),
                Some(refunds),
                Some(shipping_cost),
                Some(status),
                Some(subtotal),
                Some(subtotal_excluding_tax),
                Some(total),
                Some(total_excluding_tax),
                Some(total_taxes),
                Some(type_),
                Some(voided_at),
            ) = (
                self.builder.amount,
                self.builder.amount_shipping,
                self.builder.created,
                self.builder.currency.take(),
                self.builder.customer.take(),
                self.builder.customer_account.take(),
                self.builder.customer_balance_transaction.take(),
                self.builder.discount_amount,
                self.builder.discount_amounts.take(),
                self.builder.effective_at,
                self.builder.id.take(),
                self.builder.invoice.take(),
                self.builder.lines.take(),
                self.builder.livemode,
                self.builder.memo.take(),
                self.builder.metadata.take(),
                self.builder.number.take(),
                self.builder.out_of_band_amount,
                self.builder.pdf.take(),
                self.builder.post_payment_amount,
                self.builder.pre_payment_amount,
                self.builder.pretax_credit_amounts.take(),
                self.builder.reason.take(),
                self.builder.refunds.take(),
                self.builder.shipping_cost.take(),
                self.builder.status.take(),
                self.builder.subtotal,
                self.builder.subtotal_excluding_tax,
                self.builder.total,
                self.builder.total_excluding_tax,
                self.builder.total_taxes.take(),
                self.builder.type_.take(),
                self.builder.voided_at,
            )
            else {
                return Ok(());
            };
            *self.out = Some(CreditNote {
                amount,
                amount_shipping,
                created,
                currency,
                customer,
                customer_account,
                customer_balance_transaction,
                discount_amount,
                discount_amounts,
                effective_at,
                id,
                invoice,
                lines,
                livemode,
                memo,
                metadata,
                number,
                out_of_band_amount,
                pdf,
                post_payment_amount,
                pre_payment_amount,
                pretax_credit_amounts,
                reason,
                refunds,
                shipping_cost,
                status,
                subtotal,
                subtotal_excluding_tax,
                total,
                total_excluding_tax,
                total_taxes,
                type_,
                voided_at,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for CreditNote {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("CreditNote", 34)?;
        s.serialize_field("amount", &self.amount)?;
        s.serialize_field("amount_shipping", &self.amount_shipping)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("currency", &self.currency)?;
        s.serialize_field("customer", &self.customer)?;
        s.serialize_field("customer_account", &self.customer_account)?;
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
        s.serialize_field("post_payment_amount", &self.post_payment_amount)?;
        s.serialize_field("pre_payment_amount", &self.pre_payment_amount)?;
        s.serialize_field("pretax_credit_amounts", &self.pretax_credit_amounts)?;
        s.serialize_field("reason", &self.reason)?;
        s.serialize_field("refunds", &self.refunds)?;
        s.serialize_field("shipping_cost", &self.shipping_cost)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("subtotal", &self.subtotal)?;
        s.serialize_field("subtotal_excluding_tax", &self.subtotal_excluding_tax)?;
        s.serialize_field("total", &self.total)?;
        s.serialize_field("total_excluding_tax", &self.total_excluding_tax)?;
        s.serialize_field("total_taxes", &self.total_taxes)?;
        s.serialize_field("type", &self.type_)?;
        s.serialize_field("voided_at", &self.voided_at)?;

        s.serialize_field("object", "credit_note")?;
        s.end()
    }
}
/// Status of this credit note, one of `issued` or `void`.
/// Learn more about [voiding credit notes](https://docs.stripe.com/billing/invoices/credit-notes#voiding).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreditNoteStatus {
    Issued,
    Void,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreditNoteStatus {
    pub fn as_str(&self) -> &str {
        use CreditNoteStatus::*;
        match self {
            Issued => "issued",
            Void => "void",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreditNoteStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteStatus::*;
        match s {
            "issued" => Ok(Issued),
            "void" => Ok(Void),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreditNoteStatus");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreditNoteStatus)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for CreditNoteStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CreditNoteStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNoteStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Type of this credit note, one of `pre_payment` or `post_payment`.
/// A `pre_payment` credit note means it was issued when the invoice was open.
/// A `post_payment` credit note means it was issued when the invoice was paid.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreditNoteType {
    Mixed,
    PostPayment,
    PrePayment,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreditNoteType {
    pub fn as_str(&self) -> &str {
        use CreditNoteType::*;
        match self {
            Mixed => "mixed",
            PostPayment => "post_payment",
            PrePayment => "pre_payment",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreditNoteType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteType::*;
        match s {
            "mixed" => Ok(Mixed),
            "post_payment" => Ok(PostPayment),
            "pre_payment" => Ok(PrePayment),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreditNoteType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreditNoteType)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for CreditNoteType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CreditNoteType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNoteType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreditNoteReason {
    pub fn as_str(&self) -> &str {
        use CreditNoteReason::*;
        match self {
            Duplicate => "duplicate",
            Fraudulent => "fraudulent",
            OrderChange => "order_change",
            ProductUnsatisfactory => "product_unsatisfactory",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreditNoteReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreditNoteReason::*;
        match s {
            "duplicate" => Ok(Duplicate),
            "fraudulent" => Ok(Fraudulent),
            "order_change" => Ok(OrderChange),
            "product_unsatisfactory" => Ok(ProductUnsatisfactory),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreditNoteReason");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreditNoteReason)).finish_non_exhaustive()
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
impl stripe_miniserde::Deserialize for CreditNoteReason {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<CreditNoteReason> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(CreditNoteReason::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreditNoteReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
