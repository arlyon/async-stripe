use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes an invoice item, removing it from an invoice.
/// Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteInvoiceItem<'a> {
    invoiceitem: &'a stripe_shared::InvoiceItemId,
}
impl<'a> DeleteInvoiceItem<'a> {
    /// Construct a new `DeleteInvoiceItem`.
    pub fn new(invoiceitem: &'a stripe_shared::InvoiceItemId) -> Self {
        Self { invoiceitem }
    }
}
impl DeleteInvoiceItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for DeleteInvoiceItem<'_> {
    type Output = stripe_shared::DeletedInvoiceitem;

    fn build(&self) -> RequestBuilder {
        let invoiceitem = self.invoiceitem;
        RequestBuilder::new(StripeMethod::Delete, format!("/invoiceitems/{invoiceitem}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListInvoiceItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListInvoiceItemBuilder<'a> {
    fn new() -> Self {
        Self {
            created: None,
            customer: None,
            ending_before: None,
            expand: None,
            invoice: None,
            limit: None,
            pending: None,
            starting_after: None,
        }
    }
}
/// Returns a list of your invoice items.
/// Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListInvoiceItem<'a> {
    inner: ListInvoiceItemBuilder<'a>,
}
impl<'a> ListInvoiceItem<'a> {
    /// Construct a new `ListInvoiceItem`.
    pub fn new() -> Self {
        Self { inner: ListInvoiceItemBuilder::new() }
    }
    /// Only return invoice items that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
    }
    /// The identifier of the customer whose invoice items to return.
    /// If none is provided, all invoice items will be returned.
    pub fn customer(mut self, customer: &'a str) -> Self {
        self.inner.customer = Some(customer);
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Only return invoice items belonging to this invoice.
    /// If none is provided, all invoice items will be returned.
    /// If specifying an invoice, no customer identifier is needed.
    pub fn invoice(mut self, invoice: &'a str) -> Self {
        self.inner.invoice = Some(invoice);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// Set to `true` to only show pending invoice items, which are not yet attached to any invoices.
    /// Set to `false` to only show invoice items already attached to invoices.
    /// If unspecified, no filter is applied.
    pub fn pending(mut self, pending: bool) -> Self {
        self.inner.pending = Some(pending);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListInvoiceItem<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListInvoiceItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }

    pub fn paginate(
        &self,
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::InvoiceItem>> {
        stripe_client_core::ListPaginator::new_list("/invoiceitems", self.inner)
    }
}

impl StripeRequest for ListInvoiceItem<'_> {
    type Output = stripe_types::List<stripe_shared::InvoiceItem>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/invoiceitems").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveInvoiceItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveInvoiceItemBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the invoice item with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveInvoiceItem<'a> {
    inner: RetrieveInvoiceItemBuilder<'a>,
    invoiceitem: &'a stripe_shared::InvoiceItemId,
}
impl<'a> RetrieveInvoiceItem<'a> {
    /// Construct a new `RetrieveInvoiceItem`.
    pub fn new(invoiceitem: &'a stripe_shared::InvoiceItemId) -> Self {
        Self { invoiceitem, inner: RetrieveInvoiceItemBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveInvoiceItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for RetrieveInvoiceItem<'_> {
    type Output = stripe_shared::InvoiceItem;

    fn build(&self) -> RequestBuilder {
        let invoiceitem = self.invoiceitem;
        RequestBuilder::new(StripeMethod::Get, format!("/invoiceitems/{invoiceitem}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateInvoiceItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    customer: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<&'a [DiscountsDataParam<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data: Option<CreateInvoiceItemPriceData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<CreateInvoiceItemTaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateInvoiceItemBuilder<'a> {
    fn new(customer: &'a str) -> Self {
        Self {
            amount: None,
            currency: None,
            customer,
            description: None,
            discountable: None,
            discounts: None,
            expand: None,
            invoice: None,
            metadata: None,
            period: None,
            price: None,
            price_data: None,
            quantity: None,
            subscription: None,
            tax_behavior: None,
            tax_code: None,
            tax_rates: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceItemPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateInvoiceItemPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateInvoiceItemPriceData<'a> {
    pub fn new(currency: stripe_types::Currency, product: &'a str) -> Self {
        Self { currency, product, tax_behavior: None, unit_amount: None, unit_amount_decimal: None }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateInvoiceItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceItemPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateInvoiceItemPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceItemPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceItemPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoiceItemPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateInvoiceItemPriceDataTaxBehavior")
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateInvoiceItemTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateInvoiceItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateInvoiceItemTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateInvoiceItemTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateInvoiceItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateInvoiceItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateInvoiceItemTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateInvoiceItemTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateInvoiceItemTaxBehavior"))
    }
}
/// Creates an item to be added to a draft invoice (up to 250 items per invoice).
/// If no invoice is specified, the item will be on the next invoice created for the customer specified.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceItem<'a> {
    inner: CreateInvoiceItemBuilder<'a>,
}
impl<'a> CreateInvoiceItem<'a> {
    /// Construct a new `CreateInvoiceItem`.
    pub fn new(customer: &'a str) -> Self {
        Self { inner: CreateInvoiceItemBuilder::new(customer) }
    }
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// Passing in a negative `amount` will reduce the `amount_due` on the invoice.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: stripe_types::Currency) -> Self {
        self.inner.currency = Some(currency);
        self
    }
    /// An arbitrary string which you can attach to the invoice item.
    /// The description is displayed in the invoice for easy tracking.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Controls whether discounts apply to this invoice item.
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    pub fn discountable(mut self, discountable: bool) -> Self {
        self.inner.discountable = Some(discountable);
        self
    }
    /// The coupons and promotion codes to redeem into discounts for the invoice item or invoice line item.
    pub fn discounts(mut self, discounts: &'a [DiscountsDataParam<'a>]) -> Self {
        self.inner.discounts = Some(discounts);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// The ID of an existing invoice to add this invoice item to.
    /// When left blank, the invoice item will be added to the next upcoming scheduled invoice.
    /// This is useful when adding invoice items in response to an invoice.created webhook.
    /// You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice.
    pub fn invoice(mut self, invoice: &'a str) -> Self {
        self.inner.invoice = Some(invoice);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    pub fn period(mut self, period: Period) -> Self {
        self.inner.period = Some(period);
        self
    }
    /// The ID of the price object.
    pub fn price(mut self, price: &'a str) -> Self {
        self.inner.price = Some(price);
        self
    }
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    pub fn price_data(mut self, price_data: CreateInvoiceItemPriceData<'a>) -> Self {
        self.inner.price_data = Some(price_data);
        self
    }
    /// Non-negative integer. The quantity of units for the invoice item.
    pub fn quantity(mut self, quantity: u64) -> Self {
        self.inner.quantity = Some(quantity);
        self
    }
    /// The ID of a subscription to add this invoice item to.
    /// When left blank, the invoice item will be be added to the next upcoming scheduled invoice.
    /// When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item.
    /// Use this when you want to express that an invoice item has been accrued within the context of a particular subscription.
    pub fn subscription(mut self, subscription: &'a str) -> Self {
        self.inner.subscription = Some(subscription);
        self
    }
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub fn tax_behavior(mut self, tax_behavior: CreateInvoiceItemTaxBehavior) -> Self {
        self.inner.tax_behavior = Some(tax_behavior);
        self
    }
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub fn tax_code(mut self, tax_code: &'a str) -> Self {
        self.inner.tax_code = Some(tax_code);
        self
    }
    /// The tax rates which apply to the invoice item.
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    pub fn tax_rates(mut self, tax_rates: &'a [&'a str]) -> Self {
        self.inner.tax_rates = Some(tax_rates);
        self
    }
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// This `unit_amount` will be multiplied by the quantity to get the full amount.
    /// Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice.
    pub fn unit_amount(mut self, unit_amount: i64) -> Self {
        self.inner.unit_amount = Some(unit_amount);
        self
    }
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    pub fn unit_amount_decimal(mut self, unit_amount_decimal: &'a str) -> Self {
        self.inner.unit_amount_decimal = Some(unit_amount_decimal);
        self
    }
}
impl CreateInvoiceItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for CreateInvoiceItem<'_> {
    type Output = stripe_shared::InvoiceItem;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/invoiceitems").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateInvoiceItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<&'a [DiscountsDataParam<'a>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data: Option<UpdateInvoiceItemPriceData<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<UpdateInvoiceItemTaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateInvoiceItemBuilder<'a> {
    fn new() -> Self {
        Self {
            amount: None,
            description: None,
            discountable: None,
            discounts: None,
            expand: None,
            metadata: None,
            period: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_behavior: None,
            tax_code: None,
            tax_rates: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceItemPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateInvoiceItemPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateInvoiceItemPriceData<'a> {
    pub fn new(currency: stripe_types::Currency, product: &'a str) -> Self {
        Self { currency, product, tax_behavior: None, unit_amount: None, unit_amount_decimal: None }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateInvoiceItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceItemPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceItemPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceItemPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceItemPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceItemPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateInvoiceItemPriceDataTaxBehavior")
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceItemTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateInvoiceItemTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceItemTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceItemTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateInvoiceItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceItemTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceItemTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceItemTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for UpdateInvoiceItemTaxBehavior"))
    }
}
/// Updates the amount or description of an invoice item on an upcoming invoice.
/// Updating an invoice item is only possible before the invoice it’s attached to is closed.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceItem<'a> {
    inner: UpdateInvoiceItemBuilder<'a>,
    invoiceitem: &'a stripe_shared::InvoiceItemId,
}
impl<'a> UpdateInvoiceItem<'a> {
    /// Construct a new `UpdateInvoiceItem`.
    pub fn new(invoiceitem: &'a stripe_shared::InvoiceItemId) -> Self {
        Self { invoiceitem, inner: UpdateInvoiceItemBuilder::new() }
    }
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// If you want to apply a credit to the customer's account, pass a negative amount.
    pub fn amount(mut self, amount: i64) -> Self {
        self.inner.amount = Some(amount);
        self
    }
    /// An arbitrary string which you can attach to the invoice item.
    /// The description is displayed in the invoice for easy tracking.
    pub fn description(mut self, description: &'a str) -> Self {
        self.inner.description = Some(description);
        self
    }
    /// Controls whether discounts apply to this invoice item.
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    /// Cannot be set to true for prorations.
    pub fn discountable(mut self, discountable: bool) -> Self {
        self.inner.discountable = Some(discountable);
        self
    }
    /// The coupons, promotion codes & existing discounts which apply to the invoice item or invoice line item.
    /// Item discounts are applied before invoice discounts.
    /// Pass an empty string to remove previously-defined discounts.
    pub fn discounts(mut self, discounts: &'a [DiscountsDataParam<'a>]) -> Self {
        self.inner.discounts = Some(discounts);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    pub fn period(mut self, period: Period) -> Self {
        self.inner.period = Some(period);
        self
    }
    /// The ID of the price object.
    pub fn price(mut self, price: &'a str) -> Self {
        self.inner.price = Some(price);
        self
    }
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    pub fn price_data(mut self, price_data: UpdateInvoiceItemPriceData<'a>) -> Self {
        self.inner.price_data = Some(price_data);
        self
    }
    /// Non-negative integer. The quantity of units for the invoice item.
    pub fn quantity(mut self, quantity: u64) -> Self {
        self.inner.quantity = Some(quantity);
        self
    }
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub fn tax_behavior(mut self, tax_behavior: UpdateInvoiceItemTaxBehavior) -> Self {
        self.inner.tax_behavior = Some(tax_behavior);
        self
    }
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub fn tax_code(mut self, tax_code: &'a str) -> Self {
        self.inner.tax_code = Some(tax_code);
        self
    }
    /// The tax rates which apply to the invoice item.
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    /// Pass an empty string to remove previously-defined tax rates.
    pub fn tax_rates(mut self, tax_rates: &'a [&'a str]) -> Self {
        self.inner.tax_rates = Some(tax_rates);
        self
    }
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    pub fn unit_amount(mut self, unit_amount: i64) -> Self {
        self.inner.unit_amount = Some(unit_amount);
        self
    }
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    pub fn unit_amount_decimal(mut self, unit_amount_decimal: &'a str) -> Self {
        self.inner.unit_amount_decimal = Some(unit_amount_decimal);
        self
    }
}
impl UpdateInvoiceItem<'_> {
    /// Send the request and return the deserialized response.
    pub async fn send<C: StripeClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send(client).await
    }

    /// Send the request and return the deserialized response, blocking until completion.
    pub fn send_blocking<C: StripeBlockingClient>(
        &self,
        client: &C,
    ) -> Result<<Self as StripeRequest>::Output, C::Err> {
        self.customize().send_blocking(client)
    }
}

impl StripeRequest for UpdateInvoiceItem<'_> {
    type Output = stripe_shared::InvoiceItem;

    fn build(&self) -> RequestBuilder {
        let invoiceitem = self.invoiceitem;
        RequestBuilder::new(StripeMethod::Post, format!("/invoiceitems/{invoiceitem}"))
            .form(&self.inner)
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DiscountsDataParam<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<&'a str>,
}
impl<'a> DiscountsDataParam<'a> {
    pub fn new() -> Self {
        Self { coupon: None, discount: None, promotion_code: None }
    }
}
impl<'a> Default for DiscountsDataParam<'a> {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct Period {
    /// The end of the period, which must be greater than or equal to the start. This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period. This value is inclusive.
    pub start: stripe_types::Timestamp,
}
impl Period {
    pub fn new(end: stripe_types::Timestamp, start: stripe_types::Timestamp) -> Self {
        Self { end, start }
    }
}
