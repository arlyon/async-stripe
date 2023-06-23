use stripe::{Client, Response};

impl stripe_core::invoice_item::InvoiceItem {
    /// Returns a list of your invoice items.
    ///
    /// Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.
    pub fn list(
        client: &Client,
        params: ListInvoiceItem,
    ) -> Response<stripe_types::List<stripe_core::invoice_item::InvoiceItem>> {
        client.get_query("/invoiceitems", params)
    }
    /// Creates an item to be added to a draft invoice (up to 250 items per invoice).
    ///
    /// If no invoice is specified, the item will be on the next invoice created for the customer specified.
    pub fn create(
        client: &Client,
        params: CreateInvoiceItem,
    ) -> Response<stripe_core::invoice_item::InvoiceItem> {
        client.send_form("/invoiceitems", params, http_types::Method::Post)
    }
    /// Retrieves the invoice item with the given ID.
    pub fn retrieve(
        client: &Client,
        invoiceitem: &stripe_core::invoice_item::InvoiceitemId,
        params: RetrieveInvoiceItem,
    ) -> Response<stripe_core::invoice_item::InvoiceItem> {
        client.get_query(&format!("/invoiceitems/{invoiceitem}", invoiceitem = invoiceitem), params)
    }
    /// Updates the amount or description of an invoice item on an upcoming invoice.
    ///
    /// Updating an invoice item is only possible before the invoice it’s attached to is closed.
    pub fn update(
        client: &Client,
        invoiceitem: &stripe_core::invoice_item::InvoiceitemId,
        params: UpdateInvoiceItem,
    ) -> Response<stripe_core::invoice_item::InvoiceItem> {
        client.send_form(
            &format!("/invoiceitems/{invoiceitem}", invoiceitem = invoiceitem),
            params,
            http_types::Method::Post,
        )
    }
    /// Deletes an invoice item, removing it from an invoice.
    ///
    /// Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.
    pub fn delete(
        client: &Client,
        invoiceitem: &stripe_core::invoice_item::InvoiceitemId,
    ) -> Response<stripe_core::invoice_item::DeletedInvoiceItem> {
        client.send(
            &format!("/invoiceitems/{invoiceitem}", invoiceitem = invoiceitem),
            http_types::Method::Delete,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListInvoiceItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// The identifier of the customer whose invoice items to return.
    ///
    /// If none is provided, all invoice items will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Only return invoice items belonging to this invoice.
    ///
    /// If none is provided, all invoice items will be returned.
    /// If specifying an invoice, no customer identifier is needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
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
    pub starting_after: Option<String>,
}
impl<'a> ListInvoiceItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    pub currency: Option<stripe_types::Currency>,
    /// The ID of the customer who will be billed when this invoice item is billed.
    pub customer: &'a str,
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
    pub discounts: Option<&'a [CreateInvoiceItemDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The ID of an existing invoice to add this invoice item to.
    ///
    /// When left blank, the invoice item will be added to the next upcoming scheduled invoice.
    /// This is useful when adding invoice items in response to an invoice.created webhook.
    /// You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The period associated with this invoice item.
    ///
    /// When set to different values, the period will be rendered on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<CreateInvoiceItemPeriod>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateInvoiceItemPriceData<'a>>,
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
    pub subscription: Option<&'a str>,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateInvoiceItemTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
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
    pub fn new(customer: &'a str) -> Self {
        Self {
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
/// The coupons to redeem into discounts for the invoice item or invoice line item.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateInvoiceItemDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> CreateInvoiceItemDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The period associated with this invoice item.
///
/// When set to different values, the period will be rendered on the invoice.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceItemPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    pub end: stripe_types::Timestamp,
    /// The start of the period.
    pub start: stripe_types::Timestamp,
}
impl CreateInvoiceItemPeriod {
    pub fn new(end: stripe_types::Timestamp, start: stripe_types::Timestamp) -> Self {
        Self { end, start }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceItemPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateInvoiceItemPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateInvoiceItemPriceData<'a> {
    pub fn new(currency: stripe_types::Currency, product: &'a str) -> Self {
        Self {
            currency,
            product,
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateInvoiceItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateInvoiceItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateInvoiceItemTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateInvoiceItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateInvoiceItemTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateInvoiceItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveInvoiceItem<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveInvoiceItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
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
    pub discounts: Option<&'a [UpdateInvoiceItemDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a stripe_types::Metadata>,
    /// The period associated with this invoice item.
    ///
    /// When set to different values, the period will be rendered on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<UpdateInvoiceItemPeriod>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateInvoiceItemPriceData<'a>>,
    /// Non-negative integer.
    ///
    /// The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateInvoiceItemTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    /// Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
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
        Self::default()
    }
}
/// The coupons & existing discounts which apply to the invoice item or invoice line item.
///
/// Item discounts are applied before invoice discounts.
/// Pass an empty string to remove previously-defined discounts.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceItemDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> UpdateInvoiceItemDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The period associated with this invoice item.
///
/// When set to different values, the period will be rendered on the invoice.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceItemPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    pub end: stripe_types::Timestamp,
    /// The start of the period.
    pub start: stripe_types::Timestamp,
}
impl UpdateInvoiceItemPeriod {
    pub fn new(end: stripe_types::Timestamp, start: stripe_types::Timestamp) -> Self {
        Self { end, start }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceItemPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateInvoiceItemPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateInvoiceItemPriceData<'a> {
    pub fn new(currency: stripe_types::Currency, product: &'a str) -> Self {
        Self {
            currency,
            product,
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateInvoiceItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateInvoiceItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateInvoiceItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateInvoiceItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateInvoiceItemTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateInvoiceItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateInvoiceItemTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateInvoiceItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
