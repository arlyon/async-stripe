// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{InvoiceId, InvoicePaymentId};
use crate::params::{Expand, Expandable, List, Object, Paginable, Timestamp};
use crate::resources::{Charge, Currency, Invoice, PaymentIntent};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoicesInvoicePayment".
///
/// For more details see <https://stripe.com/docs/api/invoice-payment/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicePayment {
    /// Unique identifier for the object.
    pub id: InvoicePaymentId,

    /// Amount that was actually paid for this invoice, in cents (or local equivalent).
    ///
    /// This field is null until the payment is `paid`.
    /// This amount can be less than the `amount_requested` if the PaymentIntent’s `amount_received` is not sufficient to pay all of the invoices that it is attached to.
    pub amount_paid: Option<i64>,

    /// Amount intended to be paid toward this invoice, in cents (or local equivalent).
    pub amount_requested: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The invoice that was paid.
    pub invoice: Expandable<Invoice>,

    /// Stripe automatically creates a default InvoicePayment when the invoice is finalized, and keeps it synchronized with the invoice’s `amount_remaining`.
    ///
    /// The PaymentIntent associated with the default payment can’t be edited or canceled directly.
    pub is_default: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub payment: InvoicesPaymentsInvoicePaymentAssociatedPayment,

    /// The status of the payment, one of `open`, `paid`, or `canceled`.
    pub status: String,

    pub status_transitions: InvoicesPaymentsInvoicePaymentStatusTransitions,
}

impl InvoicePayment {

    /// When retrieving an invoice, there is an includable payments property containing the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of payments.
pub fn list(client: &Client, params: &ListInvoicePayments<'_>) -> Response<List<InvoicePayment>> {
   client.get_query("/invoice_payments", params)
}


    /// Retrieves the invoice payment with the given ID.
    pub fn retrieve(client: &Client, id: &InvoicePaymentId, expand: &[&str]) -> Response<InvoicePayment> {
        client.get_query(&format!("/invoice_payments/{}", id), Expand { expand })
    }
}

impl Object for InvoicePayment {
    type Id = InvoicePaymentId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "invoice_payment"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesPaymentsInvoicePaymentAssociatedPayment {

    /// ID of the successful charge for this payment when `type` is `charge`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Expandable<Charge>>,

    /// ID of the PaymentIntent associated with this payment when `type` is `payment_intent`.
    ///
    /// Note: This property is only populated for invoices finalized on or after March 15th, 2019.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Expandable<PaymentIntent>>,

    /// Type of payment object associated with this invoice payment.
    #[serde(rename = "type")]
    pub type_: InvoicesPaymentsInvoicePaymentAssociatedPaymentType,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoicesPaymentsInvoicePaymentStatusTransitions {

    /// The time that the payment was canceled.
    pub canceled_at: Option<Timestamp>,

    /// The time that the payment succeeded.
    pub paid_at: Option<Timestamp>,
}

/// The parameters for `InvoicePayment::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListInvoicePayments<'a> {

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<InvoicePaymentId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The identifier of the invoice whose payments to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<InvoiceId>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// The payment details of the invoice payments to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<ListInvoicePaymentsPayment>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<InvoicePaymentId>,

    /// The status of the invoice payments to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<InvoicePaymentStatus>,
}

impl<'a> ListInvoicePayments<'a> {
    pub fn new() -> Self {
        ListInvoicePayments {
            ending_before: Default::default(),
            expand: Default::default(),
            invoice: Default::default(),
            limit: Default::default(),
            payment: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}
impl Paginable for ListInvoicePayments<'_> {
    type O = InvoicePayment;
    fn set_last(&mut self, item: Self::O) {
                self.starting_after = Some(item.id());
            }}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListInvoicePaymentsPayment {

    /// Only return invoice payments associated by this payment intent ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,

    /// Only return invoice payments associated by this payment type.
    #[serde(rename = "type")]
    pub type_: ListInvoicePaymentsPaymentType,
}

/// An enum representing the possible values of an `ListInvoicePayments`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentStatus {
    Canceled,
    Open,
    Paid,
}

impl InvoicePaymentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicePaymentStatus::Canceled => "canceled",
            InvoicePaymentStatus::Open => "open",
            InvoicePaymentStatus::Paid => "paid",
        }
    }
}

impl AsRef<str> for InvoicePaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicePaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoicePaymentStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

/// An enum representing the possible values of an `InvoicesPaymentsInvoicePaymentAssociatedPayment`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    Charge,
    PaymentIntent,
}

impl InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoicesPaymentsInvoicePaymentAssociatedPaymentType::Charge => "charge",
            InvoicesPaymentsInvoicePaymentAssociatedPaymentType::PaymentIntent => "payment_intent",
        }
    }
}

impl AsRef<str> for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoicesPaymentsInvoicePaymentAssociatedPaymentType {
    fn default() -> Self {
        Self::Charge
    }
}

/// An enum representing the possible values of an `ListInvoicePaymentsPayment`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ListInvoicePaymentsPaymentType {
    PaymentIntent,
}

impl ListInvoicePaymentsPaymentType {
    pub fn as_str(self) -> &'static str {
        match self {
            ListInvoicePaymentsPaymentType::PaymentIntent => "payment_intent",
        }
    }
}

impl AsRef<str> for ListInvoicePaymentsPaymentType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListInvoicePaymentsPaymentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ListInvoicePaymentsPaymentType {
    fn default() -> Self {
        Self::PaymentIntent
    }
}
