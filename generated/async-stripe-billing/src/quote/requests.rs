use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListQuoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::QuoteStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_clock: Option<String>,
}
impl ListQuoteBuilder {
    fn new() -> Self {
        Self {
            customer: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
            test_clock: None,
        }
    }
}
/// Returns a list of your quotes.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListQuote {
    inner: ListQuoteBuilder,
}
impl ListQuote {
    /// Construct a new `ListQuote`.
    pub fn new() -> Self {
        Self { inner: ListQuoteBuilder::new() }
    }
    /// The ID of the customer whose quotes will be retrieved.
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
    /// The status of the quote.
    pub fn status(mut self, status: impl Into<stripe_shared::QuoteStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
    /// Provides a list of quotes that are associated with the specified test clock.
    /// The response will not include quotes with test clocks if this and the customer parameter is not set.
    pub fn test_clock(mut self, test_clock: impl Into<String>) -> Self {
        self.inner.test_clock = Some(test_clock.into());
        self
    }
}
impl Default for ListQuote {
    fn default() -> Self {
        Self::new()
    }
}
impl ListQuote {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Quote>> {
        stripe_client_core::ListPaginator::new_list("/quotes", &self.inner)
    }
}

impl StripeRequest for ListQuote {
    type Output = stripe_types::List<stripe_shared::Quote>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/quotes").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveQuoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveQuoteBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the quote with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveQuote {
    inner: RetrieveQuoteBuilder,
    quote: stripe_shared::QuoteId,
}
impl RetrieveQuote {
    /// Construct a new `RetrieveQuote`.
    pub fn new(quote: impl Into<stripe_shared::QuoteId>) -> Self {
        Self { quote: quote.into(), inner: RetrieveQuoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveQuote {
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

impl StripeRequest for RetrieveQuote {
    type Output = stripe_shared::Quote;

    fn build(&self) -> RequestBuilder {
        let quote = &self.quote;
        RequestBuilder::new(StripeMethod::Get, format!("/quotes/{quote}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListComputedUpfrontLineItemsQuoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListComputedUpfrontLineItemsQuoteBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// When retrieving a quote, there is an includable <a href="<https://stripe.com/docs/api/quotes/object#quote_object-computed-upfront-line_items>">**computed.upfront.line_items**</a> property containing the first handful of those items.
/// There is also a URL where you can retrieve the full (paginated) list of upfront line items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListComputedUpfrontLineItemsQuote {
    inner: ListComputedUpfrontLineItemsQuoteBuilder,
    quote: stripe_shared::QuoteId,
}
impl ListComputedUpfrontLineItemsQuote {
    /// Construct a new `ListComputedUpfrontLineItemsQuote`.
    pub fn new(quote: impl Into<stripe_shared::QuoteId>) -> Self {
        Self { quote: quote.into(), inner: ListComputedUpfrontLineItemsQuoteBuilder::new() }
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
impl ListComputedUpfrontLineItemsQuote {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::CheckoutSessionItem>>
    {
        let quote = &self.quote;

        stripe_client_core::ListPaginator::new_list(
            format!("/quotes/{quote}/computed_upfront_line_items"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListComputedUpfrontLineItemsQuote {
    type Output = stripe_types::List<stripe_shared::CheckoutSessionItem>;

    fn build(&self) -> RequestBuilder {
        let quote = &self.quote;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/quotes/{quote}/computed_upfront_line_items"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ListLineItemsQuoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListLineItemsQuoteBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// When retrieving a quote, there is an includable **line_items** property containing the first handful of those items.
/// There is also a URL where you can retrieve the full (paginated) list of line items.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListLineItemsQuote {
    inner: ListLineItemsQuoteBuilder,
    quote: stripe_shared::QuoteId,
}
impl ListLineItemsQuote {
    /// Construct a new `ListLineItemsQuote`.
    pub fn new(quote: impl Into<stripe_shared::QuoteId>) -> Self {
        Self { quote: quote.into(), inner: ListLineItemsQuoteBuilder::new() }
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
impl ListLineItemsQuote {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::CheckoutSessionItem>>
    {
        let quote = &self.quote;

        stripe_client_core::ListPaginator::new_list(
            format!("/quotes/{quote}/line_items"),
            &self.inner,
        )
    }
}

impl StripeRequest for ListLineItemsQuote {
    type Output = stripe_types::List<stripe_shared::CheckoutSessionItem>;

    fn build(&self) -> RequestBuilder {
        let quote = &self.quote;
        RequestBuilder::new(StripeMethod::Get, format!("/quotes/{quote}/line_items"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateQuoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<CreateQuoteAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_method: Option<stripe_shared::QuoteCollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    from_quote: Option<CreateQuoteFromQuote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_settings: Option<CreateQuoteInvoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_items: Option<Vec<CreateQuoteLineItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_data: Option<CreateQuoteSubscriptionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    test_clock: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<TransferDataSpecs>,
}
impl CreateQuoteBuilder {
    fn new() -> Self {
        Self {
            application_fee_amount: None,
            application_fee_percent: None,
            automatic_tax: None,
            collection_method: None,
            customer: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            expand: None,
            expires_at: None,
            footer: None,
            from_quote: None,
            header: None,
            invoice_settings: None,
            line_items: None,
            metadata: None,
            on_behalf_of: None,
            subscription_data: None,
            test_clock: None,
            transfer_data: None,
        }
    }
}
/// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateQuoteAutomaticTax {
    /// Controls whether Stripe will automatically compute tax on the resulting invoices or subscriptions as well as the quote itself.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<CreateQuoteAutomaticTaxLiability>,
}
impl CreateQuoteAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateQuoteAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateQuoteAutomaticTaxLiabilityType,
}
impl CreateQuoteAutomaticTaxLiability {
    pub fn new(type_: impl Into<CreateQuoteAutomaticTaxLiabilityType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateQuoteAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl CreateQuoteAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use CreateQuoteAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateQuoteAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateQuoteAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateQuoteAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateQuoteAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateQuoteAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateQuoteAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateQuoteAutomaticTaxLiabilityType")
        })
    }
}
/// Clone an existing quote.
/// The new quote will be created in `status=draft`.
/// When using this parameter, you cannot specify any other parameters except for `expires_at`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateQuoteFromQuote {
    /// Whether this quote is a revision of the previous quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revision: Option<bool>,
    /// The `id` of the quote that will be cloned.
    pub quote: String,
}
impl CreateQuoteFromQuote {
    pub fn new(quote: impl Into<String>) -> Self {
        Self { is_revision: None, quote: quote.into() }
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateQuoteInvoiceSettings {
    /// Number of days within which a customer must pay the invoice generated by this quote.
    /// This value will be `null` for quotes where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<CreateQuoteInvoiceSettingsIssuer>,
}
impl CreateQuoteInvoiceSettings {
    pub fn new() -> Self {
        Self { days_until_due: None, issuer: None }
    }
}
impl Default for CreateQuoteInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateQuoteInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: CreateQuoteInvoiceSettingsIssuerType,
}
impl CreateQuoteInvoiceSettingsIssuer {
    pub fn new(type_: impl Into<CreateQuoteInvoiceSettingsIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateQuoteInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl CreateQuoteInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use CreateQuoteInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for CreateQuoteInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateQuoteInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateQuoteInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateQuoteInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateQuoteInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateQuoteInvoiceSettingsIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateQuoteInvoiceSettingsIssuerType")
        })
    }
}
/// A list of line items the customer is being quoted for.
/// Each line item includes information about the product, the quantity, and the resulting cost.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateQuoteLineItems {
    /// The discounts applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of the price object. One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateQuoteLineItemsPriceData>,
    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the line item.
    /// When set, the `default_tax_rates` on the quote do not apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl CreateQuoteLineItems {
    pub fn new() -> Self {
        Self { discounts: None, price: None, price_data: None, quantity: None, tax_rates: None }
    }
}
impl Default for CreateQuoteLineItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
/// One of `price` or `price_data` is required.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateQuoteLineItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateQuoteLineItemsPriceDataRecurring>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateQuoteLineItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl CreateQuoteLineItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: None,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateQuoteLineItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: CreateQuoteLineItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateQuoteLineItemsPriceDataRecurring {
    pub fn new(interval: impl Into<CreateQuoteLineItemsPriceDataRecurringInterval>) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateQuoteLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl CreateQuoteLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use CreateQuoteLineItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for CreateQuoteLineItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateQuoteLineItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateQuoteLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateQuoteLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateQuoteLineItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateQuoteLineItemsPriceDataRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateQuoteLineItemsPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateQuoteLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl CreateQuoteLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use CreateQuoteLineItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateQuoteLineItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateQuoteLineItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateQuoteLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateQuoteLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateQuoteLineItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateQuoteLineItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateQuoteLineItemsPriceDataTaxBehavior")
        })
    }
}
/// When creating a subscription or subscription schedule, the specified configuration data will be used.
/// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
/// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateQuoteSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    /// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
    /// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
    /// The `effective_date` is ignored if it is in the past when the quote is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<CreateQuoteSubscriptionDataEffectiveDate>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on the subscription or subscription schedule when the quote is accepted.
    /// If a recurring price is included in `line_items`, this field will be passed to the resulting subscription's `metadata` field.
    /// If `subscription_data.effective_date` is used, this field will be passed to the resulting subscription schedule's `phases.metadata` field.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}
impl CreateQuoteSubscriptionData {
    pub fn new() -> Self {
        Self { description: None, effective_date: None, metadata: None, trial_period_days: None }
    }
}
impl Default for CreateQuoteSubscriptionData {
    fn default() -> Self {
        Self::new()
    }
}
/// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
/// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
/// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
/// The `effective_date` is ignored if it is in the past when the quote is accepted.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateQuoteSubscriptionDataEffectiveDate {
    CurrentPeriodEnd,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// A quote models prices and services for a customer.
/// Default options for `header`, `description`, `footer`, and `expires_at` can be set in the dashboard via the [quote template](https://dashboard.stripe.com/settings/billing/quote).
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateQuote {
    inner: CreateQuoteBuilder,
}
impl CreateQuote {
    /// Construct a new `CreateQuote`.
    pub fn new() -> Self {
        Self { inner: CreateQuoteBuilder::new() }
    }
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    /// There cannot be any line items with recurring prices when using this field.
    pub fn application_fee_amount(mut self, application_fee_amount: impl Into<i64>) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount.into());
        self
    }
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    pub fn application_fee_percent(mut self, application_fee_percent: impl Into<f64>) -> Self {
        self.inner.application_fee_percent = Some(application_fee_percent.into());
        self
    }
    /// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
    pub fn automatic_tax(mut self, automatic_tax: impl Into<CreateQuoteAutomaticTax>) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or at invoice finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    pub fn collection_method(
        mut self,
        collection_method: impl Into<stripe_shared::QuoteCollectionMethod>,
    ) -> Self {
        self.inner.collection_method = Some(collection_method.into());
        self
    }
    /// The customer for which this quote belongs to.
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    pub fn default_tax_rates(mut self, default_tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.default_tax_rates = Some(default_tax_rates.into());
        self
    }
    /// A description that will be displayed on the quote PDF.
    /// If no value is passed, the default description configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The discounts applied to the quote. You can only set up to one discount.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A future timestamp on which the quote will be canceled if in `open` or `draft` status.
    /// Measured in seconds since the Unix epoch.
    /// If no value is passed, the default expiration date configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
    /// A footer that will be displayed on the quote PDF.
    /// If no value is passed, the default footer configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.inner.footer = Some(footer.into());
        self
    }
    /// Clone an existing quote.
    /// The new quote will be created in `status=draft`.
    /// When using this parameter, you cannot specify any other parameters except for `expires_at`.
    pub fn from_quote(mut self, from_quote: impl Into<CreateQuoteFromQuote>) -> Self {
        self.inner.from_quote = Some(from_quote.into());
        self
    }
    /// A header that will be displayed on the quote PDF.
    /// If no value is passed, the default header configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.inner.header = Some(header.into());
        self
    }
    /// All invoices will be billed using the specified settings.
    pub fn invoice_settings(
        mut self,
        invoice_settings: impl Into<CreateQuoteInvoiceSettings>,
    ) -> Self {
        self.inner.invoice_settings = Some(invoice_settings.into());
        self
    }
    /// A list of line items the customer is being quoted for.
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    pub fn line_items(mut self, line_items: impl Into<Vec<CreateQuoteLineItems>>) -> Self {
        self.inner.line_items = Some(line_items.into());
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
    /// The account on behalf of which to charge.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// When creating a subscription or subscription schedule, the specified configuration data will be used.
    /// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
    /// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
    pub fn subscription_data(
        mut self,
        subscription_data: impl Into<CreateQuoteSubscriptionData>,
    ) -> Self {
        self.inner.subscription_data = Some(subscription_data.into());
        self
    }
    /// ID of the test clock to attach to the quote.
    pub fn test_clock(mut self, test_clock: impl Into<String>) -> Self {
        self.inner.test_clock = Some(test_clock.into());
        self
    }
    /// The data with which to automatically create a Transfer for each of the invoices.
    pub fn transfer_data(mut self, transfer_data: impl Into<TransferDataSpecs>) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
}
impl Default for CreateQuote {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateQuote {
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

impl StripeRequest for CreateQuote {
    type Output = stripe_shared::Quote;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/quotes").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateQuoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    application_fee_percent: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_tax: Option<UpdateQuoteAutomaticTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    collection_method: Option<stripe_shared::QuoteCollectionMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_tax_rates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    discounts: Option<Vec<DiscountsDataParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    invoice_settings: Option<UpdateQuoteInvoiceSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_items: Option<Vec<UpdateQuoteLineItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    on_behalf_of: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subscription_data: Option<UpdateQuoteSubscriptionData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transfer_data: Option<TransferDataSpecs>,
}
impl UpdateQuoteBuilder {
    fn new() -> Self {
        Self {
            application_fee_amount: None,
            application_fee_percent: None,
            automatic_tax: None,
            collection_method: None,
            customer: None,
            default_tax_rates: None,
            description: None,
            discounts: None,
            expand: None,
            expires_at: None,
            footer: None,
            header: None,
            invoice_settings: None,
            line_items: None,
            metadata: None,
            on_behalf_of: None,
            subscription_data: None,
            transfer_data: None,
        }
    }
}
/// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteAutomaticTax {
    /// Controls whether Stripe will automatically compute tax on the resulting invoices or subscriptions as well as the quote itself.
    pub enabled: bool,
    /// The account that's liable for tax.
    /// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
    /// The tax transaction is returned in the report of the connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability: Option<UpdateQuoteAutomaticTaxLiability>,
}
impl UpdateQuoteAutomaticTax {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self { enabled: enabled.into(), liability: None }
    }
}
/// The account that's liable for tax.
/// If set, the business address and tax registrations required to perform the tax calculation are loaded from this account.
/// The tax transaction is returned in the report of the connected account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteAutomaticTaxLiability {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateQuoteAutomaticTaxLiabilityType,
}
impl UpdateQuoteAutomaticTaxLiability {
    pub fn new(type_: impl Into<UpdateQuoteAutomaticTaxLiabilityType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateQuoteAutomaticTaxLiabilityType {
    Account,
    Self_,
}
impl UpdateQuoteAutomaticTaxLiabilityType {
    pub fn as_str(self) -> &'static str {
        use UpdateQuoteAutomaticTaxLiabilityType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdateQuoteAutomaticTaxLiabilityType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateQuoteAutomaticTaxLiabilityType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateQuoteAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateQuoteAutomaticTaxLiabilityType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateQuoteAutomaticTaxLiabilityType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateQuoteAutomaticTaxLiabilityType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateQuoteAutomaticTaxLiabilityType")
        })
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteInvoiceSettings {
    /// Number of days within which a customer must pay the invoice generated by this quote.
    /// This value will be `null` for quotes where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
    /// The connected account that issues the invoice.
    /// The invoice is presented with the branding and support information of the specified account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuer: Option<UpdateQuoteInvoiceSettingsIssuer>,
}
impl UpdateQuoteInvoiceSettings {
    pub fn new() -> Self {
        Self { days_until_due: None, issuer: None }
    }
}
impl Default for UpdateQuoteInvoiceSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// The connected account that issues the invoice.
/// The invoice is presented with the branding and support information of the specified account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteInvoiceSettingsIssuer {
    /// The connected account being referenced when `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// Type of the account referenced in the request.
    #[serde(rename = "type")]
    pub type_: UpdateQuoteInvoiceSettingsIssuerType,
}
impl UpdateQuoteInvoiceSettingsIssuer {
    pub fn new(type_: impl Into<UpdateQuoteInvoiceSettingsIssuerType>) -> Self {
        Self { account: None, type_: type_.into() }
    }
}
/// Type of the account referenced in the request.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateQuoteInvoiceSettingsIssuerType {
    Account,
    Self_,
}
impl UpdateQuoteInvoiceSettingsIssuerType {
    pub fn as_str(self) -> &'static str {
        use UpdateQuoteInvoiceSettingsIssuerType::*;
        match self {
            Account => "account",
            Self_ => "self",
        }
    }
}

impl std::str::FromStr for UpdateQuoteInvoiceSettingsIssuerType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateQuoteInvoiceSettingsIssuerType::*;
        match s {
            "account" => Ok(Account),
            "self" => Ok(Self_),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateQuoteInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateQuoteInvoiceSettingsIssuerType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateQuoteInvoiceSettingsIssuerType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateQuoteInvoiceSettingsIssuerType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateQuoteInvoiceSettingsIssuerType")
        })
    }
}
/// A list of line items the customer is being quoted for.
/// Each line item includes information about the product, the quantity, and the resulting cost.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteLineItems {
    /// The discounts applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<DiscountsDataParam>>,
    /// The ID of an existing line item on the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The ID of the price object. One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateQuoteLineItemsPriceData>,
    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the line item.
    /// When set, the `default_tax_rates` on the quote do not apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}
impl UpdateQuoteLineItems {
    pub fn new() -> Self {
        Self {
            discounts: None,
            id: None,
            price: None,
            price_data: None,
            quantity: None,
            tax_rates: None,
        }
    }
}
impl Default for UpdateQuoteLineItems {
    fn default() -> Self {
        Self::new()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
/// One of `price` or `price_data` is required.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteLineItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The ID of the product that this price will belong to.
    pub product: String,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<UpdateQuoteLineItemsPriceDataRecurring>,
    /// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateQuoteLineItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}
impl UpdateQuoteLineItemsPriceData {
    pub fn new(currency: impl Into<stripe_types::Currency>, product: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            product: product.into(),
            recurring: None,
            tax_behavior: None,
            unit_amount: None,
            unit_amount_decimal: None,
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteLineItemsPriceDataRecurring {
    /// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
    pub interval: UpdateQuoteLineItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of three years interval allowed (3 years, 36 months, or 156 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpdateQuoteLineItemsPriceDataRecurring {
    pub fn new(interval: impl Into<UpdateQuoteLineItemsPriceDataRecurringInterval>) -> Self {
        Self { interval: interval.into(), interval_count: None }
    }
}
/// Specifies billing frequency. Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateQuoteLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}
impl UpdateQuoteLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateQuoteLineItemsPriceDataRecurringInterval::*;
        match self {
            Day => "day",
            Month => "month",
            Week => "week",
            Year => "year",
        }
    }
}

impl std::str::FromStr for UpdateQuoteLineItemsPriceDataRecurringInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateQuoteLineItemsPriceDataRecurringInterval::*;
        match s {
            "day" => Ok(Day),
            "month" => Ok(Month),
            "week" => Ok(Week),
            "year" => Ok(Year),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateQuoteLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateQuoteLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateQuoteLineItemsPriceDataRecurringInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateQuoteLineItemsPriceDataRecurringInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateQuoteLineItemsPriceDataRecurringInterval",
            )
        })
    }
}
/// Only required if a [default tax behavior](https://stripe.com/docs/tax/products-prices-tax-categories-tax-behavior#setting-a-default-tax-behavior-(recommended)) was not provided in the Stripe Tax settings.
/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
/// One of `inclusive`, `exclusive`, or `unspecified`.
/// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateQuoteLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}
impl UpdateQuoteLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        use UpdateQuoteLineItemsPriceDataTaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateQuoteLineItemsPriceDataTaxBehavior {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateQuoteLineItemsPriceDataTaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateQuoteLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateQuoteLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateQuoteLineItemsPriceDataTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateQuoteLineItemsPriceDataTaxBehavior {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateQuoteLineItemsPriceDataTaxBehavior")
        })
    }
}
/// When creating a subscription or subscription schedule, the specified configuration data will be used.
/// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
/// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    /// Use this field to optionally store an explanation of the subscription for rendering in Stripe surfaces and certain local payment methods UIs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    /// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
    /// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
    /// The `effective_date` is ignored if it is in the past when the quote is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<UpdateQuoteSubscriptionDataEffectiveDate>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that will set metadata on the subscription or subscription schedule when the quote is accepted.
    /// If a recurring price is included in `line_items`, this field will be passed to the resulting subscription's `metadata` field.
    /// If `subscription_data.effective_date` is used, this field will be passed to the resulting subscription schedule's `phases.metadata` field.
    /// Unlike object-level metadata, this field is declarative.
    /// Updates will clear prior values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}
impl UpdateQuoteSubscriptionData {
    pub fn new() -> Self {
        Self { description: None, effective_date: None, metadata: None, trial_period_days: None }
    }
}
impl Default for UpdateQuoteSubscriptionData {
    fn default() -> Self {
        Self::new()
    }
}
/// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
/// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
/// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
/// The `effective_date` is ignored if it is in the past when the quote is accepted.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateQuoteSubscriptionDataEffectiveDate {
    CurrentPeriodEnd,
    #[serde(untagged)]
    Timestamp(stripe_types::Timestamp),
}
/// A quote models prices and services for a customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateQuote {
    inner: UpdateQuoteBuilder,
    quote: stripe_shared::QuoteId,
}
impl UpdateQuote {
    /// Construct a new `UpdateQuote`.
    pub fn new(quote: impl Into<stripe_shared::QuoteId>) -> Self {
        Self { quote: quote.into(), inner: UpdateQuoteBuilder::new() }
    }
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    /// There cannot be any line items with recurring prices when using this field.
    pub fn application_fee_amount(mut self, application_fee_amount: impl Into<i64>) -> Self {
        self.inner.application_fee_amount = Some(application_fee_amount.into());
        self
    }
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    pub fn application_fee_percent(mut self, application_fee_percent: impl Into<f64>) -> Self {
        self.inner.application_fee_percent = Some(application_fee_percent.into());
        self
    }
    /// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
    pub fn automatic_tax(mut self, automatic_tax: impl Into<UpdateQuoteAutomaticTax>) -> Self {
        self.inner.automatic_tax = Some(automatic_tax.into());
        self
    }
    /// Either `charge_automatically`, or `send_invoice`.
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or at invoice finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    pub fn collection_method(
        mut self,
        collection_method: impl Into<stripe_shared::QuoteCollectionMethod>,
    ) -> Self {
        self.inner.collection_method = Some(collection_method.into());
        self
    }
    /// The customer for which this quote belongs to.
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    pub fn default_tax_rates(mut self, default_tax_rates: impl Into<Vec<String>>) -> Self {
        self.inner.default_tax_rates = Some(default_tax_rates.into());
        self
    }
    /// A description that will be displayed on the quote PDF.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.inner.description = Some(description.into());
        self
    }
    /// The discounts applied to the quote. You can only set up to one discount.
    pub fn discounts(mut self, discounts: impl Into<Vec<DiscountsDataParam>>) -> Self {
        self.inner.discounts = Some(discounts.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A future timestamp on which the quote will be canceled if in `open` or `draft` status.
    /// Measured in seconds since the Unix epoch.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
    /// A footer that will be displayed on the quote PDF.
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.inner.footer = Some(footer.into());
        self
    }
    /// A header that will be displayed on the quote PDF.
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.inner.header = Some(header.into());
        self
    }
    /// All invoices will be billed using the specified settings.
    pub fn invoice_settings(
        mut self,
        invoice_settings: impl Into<UpdateQuoteInvoiceSettings>,
    ) -> Self {
        self.inner.invoice_settings = Some(invoice_settings.into());
        self
    }
    /// A list of line items the customer is being quoted for.
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    pub fn line_items(mut self, line_items: impl Into<Vec<UpdateQuoteLineItems>>) -> Self {
        self.inner.line_items = Some(line_items.into());
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
    /// The account on behalf of which to charge.
    pub fn on_behalf_of(mut self, on_behalf_of: impl Into<String>) -> Self {
        self.inner.on_behalf_of = Some(on_behalf_of.into());
        self
    }
    /// When creating a subscription or subscription schedule, the specified configuration data will be used.
    /// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
    /// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
    pub fn subscription_data(
        mut self,
        subscription_data: impl Into<UpdateQuoteSubscriptionData>,
    ) -> Self {
        self.inner.subscription_data = Some(subscription_data.into());
        self
    }
    /// The data with which to automatically create a Transfer for each of the invoices.
    pub fn transfer_data(mut self, transfer_data: impl Into<TransferDataSpecs>) -> Self {
        self.inner.transfer_data = Some(transfer_data.into());
        self
    }
}
impl UpdateQuote {
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

impl StripeRequest for UpdateQuote {
    type Output = stripe_shared::Quote;

    fn build(&self) -> RequestBuilder {
        let quote = &self.quote;
        RequestBuilder::new(StripeMethod::Post, format!("/quotes/{quote}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct AcceptQuoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl AcceptQuoteBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Accepts the specified quote.
#[derive(Clone, Debug, serde::Serialize)]
pub struct AcceptQuote {
    inner: AcceptQuoteBuilder,
    quote: stripe_shared::QuoteId,
}
impl AcceptQuote {
    /// Construct a new `AcceptQuote`.
    pub fn new(quote: impl Into<stripe_shared::QuoteId>) -> Self {
        Self { quote: quote.into(), inner: AcceptQuoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl AcceptQuote {
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

impl StripeRequest for AcceptQuote {
    type Output = stripe_shared::Quote;

    fn build(&self) -> RequestBuilder {
        let quote = &self.quote;
        RequestBuilder::new(StripeMethod::Post, format!("/quotes/{quote}/accept")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CancelQuoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CancelQuoteBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Cancels the quote.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CancelQuote {
    inner: CancelQuoteBuilder,
    quote: stripe_shared::QuoteId,
}
impl CancelQuote {
    /// Construct a new `CancelQuote`.
    pub fn new(quote: impl Into<stripe_shared::QuoteId>) -> Self {
        Self { quote: quote.into(), inner: CancelQuoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CancelQuote {
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

impl StripeRequest for CancelQuote {
    type Output = stripe_shared::Quote;

    fn build(&self) -> RequestBuilder {
        let quote = &self.quote;
        RequestBuilder::new(StripeMethod::Post, format!("/quotes/{quote}/cancel")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct FinalizeQuoteQuoteBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expires_at: Option<stripe_types::Timestamp>,
}
impl FinalizeQuoteQuoteBuilder {
    fn new() -> Self {
        Self { expand: None, expires_at: None }
    }
}
/// Finalizes the quote.
#[derive(Clone, Debug, serde::Serialize)]
pub struct FinalizeQuoteQuote {
    inner: FinalizeQuoteQuoteBuilder,
    quote: stripe_shared::QuoteId,
}
impl FinalizeQuoteQuote {
    /// Construct a new `FinalizeQuoteQuote`.
    pub fn new(quote: impl Into<stripe_shared::QuoteId>) -> Self {
        Self { quote: quote.into(), inner: FinalizeQuoteQuoteBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A future timestamp on which the quote will be canceled if in `open` or `draft` status.
    /// Measured in seconds since the Unix epoch.
    pub fn expires_at(mut self, expires_at: impl Into<stripe_types::Timestamp>) -> Self {
        self.inner.expires_at = Some(expires_at.into());
        self
    }
}
impl FinalizeQuoteQuote {
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

impl StripeRequest for FinalizeQuoteQuote {
    type Output = stripe_shared::Quote;

    fn build(&self) -> RequestBuilder {
        let quote = &self.quote;
        RequestBuilder::new(StripeMethod::Post, format!("/quotes/{quote}/finalize"))
            .form(&self.inner)
    }
}

#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct TransferDataSpecs {
    /// The amount that will be transferred automatically when the invoice is paid.
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    /// This represents the percentage of the subscription invoice total that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: String,
}
impl TransferDataSpecs {
    pub fn new(destination: impl Into<String>) -> Self {
        Self { amount: None, amount_percent: None, destination: destination.into() }
    }
}
