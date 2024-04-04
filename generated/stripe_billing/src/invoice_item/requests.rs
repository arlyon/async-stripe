#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteInvoiceItem {}
impl DeleteInvoiceItem {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteInvoiceItem {
    /// Deletes an invoice item, removing it from an invoice.
    /// Deleting invoice items is only possible when they’re not attached to invoices, or if it’s attached to a draft invoice.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoiceitem: &stripe_shared::InvoiceItemId,
    ) -> stripe::Response<stripe_shared::DeletedInvoiceitem> {
        client.send_form(&format!("/invoiceitems/{invoiceitem}"), self, http_types::Method::Delete)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListInvoiceItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// The identifier of the customer whose invoice items to return.
    /// If none is provided, all invoice items will be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Only return invoice items belonging to this invoice.
    /// If none is provided, all invoice items will be returned.
    /// If specifying an invoice, no customer identifier is needed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Set to `true` to only show pending invoice items, which are not yet attached to any invoices.
    /// Set to `false` to only show invoice items already attached to invoices.
    /// If unspecified, no filter is applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListInvoiceItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListInvoiceItem<'a> {
    /// Returns a list of your invoice items.
    /// Invoice items are returned sorted by creation date, with the most recently created invoice items appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_shared::InvoiceItem>> {
        client.get_query("/invoiceitems", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::List<stripe_shared::InvoiceItem>> {
        stripe::ListPaginator::from_list_params("/invoiceitems", self)
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
impl<'a> RetrieveInvoiceItem<'a> {
    /// Retrieves the invoice item with the given ID.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoiceitem: &stripe_shared::InvoiceItemId,
    ) -> stripe::Response<stripe_shared::InvoiceItem> {
        client.get_query(&format!("/invoiceitems/{invoiceitem}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateInvoiceItem<'a> {
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// Passing in a negative `amount` will reduce the `amount_due` on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
    /// The ID of the customer who will be billed when this invoice item is billed.
    pub customer: &'a str,
    /// An arbitrary string which you can attach to the invoice item.
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Controls whether discounts apply to this invoice item.
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    /// The coupons to redeem into discounts for the invoice item or invoice line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [DiscountsDataParam<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The ID of an existing invoice to add this invoice item to.
    /// When left blank, the invoice item will be added to the next upcoming scheduled invoice.
    /// This is useful when adding invoice items in response to an invoice.created webhook.
    /// You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<&'a str>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateInvoiceItemPriceData<'a>>,
    /// Non-negative integer. The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The ID of a subscription to add this invoice item to.
    /// When left blank, the invoice item will be be added to the next upcoming scheduled invoice.
    /// When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item.
    /// Use this when you want to express that an invoice item has been accrued within the context of a particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<&'a str>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateInvoiceItemTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The tax rates which apply to the invoice item.
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// This `unit_amount` will be multiplied by the quantity to get the full amount.
    /// Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateInvoiceItem<'a> {
    pub fn new(customer: &'a str) -> Self {
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceItemPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateInvoiceItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
impl<'a> CreateInvoiceItem<'a> {
    /// Creates an item to be added to a draft invoice (up to 250 items per invoice).
    /// If no invoice is specified, the item will be on the next invoice created for the customer specified.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_shared::InvoiceItem> {
        client.send_form("/invoiceitems", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceItem<'a> {
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// If you want to apply a credit to the customer's account, pass a negative amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// An arbitrary string which you can attach to the invoice item.
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// Controls whether discounts apply to this invoice item.
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    /// Cannot be set to true for prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    /// The coupons & existing discounts which apply to the invoice item or invoice line item.
    /// Item discounts are applied before invoice discounts.
    /// Pass an empty string to remove previously-defined discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [DiscountsDataParam<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateInvoiceItemPriceData<'a>>,
    /// Non-negative integer. The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateInvoiceItemTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The tax rates which apply to the invoice item.
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    /// Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateInvoiceItem<'a> {
    pub fn new() -> Self {
        Self::default()
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceItemPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceItemTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
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
impl<'a> UpdateInvoiceItem<'a> {
    /// Updates the amount or description of an invoice item on an upcoming invoice.
    /// Updating an invoice item is only possible before the invoice it’s attached to is closed.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoiceitem: &stripe_shared::InvoiceItemId,
    ) -> stripe::Response<stripe_shared::InvoiceItem> {
        client.send_form(&format!("/invoiceitems/{invoiceitem}"), self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DiscountsDataParam<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> DiscountsDataParam<'a> {
    pub fn new() -> Self {
        Self::default()
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
