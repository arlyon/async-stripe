#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListInvoiceLineItem<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
}
impl<'a> ListInvoiceLineItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListInvoiceLineItem<'a> {
    /// When retrieving an invoice, you’ll get a **lines** property containing the total count of line items and the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::Response<stripe_types::List<stripe_types::InvoiceLineItem>> {
        client.get_query(&format!("/invoices/{invoice}/lines", invoice = invoice), self)
    }
    pub fn paginate(
        self,
        invoice: &stripe_types::invoice::InvoiceId,
    ) -> stripe::ListPaginator<stripe_types::InvoiceLineItem> {
        stripe::ListPaginator::from_params(
            &format!("/invoices/{invoice}/lines", invoice = invoice),
            self,
        )
    }
}
impl<'a> stripe::PaginationParams for ListInvoiceLineItem<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceLineItem<'a> {
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
    /// Controls whether discounts apply to this line item.
    ///
    /// Defaults to false for prorations or negative line items, and true for all other line items.
    /// Cannot be set to true for prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    /// The coupons & existing discounts which apply to the line item.
    ///
    /// Item discounts are applied before invoice discounts.
    /// Pass an empty string to remove previously-defined discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [UpdateInvoiceLineItemDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The period associated with this invoice item.
    ///
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<UpdateInvoiceLineItemPeriod>,
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateInvoiceLineItemPriceData<'a>>,
    /// Non-negative integer.
    ///
    /// The quantity of units for the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// A list of up to 10 tax amounts for this line item.
    ///
    /// This can be useful if you calculate taxes on your own or use a third-party to calculate them.
    /// You cannot set tax amounts if any line item has [tax_rates](https://stripe.com/docs/api/invoices/line_item#invoice_line_item_object-tax_rates) or if the invoice has [default_tax_rates](https://stripe.com/docs/api/invoices/object#invoice_object-default_tax_rates) or uses [automatic tax](https://stripe.com/docs/tax/invoicing).
    /// Pass an empty string to remove previously defined tax amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<&'a [UpdateInvoiceLineItemTaxAmounts<'a>]>,
    /// The tax rates which apply to the line item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this line item.
    /// Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpdateInvoiceLineItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The coupons & existing discounts which apply to the line item.
///
/// Item discounts are applied before invoice discounts.
/// Pass an empty string to remove previously-defined discounts.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateInvoiceLineItemDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> UpdateInvoiceLineItemDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The period associated with this invoice item.
///
/// When set to different values, the period will be rendered on the invoice.
/// If you have [Stripe Revenue Recognition](https://stripe.com/docs/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
/// See the [Revenue Recognition documentation](https://stripe.com/docs/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    ///
    /// This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period.
    ///
    /// This value is inclusive.
    pub start: stripe_types::Timestamp,
}
impl UpdateInvoiceLineItemPeriod {
    pub fn new(end: stripe_types::Timestamp, start: stripe_types::Timestamp) -> Self {
        Self { end, start }
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    ///
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<&'a str>,
    /// Data used to generate a new product object inline.
    ///
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<UpdateInvoiceLineItemPriceDataProductData<'a>>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    ///
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateInvoiceLineItemPriceDataTaxBehavior>,
    /// A non-negative integer in cents (or local equivalent) representing how much to charge.
    ///
    /// One of `unit_amount` or `unit_amount_decimal` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateInvoiceLineItemPriceData<'a> {
    pub fn new(currency: stripe_types::Currency) -> Self {
        Self {
            currency,
            product: Default::default(),
            product_data: Default::default(),
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// Data used to generate a new product object inline.
///
/// One of `product` or `product_data` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemPriceDataProductData<'a> {
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: &'a str,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
}
impl<'a> UpdateInvoiceLineItemPriceDataProductData<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            description: Default::default(),
            images: Default::default(),
            metadata: Default::default(),
            name,
            tax_code: Default::default(),
        }
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
///
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateInvoiceLineItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateInvoiceLineItemPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceLineItemPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceLineItemPriceDataTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceLineItemPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceLineItemPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateInvoiceLineItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceLineItemPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceLineItemPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// A list of up to 10 tax amounts for this line item.
///
/// This can be useful if you calculate taxes on your own or use a third-party to calculate them.
/// You cannot set tax amounts if any line item has [tax_rates](https://stripe.com/docs/api/invoices/line_item#invoice_line_item_object-tax_rates) or if the invoice has [default_tax_rates](https://stripe.com/docs/api/invoices/object#invoice_object-default_tax_rates) or uses [automatic tax](https://stripe.com/docs/tax/invoicing).
/// Pass an empty string to remove previously defined tax amounts.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemTaxAmounts<'a> {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// Data to find or create a TaxRate object.
    ///
    /// Stripe automatically creates or reuses a TaxRate object for each tax amount.
    ///
    /// If the `tax_rate_data` exactly matches a previous value, Stripe will reuse the TaxRate object.
    /// TaxRate objects created automatically by Stripe are immediately archived, do not appear in the line item’s `tax_rates`, and cannot be directly added to invoices, payments, or line items.
    pub tax_rate_data: UpdateInvoiceLineItemTaxAmountsTaxRateData<'a>,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: i64,
}
impl<'a> UpdateInvoiceLineItemTaxAmounts<'a> {
    pub fn new(
        amount: i64,
        tax_rate_data: UpdateInvoiceLineItemTaxAmountsTaxRateData<'a>,
        taxable_amount: i64,
    ) -> Self {
        Self { amount, tax_rate_data, taxable_amount }
    }
}
/// Data to find or create a TaxRate object.
///
/// Stripe automatically creates or reuses a TaxRate object for each tax amount.
///
/// If the `tax_rate_data` exactly matches a previous value, Stripe will reuse the TaxRate object.
/// TaxRate objects created automatically by Stripe are immediately archived, do not appear in the line item’s `tax_rates`, and cannot be directly added to invoices, payments, or line items.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemTaxAmountsTaxRateData<'a> {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
    /// An arbitrary string attached to the tax rate for your internal use only.
    ///
    /// It will not be visible to your customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The display name of the tax rate, which will be shown to users.
    pub display_name: &'a str,
    /// This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,
    /// The jurisdiction for the tax rate.
    ///
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<&'a str>,
    /// The statutory tax rate percent.
    ///
    /// This field accepts decimal values between 0 and 100 inclusive with at most 4 decimal places.
    /// To accommodate fixed-amount taxes, set the percentage to zero.
    /// Stripe will not display zero percentages on the invoice unless the `amount` of the tax is also zero.
    pub percentage: f64,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    ///
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<&'a str>,
    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType>,
}
impl<'a> UpdateInvoiceLineItemTaxAmountsTaxRateData<'a> {
    pub fn new(display_name: &'a str, inclusive: bool, percentage: f64) -> Self {
        Self {
            country: Default::default(),
            description: Default::default(),
            display_name,
            inclusive,
            jurisdiction: Default::default(),
            percentage,
            state: Default::default(),
            tax_type: Default::default(),
        }
    }
}
/// The high-level tax type, such as `vat` or `sales_tax`.
#[derive(Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    AmusementTax,
    CommunicationsTax,
    Gst,
    Hst,
    Igst,
    Jct,
    LeaseTax,
    Pst,
    Qst,
    Rst,
    SalesTax,
    ServiceTax,
    Vat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown,
}

impl UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    pub fn as_str(self) -> &'static str {
        use UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType::*;
        match self {
            AmusementTax => "amusement_tax",
            CommunicationsTax => "communications_tax",
            Gst => "gst",
            Hst => "hst",
            Igst => "igst",
            Jct => "jct",
            LeaseTax => "lease_tax",
            Pst => "pst",
            Qst => "qst",
            Rst => "rst",
            SalesTax => "sales_tax",
            ServiceTax => "service_tax",
            Vat => "vat",
            Unknown => "unknown",
        }
    }
}

impl std::str::FromStr for UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType::*;
        match s {
            "amusement_tax" => Ok(AmusementTax),
            "communications_tax" => Ok(CommunicationsTax),
            "gst" => Ok(Gst),
            "hst" => Ok(Hst),
            "igst" => Ok(Igst),
            "jct" => Ok(Jct),
            "lease_tax" => Ok(LeaseTax),
            "pst" => Ok(Pst),
            "qst" => Ok(Qst),
            "rst" => Ok(Rst),
            "sales_tax" => Ok(SalesTax),
            "service_tax" => Ok(ServiceTax),
            "vat" => Ok(Vat),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> UpdateInvoiceLineItem<'a> {
    /// Updates an invoice’s line item.
    ///
    /// Some fields, such as `tax_amounts`, only live on the invoice line item, so they can only be updated through this endpoint.
    /// Other fields, such as `amount`, live on both the invoice item and the invoice line item, so updates on this endpoint will propagate to the invoice item as well. Updating an invoice’s line item is only possible before the invoice is finalized.
    pub fn send(
        &self,
        client: &stripe::Client,
        invoice: &stripe_types::invoice::InvoiceId,
        line_item_id: &str,
    ) -> stripe::Response<stripe_types::InvoiceLineItem> {
        client.send_form(
            &format!(
                "/invoices/{invoice}/lines/{line_item_id}",
                invoice = invoice,
                line_item_id = line_item_id
            ),
            self,
            http_types::Method::Post,
        )
    }
}
