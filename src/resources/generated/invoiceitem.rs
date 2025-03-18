// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{CustomerId, InvoiceId, InvoiceItemId, PriceId, SubscriptionId};
use crate::params::{
    Deleted, Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp,
};
use crate::resources::{
    Currency, Customer, Discount, Invoice, Period, Plan, Price, Subscription, TaxRate,
    TestHelpersTestClock,
};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "InvoiceItem".
///
/// For more details see <https://stripe.com/docs/api/invoiceitems/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// If true, discounts will apply to this invoice item.
    ///
    /// Always false for prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// The discounts which apply to the invoice item.
    ///
    /// Item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<Expandable<Discount>>>,

    /// The ID of the invoice this invoice item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Expandable<Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub livemode: Option<bool>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    /// If the invoice item is a proration, the plan of the subscription that the proration was computed for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Plan>,

    /// The price of the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,

    /// Whether the invoice item was created automatically as a proration adjustment when the customer switched plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration: Option<bool>,

    /// Quantity of units for the invoice item.
    ///
    /// If the invoice item is a proration, the quantity of the subscription that the proration was computed for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The subscription that this invoice item has been created for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,

    /// The subscription item that this invoice item has been created for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,

    /// ID of the test clock this invoice item belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<Expandable<TestHelpersTestClock>>,

    /// Unit amount (in the `currency` specified) of the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

impl InvoiceItem {
    /// Returns a list of your invoice items.
    ///
    /// Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.
    pub fn list(client: &Client, params: &ListInvoiceItems<'_>) -> Response<List<InvoiceItem>> {
        client.get_query("/invoiceitems", params)
    }

    /// Creates an item to be added to a draft invoice (up to 250 items per invoice).
    ///
    /// If no invoice is specified, the item will be on the next invoice created for the customer specified.
    pub fn create(client: &Client, params: CreateInvoiceItem<'_>) -> Response<InvoiceItem> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/invoiceitems", &params)
    }

    /// Retrieves the invoice item with the given ID.
    pub fn retrieve(client: &Client, id: &InvoiceItemId, expand: &[&str]) -> Response<InvoiceItem> {
        client.get_query(&format!("/invoiceitems/{}", id), Expand { expand })
    }

    /// Updates the amount or description of an invoice item on an upcoming invoice.
    ///
    /// Updating an invoice item is only possible before the invoice it’s attached to is closed.
    pub fn update(
        client: &Client,
        id: &InvoiceItemId,
        params: UpdateInvoiceItem<'_>,
    ) -> Response<InvoiceItem> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/invoiceitems/{}", id), &params)
    }

    /// Deletes an invoice item, removing it from an invoice.
    ///
    /// Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.
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

/// The parameters for `InvoiceItem::create`.
#[derive(Clone, Debug, Serialize)]
pub struct CreateInvoiceItem<'a> {
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// Passing in a negative `amount` will reduce the `amount_due` on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// The ID of the customer who will be billed when this invoice item is billed.
    pub customer: CustomerId,

    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Controls whether discounts apply to this invoice item.
    ///
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// The coupons to redeem into discounts for the invoice item or invoice line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<CreateInvoiceItemDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// The ID of an existing invoice to add this invoice item to.
    ///
    /// When left blank, the invoice item will be added to the next upcoming scheduled invoice.
    /// This is useful when adding invoice items in response to an invoice.created webhook.
    /// You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<InvoiceId>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The period associated with this invoice item.
    ///
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceId>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<InvoiceItemPriceData>,

    /// Non-negative integer.
    ///
    /// The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The ID of a subscription to add this invoice item to.
    ///
    /// When left blank, the invoice item will be be added to the next upcoming scheduled invoice.
    /// When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item.
    /// Use this when you want to express that an invoice item has been accrued within the context of a particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<SubscriptionId>,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<InvoiceItemTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// This `unit_amount` will be multiplied by the quantity to get the full amount.
    /// Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}

impl<'a> CreateInvoiceItem<'a> {
    pub fn new(customer: CustomerId) -> Self {
        CreateInvoiceItem {
            amount: Default::default(),
            currency: Default::default(),
            customer,
            description: Default::default(),
            discountable: Default::default(),
            discounts: Default::default(),
            expand: Default::default(),
            invoice: Default::default(),
            metadata: Default::default(),
            period: Default::default(),
            price: Default::default(),
            price_data: Default::default(),
            quantity: Default::default(),
            subscription: Default::default(),
            tax_behavior: Default::default(),
            tax_code: Default::default(),
            tax_rates: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}

/// The parameters for `InvoiceItem::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListInvoiceItems<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// The identifier of the customer whose invoice items to return.
    ///
    /// If none is provided, all invoice items will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<InvoiceItemId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Only return invoice items belonging to this invoice.
    ///
    /// If none is provided, all invoice items will be returned.
    /// If specifying an invoice, no customer identifier is needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<InvoiceId>,

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Set to `true` to only show pending invoice items, which are not yet attached to any invoices.
    ///
    /// Set to `false` to only show invoice items already attached to invoices.
    /// If unspecified, no filter is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<InvoiceItemId>,
}

impl<'a> ListInvoiceItems<'a> {
    pub fn new() -> Self {
        ListInvoiceItems {
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
impl Paginable for ListInvoiceItems<'_> {
    type O = InvoiceItem;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `InvoiceItem::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateInvoiceItem<'a> {
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// If you want to apply a credit to the customer's account, pass a negative amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,

    /// Controls whether discounts apply to this invoice item.
    ///
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    /// Cannot be set to true for prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// The coupons & existing discounts which apply to the invoice item or invoice line item.
    ///
    /// Item discounts are applied before invoice discounts.
    /// Pass an empty string to remove previously-defined discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<UpdateInvoiceItemDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The period associated with this invoice item.
    ///
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<PriceId>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<InvoiceItemPriceData>,

    /// Non-negative integer.
    ///
    /// The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<InvoiceItemTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    /// Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}

impl<'a> UpdateInvoiceItem<'a> {
    pub fn new() -> Self {
        UpdateInvoiceItem {
            amount: Default::default(),
            description: Default::default(),
            discountable: Default::default(),
            discounts: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            period: Default::default(),
            price: Default::default(),
            price_data: Default::default(),
            quantity: Default::default(),
            tax_behavior: Default::default(),
            tax_code: Default::default(),
            tax_rates: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateInvoiceItemDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InvoiceItemPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<InvoiceItemPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateInvoiceItemDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

/// An enum representing the possible values of an `InvoiceItemPriceData`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl InvoiceItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceItemPriceDataTaxBehavior::Exclusive => "exclusive",
            InvoiceItemPriceDataTaxBehavior::Inclusive => "inclusive",
            InvoiceItemPriceDataTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for InvoiceItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoiceItemPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `CreateInvoiceItem`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceItemTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl InvoiceItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            InvoiceItemTaxBehavior::Exclusive => "exclusive",
            InvoiceItemTaxBehavior::Inclusive => "inclusive",
            InvoiceItemTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for InvoiceItemTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for InvoiceItemTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
