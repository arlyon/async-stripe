use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListInvoiceInvoiceLineItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListInvoiceInvoiceLineItemBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// When retrieving an invoice, you’ll get a **lines** property containing the total count of line items and the first handful of those items.
/// There is also a URL where you can retrieve the full (paginated) list of line items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListInvoiceInvoiceLineItem {
    inner: ListInvoiceInvoiceLineItemBuilder,
    invoice: stripe_shared::InvoiceId,
}
impl ListInvoiceInvoiceLineItem {
    /// Construct a new `ListInvoiceInvoiceLineItem`.
    pub fn new(invoice: impl Into<stripe_shared::InvoiceId>) -> Self {
        Self { invoice: invoice.into(), inner: ListInvoiceInvoiceLineItemBuilder::new() }
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
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: impl Into<i64>) -> Self {
        self.inner.limit = Some(limit.into());
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
impl ListInvoiceInvoiceLineItem {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::InvoiceLineItem>> {
        let invoice = &self.invoice;

        stripe_client_core::ListPaginator::new_list(
            format!("/invoices/{invoice}/lines"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListInvoiceInvoiceLineItem {
    type Output = stripe_types::List<stripe_shared::InvoiceLineItem>;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        RequestBuilder::new(StripeMethod::Get, format!("/invoices/{invoice}/lines"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateInvoiceLineItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<UpdateInvoiceLineItemDiscounts>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    period: Option<UpdateInvoiceLineItemPeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    price_data: Option<UpdateInvoiceLineItemPriceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pricing: Option<UpdateInvoiceLineItemPricing>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_amounts: Option<Vec<UpdateInvoiceLineItemTaxAmounts>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rates: Option<Vec<String>>,
}
impl UpdateInvoiceLineItemBuilder {
    fn new() -> Self {
        Self {
            amount: None,
            description: None,
            discountable: None,
            discounts: None,
            expand: None,
            metadata: None,
            period: None,
            price_data: None,
            pricing: None,
            quantity: None,
            tax_amounts: None,
            tax_rates: None,
        }
    }
}
/// The coupons, promotion codes & existing discounts which apply to the line item.
/// Item discounts are applied before invoice discounts.
/// Pass an empty string to remove previously-defined discounts.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemDiscounts {
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
impl UpdateInvoiceLineItemDiscounts {
    pub fn new() -> Self {
        Self { coupon: None, discount: None, promotion_code: None }
    }
}
impl Default for UpdateInvoiceLineItemDiscounts {
    fn default() -> Self {
        Self::new()
    }
}
/// The period associated with this invoice item.
/// When set to different values, the period will be rendered on the invoice.
/// If you have [Stripe Revenue Recognition](https://docs.stripe.com/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
/// See the [Revenue Recognition documentation](https://docs.stripe.com/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemPeriod {
    /// The end of the period, which must be greater than or equal to the start. This value is inclusive.
    pub end: stripe_types::Timestamp,
    /// The start of the period. This value is inclusive.
    pub start: stripe_types::Timestamp,
}
impl UpdateInvoiceLineItemPeriod {
    pub fn new(
        end: impl Into<stripe_types::Timestamp>,
        start: impl Into<stripe_types::Timestamp>,
    ) -> Self {
        Self { end: end.into(), start: start.into() }
    }
}
/// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the [Product](https://docs.stripe.com/api/products) that this [Price](https://docs.stripe.com/api/prices) will belong to.
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline.
    /// One of `product` or `product_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<UpdateInvoiceLineItemPriceDataProductData>,
    /// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateInvoiceLineItemPriceDataTaxBehavior>,
    /// A non-negative integer in cents (or local equivalent) representing how much to charge.
    /// One of `unit_amount` or `unit_amount_decimal` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpdateInvoiceLineItemPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>) -> Self {
        Self {
            currency: currency.into(),
            product: None,
            product_data: None,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// Data used to generate a new [Product](https://docs.stripe.com/api/products) object inline.
/// One of `product` or `product_data` is required.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemPriceDataProductData {
    /// The product's description, meant to be displayable to the customer.
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The product's name, meant to be displayable to the customer.
    pub name: String,
    /// A [tax code](https://docs.stripe.com/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// A label that represents units of this product.
    /// When set, this will be included in customers' receipts, invoices, Checkout, and the customer portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
}
impl UpdateInvoiceLineItemPriceDataProductData {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            description: None,
            images: None,
            metadata: None,
            name: name.into(),
            tax_code: None,
            unit_label: None,
        }
    }
}
/// Only required if a [default tax behavior](https://docs.stripe.com/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateInvoiceLineItemPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateInvoiceLineItemPriceDataTaxBehavior {
    pub fn as_str(&self) -> &str {
        use UpdateInvoiceLineItemPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateInvoiceLineItemPriceDataTaxBehavior {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceLineItemPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateInvoiceLineItemPriceDataTaxBehavior"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceLineItemPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The pricing information for the invoice item.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemPricing {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
}
impl UpdateInvoiceLineItemPricing {
    pub fn new() -> Self {
        Self { price: None }
    }
}
impl Default for UpdateInvoiceLineItemPricing {
    fn default() -> Self {
        Self::new()
    }
}
/// A list of up to 10 tax amounts for this line item.
/// This can be useful if you calculate taxes on your own or use a third-party to calculate them.
/// You cannot set tax amounts if any line item has [tax_rates](https://docs.stripe.com/api/invoices/line_item#invoice_line_item_object-tax_rates) or if the invoice has [default_tax_rates](https://docs.stripe.com/api/invoices/object#invoice_object-default_tax_rates) or uses [automatic tax](https://docs.stripe.com/tax/invoicing).
/// Pass an empty string to remove previously defined tax amounts.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemTaxAmounts {
    /// The amount, in cents (or local equivalent), of the tax.
    pub amount: i64,
    /// Data to find or create a TaxRate object.
    ///
    /// Stripe automatically creates or reuses a TaxRate object for each tax amount.
    /// If the `tax_rate_data` exactly matches a previous value, Stripe will reuse the TaxRate object.
    /// TaxRate objects created automatically by Stripe are immediately archived, do not appear in the line item’s `tax_rates`, and cannot be directly added to invoices, payments, or line items.
    pub tax_rate_data: UpdateInvoiceLineItemTaxAmountsTaxRateData,
    /// The reasoning behind this tax, for example, if the product is tax exempt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxability_reason: Option<UpdateInvoiceLineItemTaxAmountsTaxabilityReason>,
    /// The amount on which tax is calculated, in cents (or local equivalent).
    pub taxable_amount: i64,
}
impl UpdateInvoiceLineItemTaxAmounts {
    pub fn new(
        amount: impl Into<i64>,
        tax_rate_data: impl Into<UpdateInvoiceLineItemTaxAmountsTaxRateData>,
        taxable_amount: impl Into<i64>,
    ) -> Self {
        Self {
            amount: amount.into(),
            tax_rate_data: tax_rate_data.into(),
            taxability_reason: None,
            taxable_amount: taxable_amount.into(),
        }
    }
}
/// Data to find or create a TaxRate object.
///
/// Stripe automatically creates or reuses a TaxRate object for each tax amount.
/// If the `tax_rate_data` exactly matches a previous value, Stripe will reuse the TaxRate object.
/// TaxRate objects created automatically by Stripe are immediately archived, do not appear in the line item’s `tax_rates`, and cannot be directly added to invoices, payments, or line items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItemTaxAmountsTaxRateData {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// An arbitrary string attached to the tax rate for your internal use only.
    /// It will not be visible to your customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The display name of the tax rate, which will be shown to users.
    pub display_name: String,
    /// This specifies if the tax rate is inclusive or exclusive.
    pub inclusive: bool,
    /// The jurisdiction for the tax rate.
    /// You can use this label field for tax reporting purposes.
    /// It also appears on your customer’s invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<String>,
    /// The level of the jurisdiction that imposes this tax rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction_level: Option<UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel>,
    /// The statutory tax rate percent.
    /// This field accepts decimal values between 0 and 100 inclusive with at most 4 decimal places.
    /// To accommodate fixed-amount taxes, set the percentage to zero.
    /// Stripe will not display zero percentages on the invoice unless the `amount` of the tax is also zero.
    pub percentage: f64,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2:US), without country prefix.
    /// For example, "NY" for New York, United States.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The high-level tax type, such as `vat` or `sales_tax`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type: Option<UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType>,
}
impl UpdateInvoiceLineItemTaxAmountsTaxRateData {
    pub fn new(
        display_name: impl Into<String>,
        inclusive: impl Into<bool>,
        percentage: impl Into<f64>,
    ) -> Self {
        Self {
            country: None,
            description: None,
            display_name: display_name.into(),
            inclusive: inclusive.into(),
            jurisdiction: None,
            jurisdiction_level: None,
            percentage: percentage.into(),
            state: None,
            tax_type: None,
        }
    }
}
/// The level of the jurisdiction that imposes this tax rate.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel {
    City,
    Country,
    County,
    District,
    Multiple,
    State,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel {
    pub fn as_str(&self) -> &str {
        use UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel::*;
        match self {
            City => "city",
            Country => "country",
            County => "county",
            District => "district",
            Multiple => "multiple",
            State => "state",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel::*;
        match s {
            "city" => Ok(City),
            "country" => Ok(Country),
            "county" => Ok(County),
            "district" => Ok(District),
            "multiple" => Ok(Multiple),
            "state" => Ok(State),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceLineItemTaxAmountsTaxRateDataJurisdictionLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The high-level tax type, such as `vat` or `sales_tax`.
#[derive(Clone, Eq, PartialEq)]
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
    RetailDeliveryFee,
    Rst,
    SalesTax,
    ServiceTax,
    Vat,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    pub fn as_str(&self) -> &str {
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
            RetailDeliveryFee => "retail_delivery_fee",
            Rst => "rst",
            SalesTax => "sales_tax",
            ServiceTax => "service_tax",
            Vat => "vat",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    type Err = std::convert::Infallible;
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
            "retail_delivery_fee" => Ok(RetailDeliveryFee),
            "rst" => Ok(Rst),
            "sales_tax" => Ok(SalesTax),
            "service_tax" => Ok(ServiceTax),
            "vat" => Ok(Vat),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
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
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceLineItemTaxAmountsTaxRateDataTaxType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The reasoning behind this tax, for example, if the product is tax exempt.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateInvoiceLineItemTaxAmountsTaxabilityReason {
    CustomerExempt,
    NotCollecting,
    NotSubjectToTax,
    NotSupported,
    PortionProductExempt,
    PortionReducedRated,
    PortionStandardRated,
    ProductExempt,
    ProductExemptHoliday,
    ProportionallyRated,
    ReducedRated,
    ReverseCharge,
    StandardRated,
    TaxableBasisReduced,
    ZeroRated,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateInvoiceLineItemTaxAmountsTaxabilityReason {
    pub fn as_str(&self) -> &str {
        use UpdateInvoiceLineItemTaxAmountsTaxabilityReason::*;
        match self {
            CustomerExempt => "customer_exempt",
            NotCollecting => "not_collecting",
            NotSubjectToTax => "not_subject_to_tax",
            NotSupported => "not_supported",
            PortionProductExempt => "portion_product_exempt",
            PortionReducedRated => "portion_reduced_rated",
            PortionStandardRated => "portion_standard_rated",
            ProductExempt => "product_exempt",
            ProductExemptHoliday => "product_exempt_holiday",
            ProportionallyRated => "proportionally_rated",
            ReducedRated => "reduced_rated",
            ReverseCharge => "reverse_charge",
            StandardRated => "standard_rated",
            TaxableBasisReduced => "taxable_basis_reduced",
            ZeroRated => "zero_rated",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateInvoiceLineItemTaxAmountsTaxabilityReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateInvoiceLineItemTaxAmountsTaxabilityReason::*;
        match s {
            "customer_exempt" => Ok(CustomerExempt),
            "not_collecting" => Ok(NotCollecting),
            "not_subject_to_tax" => Ok(NotSubjectToTax),
            "not_supported" => Ok(NotSupported),
            "portion_product_exempt" => Ok(PortionProductExempt),
            "portion_reduced_rated" => Ok(PortionReducedRated),
            "portion_standard_rated" => Ok(PortionStandardRated),
            "product_exempt" => Ok(ProductExempt),
            "product_exempt_holiday" => Ok(ProductExemptHoliday),
            "proportionally_rated" => Ok(ProportionallyRated),
            "reduced_rated" => Ok(ReducedRated),
            "reverse_charge" => Ok(ReverseCharge),
            "standard_rated" => Ok(StandardRated),
            "taxable_basis_reduced" => Ok(TaxableBasisReduced),
            "zero_rated" => Ok(ZeroRated),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateInvoiceLineItemTaxAmountsTaxabilityReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateInvoiceLineItemTaxAmountsTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateInvoiceLineItemTaxAmountsTaxabilityReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateInvoiceLineItemTaxAmountsTaxabilityReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateInvoiceLineItemTaxAmountsTaxabilityReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Updates an invoice’s line item.
/// Some fields, such as `tax_amounts`, only live on the invoice line item,.
/// so they can only be updated through this endpoint.
/// Other fields, such as `amount`, live on both the invoice.
/// item and the invoice line item, so updates on this endpoint will propagate to the invoice item as well.
/// Updating an invoice’s line item is only possible before the invoice is finalized.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateInvoiceLineItem {
    inner: UpdateInvoiceLineItemBuilder,
    invoice: stripe_shared::InvoiceId,
    line_item_id: String,
}
impl UpdateInvoiceLineItem {
    /// Construct a new `UpdateInvoiceLineItem`.
    pub fn new(
        invoice: impl Into<stripe_shared::InvoiceId>,
        line_item_id: impl Into<String>,
    ) -> Self {
        Self {
            invoice: invoice.into(),
            line_item_id: line_item_id.into(),
            inner: UpdateInvoiceLineItemBuilder::new(),
        }
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
    /// Controls whether discounts apply to this line item.
    /// Defaults to false for prorations or negative line items, and true for all other line items.
    /// Cannot be set to true for prorations.
    pub fn discountable(mut self, discountable: impl Into<bool>) -> Self {
        self.inner.discountable = Some(discountable.into());
        self
    }
    /// The coupons, promotion codes & existing discounts which apply to the line item.
    /// Item discounts are applied before invoice discounts.
    /// Pass an empty string to remove previously-defined discounts.
    pub fn discounts(mut self, discounts: impl Into<Vec<UpdateInvoiceLineItemDiscounts>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    /// For [type=subscription](https://docs.stripe.com/api/invoices/line_item#invoice_line_item_object-type) line items, the incoming metadata specified on the request is directly used to set this value, in contrast to [type=invoiceitem](api/invoices/line_item#invoice_line_item_object-type) line items, where any existing metadata on the invoice line is merged with the incoming data.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// The period associated with this invoice item.
    /// When set to different values, the period will be rendered on the invoice.
    /// If you have [Stripe Revenue Recognition](https://docs.stripe.com/revenue-recognition) enabled, the period will be used to recognize and defer revenue.
    /// See the [Revenue Recognition documentation](https://docs.stripe.com/revenue-recognition/methodology/subscriptions-and-invoicing) for details.
    pub fn period(mut self, period: impl Into<UpdateInvoiceLineItemPeriod>) -> Self {
        self.inner.period = Some(period.into());
        self
    }
    /// Data used to generate a new [Price](https://docs.stripe.com/api/prices) object inline.
    pub fn price_data(mut self, price_data: impl Into<UpdateInvoiceLineItemPriceData>) -> Self {
        self.inner.price_data = Some(price_data.into());
        self
    }
    /// The pricing information for the invoice item.
    pub fn pricing(mut self, pricing: impl Into<UpdateInvoiceLineItemPricing>) -> Self {
        self.inner.pricing = Some(pricing.into());
        self
    }
    /// Non-negative integer. The quantity of units for the line item.
    pub fn quantity(mut self, quantity: impl Into<u64>) -> Self {
        self.inner.quantity = Some(quantity.into());
        self
    }
    /// A list of up to 10 tax amounts for this line item.
    /// This can be useful if you calculate taxes on your own or use a third-party to calculate them.
    /// You cannot set tax amounts if any line item has [tax_rates](https://docs.stripe.com/api/invoices/line_item#invoice_line_item_object-tax_rates) or if the invoice has [default_tax_rates](https://docs.stripe.com/api/invoices/object#invoice_object-default_tax_rates) or uses [automatic tax](https://docs.stripe.com/tax/invoicing).
    /// Pass an empty string to remove previously defined tax amounts.
    pub fn tax_amounts(
        mut self,
        tax_amounts: impl Into<Vec<UpdateInvoiceLineItemTaxAmounts>>,
    ) -> Self {
        self.inner.tax_amounts = Some(tax_amounts.into());
        self
    }
    /// The tax rates which apply to the line item.
    /// When set, the `default_tax_rates` on the invoice do not apply to this line item.
    /// Pass an empty string to remove previously-defined tax rates.
    pub fn tax_rates(mut self, tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.tax_rates = Some(tax_rates.into());
        self
    }
}
impl UpdateInvoiceLineItem {
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

impl StripeRequest for UpdateInvoiceLineItem {
    type Output = stripe_shared::InvoiceLineItem;

    fn build(&self) -> RequestBuilder {
        let invoice = &self.invoice;
        let line_item_id = &self.line_item_id;
        RequestBuilder::new(StripeMethod::Post, format!("/invoices/{invoice}/lines/{line_item_id}"))
            .form(&self.inner)
    }
}
