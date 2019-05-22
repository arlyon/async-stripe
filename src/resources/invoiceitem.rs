// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CustomerId, InvoiceId, InvoiceItemId};
use crate::params::{Deleted, Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{Currency, Customer, Invoice, Period, Plan, Subscription, TaxRate};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoiceItem".
///
/// For more details see [https://stripe.com/docs/api/invoiceitems/object](https://stripe.com/docs/api/invoiceitems/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct InvoiceItem {
    /// Unique identifier for the object.
    pub id: InvoiceItemId,

    /// Amount (in the `currency` specified) of the invoice item.
    ///
    /// This should always be equal to `unit_amount * quantity`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The ID of the customer who will be billed when this invoice item is billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Expandable<Customer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    deleted: bool,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// If true, discounts will apply to this invoice item.
    ///
    /// Always false for prorations.
    #[serde(default)]
    pub discountable: bool,

    /// The ID of the invoice this invoice item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Expandable<Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(default)]
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    /// If the invoice item is a proration, the plan of the subscription that the proration was computed for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,

    /// Whether the invoice item was created automatically as a proration adjustment when the customer switched plans.
    #[serde(default)]
    pub proration: bool,

    /// Quantity of units for the invoice item.
    ///
    /// If the invoice item is a proration, the quantity of the subscription that the proration was computed for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The subscription that this invoice item has been created for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,

    /// Unit Amount (in the `currency` specified) of the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
}

impl InvoiceItem {
    /// Returns a list of your invoice items.
    ///
    /// Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.
    pub fn list(client: &Client, params: InvoiceItemListParams<'_>) -> Response<List<InvoiceItem>> {
        client.get_query("/invoiceitems", &params)
    }

    /// Retrieves the invoice item with the given ID.
    pub fn retrieve(client: &Client, id: &InvoiceItemId, expand: &[&str]) -> Response<InvoiceItem> {
        client.get_query(&format!("/invoiceitems/{}", id), &Expand { expand })
    }

    /// Retrieves the invoice item with the given ID.
    pub fn delete(client: &Client, id: &InvoiceItemId) -> Response<Deleted<InvoiceItemId>> {
        client.delete(&format!("/invoiceitems/{}", id))
    }
}

impl Object for InvoiceItem {
    type Id = InvoiceItemId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "invoiceitem"
    }
}

/// The parameters for `InvoiceItem::list`.
#[derive(Clone, Debug, Serialize)]
pub struct InvoiceItemListParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<RangeQuery<Timestamp>>,

    /// The identifier of the customer whose invoice items to return.
    ///
    /// If none is provided, all invoice items will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a InvoiceItemId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    expand: &'a [&'a str],

    /// Only return invoice items belonging to this invoice.
    ///
    /// If none is provided, all invoice items will be returned.
    /// If specifying an invoice, no customer identifier is needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<InvoiceId>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u64>,

    /// Set to `true` to only show pending invoice items, which are not yet attached to any invoices.
    ///
    /// Set to `false` to only show invoice items already attached to invoices.
    /// If unspecified, no filter is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pending: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a InvoiceItemId>,
}

impl<'a> InvoiceItemListParams<'a> {
    pub fn new() -> Self {
        InvoiceItemListParams {
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            invoice: Default::default(),
            limit: Default::default(),
            pending: Default::default(),
            starting_after: Default::default(),
        }
    }
}
