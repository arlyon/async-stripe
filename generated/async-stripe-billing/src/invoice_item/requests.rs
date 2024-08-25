use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes an invoice item, removing it from an invoice.
/// Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct DeleteInvoiceItem {
    invoiceitem: stripe_shared::InvoiceItemId,
}
impl DeleteInvoiceItem {
    /// Construct a new `DeleteInvoiceItem`.
    pub fn new(invoiceitem: impl Into<stripe_shared::InvoiceItemId>) -> Self {
        Self { invoiceitem: invoiceitem.into() }
    }
}
impl DeleteInvoiceItem {
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

impl StripeRequest for DeleteInvoiceItem {
    type Output = stripe_shared::DeletedInvoiceitem;

    fn build(&self) -> RequestBuilder {
        let invoiceitem = &self.invoiceitem;
        RequestBuilder::new(StripeMethod::Delete, format!("/invoiceitems/{invoiceitem}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListInvoiceItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pending: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListInvoiceItemBuilder {
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
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListInvoiceItem {
    inner: ListInvoiceItemBuilder,
}
impl ListInvoiceItem {
    /// Construct a new `ListInvoiceItem`.
    pub fn new() -> Self {
        Self { inner: ListInvoiceItemBuilder::new() }
    }
    /// Only return invoice items that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// The identifier of the customer whose invoice items to return.
    /// If none is provided, all invoice items will be returned.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: impl Into<String>) -> Self {
        self.inner.ending_before = Some(ending_before.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Only return invoice items belonging to this invoice.
    /// If none is provided, all invoice items will be returned.
    /// If specifying an invoice, no customer identifier is needed.
    pub fn invoice(mut self, invoice: impl Into<String>) -> Self {
        self.inner.invoice = Some(invoice.into());
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
        self
    }
    /// Set to `true` to only show pending invoice items, which are not yet attached to any invoices.
    /// Set to `false` to only show invoice items already attached to invoices.
    /// If unspecified, no filter is applied.
    pub fn pending(mut self, pending: impl Into<bool>) -> Self {
        self.inner.pending = Some(pending.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListInvoiceItem {
    fn default() -> Self {
        Self::new()
    }
}
impl ListInvoiceItem {
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
        stripe_client_core::ListPaginator::new_list("/invoiceitems", &self.inner)
    }
}

impl StripeRequest for ListInvoiceItem {
    type Output = stripe_types::List<stripe_shared::InvoiceItem>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/invoiceitems").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveInvoiceItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveInvoiceItemBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the invoice item with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveInvoiceItem {
    inner: RetrieveInvoiceItemBuilder,
    invoiceitem: stripe_shared::InvoiceItemId,
}
impl RetrieveInvoiceItem {
    /// Construct a new `RetrieveInvoiceItem`.
    pub fn new(invoiceitem: impl Into<stripe_shared::InvoiceItemId>) -> Self {
        Self { invoiceitem: invoiceitem.into(), inner: RetrieveInvoiceItemBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveInvoiceItem {
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

impl StripeRequest for RetrieveInvoiceItem {
    type Output = stripe_shared::InvoiceItem;

    fn build(&self) -> RequestBuilder {
        let invoiceitem = &self.invoiceitem;
        RequestBuilder::new(StripeMethod::Get, format!("/invoiceitems/{invoiceitem}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateInvoiceItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    customer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data: Option<CreateInvoiceItemPriceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<CreateInvoiceItemTaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount_decimal: Option<String>,
}
impl CreateInvoiceItemBuilder {
    fn new(customer: impl Into<String>) -> Self {
        Self {
            amount: None,
            currency: None,
            customer: customer.into(),
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateInvoiceItemPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
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
    pub unit_amount_decimal: Option<String>,
}
impl CreateInvoiceItemPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
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
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateInvoiceItem {
    inner: CreateInvoiceItemBuilder,
}
impl CreateInvoiceItem {
    /// Construct a new `CreateInvoiceItem`.
    pub fn new(customer: impl Into<String>) -> Self {
        Self { inner: CreateInvoiceItemBuilder::new(customer.into()) }
    }
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// Passing in a negative `amount` will reduce the `amount_due` on the invoice.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
        self
    }
    /// An arbitrary string which you can attach to the invoice item.
    /// The description is displayed in the invoice for easy tracking.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Controls whether discounts apply to this invoice item.
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    pub fn discountable(mut self, discountable: impl Into<bool>) -> Self {
        self.inner.discountable = Some(discountable.into());
        self
    }
    /// The coupons and promotion codes to redeem into discounts for the invoice item or invoice line item.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The ID of an existing invoice to add this invoice item to.
    /// When left blank, the invoice item will be added to the next upcoming scheduled invoice.
    /// This is useful when adding invoice items in response to an invoice.created webhook.
    /// You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice.
    pub fn invoice(mut self, invoice: impl Into<String>) -> Self {
        self.inner.invoice = Some(invoice.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    pub fn period(mut self, period: impl Into<Period>) -> Self {
        self.inner.period = Some(period.into());
        self
    }
    /// The ID of the price object.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.inner.price = Some(price.into());
        self
    }
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    pub fn price_data(mut self, price_data: impl Into<CreateInvoiceItemPriceData>) -> Self {
        self.inner.price_data = Some(price_data.into());
        self
    }
    /// Non-negative integer. The quantity of units for the invoice item.
    pub fn quantity(mut self, quantity: impl Into<u64>) -> Self {
        self.inner.quantity = Some(quantity.into());
        self
    }
    /// The ID of a subscription to add this invoice item to.
    /// When left blank, the invoice item will be be added to the next upcoming scheduled invoice.
    /// When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item.
    /// Use this when you want to express that an invoice item has been accrued within the context of a particular subscription.
    pub fn subscription(mut self, subscription: impl Into<String>) -> Self {
        self.inner.subscription = Some(subscription.into());
        self
    }
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub fn tax_behavior(mut self, tax_behavior: impl Into<CreateInvoiceItemTaxBehavior>) -> Self {
        self.inner.tax_behavior = Some(tax_behavior.into());
        self
    }
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub fn tax_code(mut self, tax_code: impl Into<String>) -> Self {
        self.inner.tax_code = Some(tax_code.into());
        self
    }
    /// The tax rates which apply to the invoice item.
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    pub fn tax_rates(mut self, tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.tax_rates = Some(tax_rates.into());
        self
    }
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// This `unit_amount` will be multiplied by the quantity to get the full amount.
    /// Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice.
    pub fn unit_amount(mut self, unit_amount: impl Into<i64>) -> Self {
        self.inner.unit_amount = Some(unit_amount.into());
        self
    }
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    pub fn unit_amount_decimal(mut self, unit_amount_decimal: impl Into<String>) -> Self {
        self.inner.unit_amount_decimal = Some(unit_amount_decimal.into());
        self
    }
}
impl CreateInvoiceItem {
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

impl StripeRequest for CreateInvoiceItem {
    type Output = stripe_shared::InvoiceItem;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/invoiceitems").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct UpdateInvoiceItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data: Option<UpdateInvoiceItemPriceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<UpdateInvoiceItemTaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unit_amount_decimal: Option<String>,
}
impl UpdateInvoiceItemBuilder {
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
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateInvoiceItemPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
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
    pub unit_amount_decimal: Option<String>,
}
impl UpdateInvoiceItemPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
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
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdateInvoiceItem {
    inner: UpdateInvoiceItemBuilder,
    invoiceitem: stripe_shared::InvoiceItemId,
}
impl UpdateInvoiceItem {
    /// Construct a new `UpdateInvoiceItem`.
    pub fn new(invoiceitem: impl Into<stripe_shared::InvoiceItemId>) -> Self {
        Self { invoiceitem: invoiceitem.into(), inner: UpdateInvoiceItemBuilder::new() }
    }
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// If you want to apply a credit to the customer's account, pass a negative amount.
    pub fn amount(mut self, amount: impl Into<i64>) -> Self {
        self.inner.amount = Some(amount.into());
        self
    }
    /// An arbitrary string which you can attach to the invoice item.
    /// The description is displayed in the invoice for easy tracking.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// Controls whether discounts apply to this invoice item.
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    /// Cannot be set to true for prorations.
    pub fn discountable(mut self, discountable: impl Into<bool>) -> Self {
        self.inner.discountable = Some(discountable.into());
        self
    }
    /// The coupons, promotion codes & existing discounts which apply to the invoice item or invoice line item.
    /// Item discounts are applied before invoice discounts.
    /// Pass an empty string to remove previously-defined discounts.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    pub fn period(mut self, period: impl Into<Period>) -> Self {
        self.inner.period = Some(period.into());
        self
    }
    /// The ID of the price object.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.inner.price = Some(price.into());
        self
    }
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    pub fn price_data(mut self, price_data: impl Into<UpdateInvoiceItemPriceData>) -> Self {
        self.inner.price_data = Some(price_data.into());
        self
    }
    /// Non-negative integer. The quantity of units for the invoice item.
    pub fn quantity(mut self, quantity: impl Into<u64>) -> Self {
        self.inner.quantity = Some(quantity.into());
        self
    }
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    pub fn tax_behavior(mut self, tax_behavior: impl Into<UpdateInvoiceItemTaxBehavior>) -> Self {
        self.inner.tax_behavior = Some(tax_behavior.into());
        self
    }
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    pub fn tax_code(mut self, tax_code: impl Into<String>) -> Self {
        self.inner.tax_code = Some(tax_code.into());
        self
    }
    /// The tax rates which apply to the invoice item.
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    /// Pass an empty string to remove previously-defined tax rates.
    pub fn tax_rates(mut self, tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.tax_rates = Some(tax_rates.into());
        self
    }
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    pub fn unit_amount(mut self, unit_amount: impl Into<i64>) -> Self {
        self.inner.unit_amount = Some(unit_amount.into());
        self
    }
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    pub fn unit_amount_decimal(mut self, unit_amount_decimal: impl Into<String>) -> Self {
        self.inner.unit_amount_decimal = Some(unit_amount_decimal.into());
        self
    }
}
impl UpdateInvoiceItem {
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

impl StripeRequest for UpdateInvoiceItem {
    type Output = stripe_shared::InvoiceItem;

    fn build(&self) -> RequestBuilder {
        let invoiceitem = &self.invoiceitem;
        RequestBuilder::new(StripeMethod::Post, format!("/invoiceitems/{invoiceitem}"))
            .form(&self.inner)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct DiscountsDataParam {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}
impl DiscountsDataParam {
    pub fn new() -> Self {
        Self { coupon: None, discount: None, promotion_code: None }
    }
}
impl Default for DiscountsDataParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct Period {
    /// The end of the period, which must be greater than or equal to the start. This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period. This value is inclusive.
    pub start: stripe_types::Timestamp,
}
impl Period {
    pub fn new(
        end: impl Into<stripe_types::Timestamp>,
        start: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self { end: end.into(), start: start.into() }
    }
}
