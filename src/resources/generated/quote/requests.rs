use crate::{Client, Response};

impl crate::quote::Quote {
    /// Retrieves the quote with the given ID.
    pub fn retrieve(
        client: &Client,
        quote: &crate::quote::QuoteId,
        params: RetrieveQuote,
    ) -> Response<crate::quote::Quote> {
        client.get_query(&format!("/quotes/{quote}", quote = quote), params)
    }
    /// A quote models prices and services for a customer.
    ///
    /// Default options for `header`, `description`, `footer`, and `expires_at` can be set in the dashboard via the [quote template](https://dashboard.stripe.com/settings/billing/quote).
    pub fn create(client: &Client, params: CreateQuote) -> Response<crate::quote::Quote> {
        client.send_form("/quotes", params, http_types::Method::Post)
    }
    /// A quote models prices and services for a customer.
    pub fn update(
        client: &Client,
        quote: &crate::quote::QuoteId,
        params: UpdateQuote,
    ) -> Response<crate::quote::Quote> {
        client.send_form(
            &format!("/quotes/{quote}", quote = quote),
            params,
            http_types::Method::Post,
        )
    }
    /// Cancels the quote.
    pub fn cancel(
        client: &Client,
        quote: &crate::quote::QuoteId,
        params: CancelQuote,
    ) -> Response<crate::quote::Quote> {
        client.send_form(
            &format!("/quotes/{quote}/cancel", quote = quote),
            params,
            http_types::Method::Post,
        )
    }
    /// Finalizes the quote.
    pub fn finalize_quote(
        client: &Client,
        quote: &crate::quote::QuoteId,
        params: FinalizeQuoteQuote,
    ) -> Response<crate::quote::Quote> {
        client.send_form(
            &format!("/quotes/{quote}/finalize", quote = quote),
            params,
            http_types::Method::Post,
        )
    }
    /// Accepts the specified quote.
    pub fn accept(
        client: &Client,
        quote: &crate::quote::QuoteId,
        params: AcceptQuote,
    ) -> Response<crate::quote::Quote> {
        client.send_form(
            &format!("/quotes/{quote}/accept", quote = quote),
            params,
            http_types::Method::Post,
        )
    }
    /// Returns a list of your quotes.
    pub fn list(client: &Client, params: ListQuote) -> Response<crate::List<crate::quote::Quote>> {
        client.get_query("/quotes", params)
    }
    /// When retrieving a quote, there is an includable **line_items** property containing the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of line items.
    pub fn list_line_items(
        client: &Client,
        quote: &crate::quote::QuoteId,
        params: ListLineItemsQuote,
    ) -> Response<crate::List<crate::line_item::LineItem>> {
        client.get_query(&format!("/quotes/{quote}/line_items", quote = quote), params)
    }
    /// When retrieving a quote, there is an includable <a href="https://stripe.com/docs/api/quotes/object#quote_object-computed-upfront-line_items">**computed.upfront.line_items**</a> property containing the first handful of those items.
    ///
    /// There is also a URL where you can retrieve the full (paginated) list of upfront line items.
    pub fn list_computed_upfront_line_items(
        client: &Client,
        quote: &crate::quote::QuoteId,
        params: ListComputedUpfrontLineItemsQuote,
    ) -> Response<crate::List<crate::line_item::LineItem>> {
        client.get_query(
            &format!("/quotes/{quote}/computed_upfront_line_items", quote = quote),
            params,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveQuote<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveQuote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateQuote<'a> {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<CreateQuoteAutomaticTax>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or at invoice finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CreateQuoteCollectionMethod>,
    /// The customer for which this quote belongs to.
    ///
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// A description that will be displayed on the quote PDF.
    ///
    /// If no value is passed, the default description configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The discounts applied to the quote.
    ///
    /// You can only set up to one discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [CreateQuoteDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A future timestamp on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    /// If no value is passed, the default expiration date configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    /// A footer that will be displayed on the quote PDF.
    ///
    /// If no value is passed, the default footer configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// Clone an existing quote.
    ///
    /// The new quote will be created in `status=draft`.
    /// When using this parameter, you cannot specify any other parameters except for `expires_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_quote: Option<CreateQuoteFromQuote<'a>>,
    /// A header that will be displayed on the quote PDF.
    ///
    /// If no value is passed, the default header configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<&'a str>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateQuoteInvoiceSettings>,
    /// A list of line items the customer is being quoted for.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<&'a [CreateQuoteLineItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// The account on behalf of which to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// When creating a subscription or subscription schedule, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
    /// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<CreateQuoteSubscriptionData<'a>>,
    /// ID of the test clock to attach to the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,
    /// The data with which to automatically create a Transfer for each of the invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<CreateQuoteTransferData<'a>>,
}
impl<'a> CreateQuote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateQuoteAutomaticTax {
    /// Controls whether Stripe will automatically compute tax on the resulting invoices or subscriptions as well as the quote itself.
    pub enabled: bool,
}
impl CreateQuoteAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or at invoice finalization using the default payment method attached to the subscription or customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateQuoteCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl CreateQuoteCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for CreateQuoteCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateQuoteCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The discounts applied to the quote.
///
/// You can only set up to one discount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateQuoteDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> CreateQuoteDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Clone an existing quote.
///
/// The new quote will be created in `status=draft`.
/// When using this parameter, you cannot specify any other parameters except for `expires_at`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateQuoteFromQuote<'a> {
    /// Whether this quote is a revision of the previous quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revision: Option<bool>,
    /// The `id` of the quote that will be cloned.
    pub quote: &'a str,
}
impl<'a> CreateQuoteFromQuote<'a> {
    pub fn new(quote: &'a str) -> Self {
        Self { is_revision: Default::default(), quote }
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateQuoteInvoiceSettings {
    /// Number of days within which a customer must pay the invoice generated by this quote.
    ///
    /// This value will be `null` for quotes where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}
impl CreateQuoteInvoiceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A list of line items the customer is being quoted for.
///
/// Each line item includes information about the product, the quantity, and the resulting cost.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateQuoteLineItems<'a> {
    /// The ID of the price object.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<CreateQuoteLineItemsPriceData<'a>>,
    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the line item.
    ///
    /// When set, the `default_tax_rates` on the quote do not apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> CreateQuoteLineItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
///
/// One of `price` or `price_data` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateQuoteLineItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<CreateQuoteLineItemsPriceDataRecurring>,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateQuoteLineItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> CreateQuoteLineItemsPriceData<'a> {
    pub fn new(currency: crate::Currency, product: &'a str) -> Self {
        Self {
            currency,
            product,
            recurring: Default::default(),
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateQuoteLineItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: CreateQuoteLineItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl CreateQuoteLineItemsPriceDataRecurring {
    pub fn new(interval: CreateQuoteLineItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateQuoteLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl CreateQuoteLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for CreateQuoteLineItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateQuoteLineItemsPriceDataRecurringInterval {
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
pub enum CreateQuoteLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateQuoteLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for CreateQuoteLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateQuoteLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// When creating a subscription or subscription schedule, the specified configuration data will be used.
///
/// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
/// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateQuoteSubscriptionData<'a> {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    ///
    /// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
    /// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
    /// The `effective_date` is ignored if it is in the past when the quote is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<CreateQuoteSubscriptionDataEffectiveDate>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}
impl<'a> CreateQuoteSubscriptionData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
///
/// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
/// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
/// The `effective_date` is ignored if it is in the past when the quote is accepted.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum CreateQuoteSubscriptionDataEffectiveDate {
    CurrentPeriodEnd,
    Timestamp(crate::Timestamp),
}
/// The data with which to automatically create a Transfer for each of the invoices.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateQuoteTransferData<'a> {
    /// The amount that will be transferred automatically when the invoice is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> CreateQuoteTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), amount_percent: Default::default(), destination }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateQuote<'a> {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    /// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<UpdateQuoteAutomaticTax>,
    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or at invoice finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<UpdateQuoteCollectionMethod>,
    /// The customer for which this quote belongs to.
    ///
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<&'a str>,
    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<&'a [&'a str]>,
    /// A description that will be displayed on the quote PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// The discounts applied to the quote.
    ///
    /// You can only set up to one discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<&'a [UpdateQuoteDiscounts<'a>]>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A future timestamp on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
    /// A footer that will be displayed on the quote PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<&'a str>,
    /// A header that will be displayed on the quote PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<&'a str>,
    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdateQuoteInvoiceSettings>,
    /// A list of line items the customer is being quoted for.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<&'a [UpdateQuoteLineItems<'a>]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a crate::Metadata>,
    /// The account on behalf of which to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<&'a str>,
    /// When creating a subscription or subscription schedule, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
    /// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<UpdateQuoteSubscriptionData<'a>>,
    /// The data with which to automatically create a Transfer for each of the invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<UpdateQuoteTransferData<'a>>,
}
impl<'a> UpdateQuote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteAutomaticTax {
    /// Controls whether Stripe will automatically compute tax on the resulting invoices or subscriptions as well as the quote itself.
    pub enabled: bool,
}
impl UpdateQuoteAutomaticTax {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}
/// Either `charge_automatically`, or `send_invoice`.
///
/// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or at invoice finalization using the default payment method attached to the subscription or customer.
/// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
/// Defaults to `charge_automatically`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateQuoteCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl UpdateQuoteCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for UpdateQuoteCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateQuoteCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// The discounts applied to the quote.
///
/// You can only set up to one discount.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateQuoteDiscounts<'a> {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<&'a str>,
    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<&'a str>,
}
impl<'a> UpdateQuoteDiscounts<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateQuoteInvoiceSettings {
    /// Number of days within which a customer must pay the invoice generated by this quote.
    ///
    /// This value will be `null` for quotes where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}
impl UpdateQuoteInvoiceSettings {
    pub fn new() -> Self {
        Self::default()
    }
}
/// A list of line items the customer is being quoted for.
///
/// Each line item includes information about the product, the quantity, and the resulting cost.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateQuoteLineItems<'a> {
    /// The ID of an existing line item on the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<&'a str>,
    /// The ID of the price object.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<&'a str>,
    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<UpdateQuoteLineItemsPriceData<'a>>,
    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,
    /// The tax rates which apply to the line item.
    ///
    /// When set, the `default_tax_rates` on the quote do not apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<&'a [&'a str]>,
}
impl<'a> UpdateQuoteLineItems<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
///
/// One of `price` or `price_data` is required.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteLineItemsPriceData<'a> {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::Currency,
    /// The ID of the product that this price will belong to.
    pub product: &'a str,
    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<UpdateQuoteLineItemsPriceDataRecurring>,
    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateQuoteLineItemsPriceDataTaxBehavior>,
    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<&'a str>,
}
impl<'a> UpdateQuoteLineItemsPriceData<'a> {
    pub fn new(currency: crate::Currency, product: &'a str) -> Self {
        Self {
            currency,
            product,
            recurring: Default::default(),
            tax_behavior: Default::default(),
            unit_amount: Default::default(),
            unit_amount_decimal: Default::default(),
        }
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteLineItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: UpdateQuoteLineItemsPriceDataRecurringInterval,
    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}
impl UpdateQuoteLineItemsPriceDataRecurring {
    pub fn new(interval: UpdateQuoteLineItemsPriceDataRecurringInterval) -> Self {
        Self { interval, interval_count: Default::default() }
    }
}
/// Specifies billing frequency.
///
/// Either `day`, `week`, `month` or `year`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateQuoteLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl UpdateQuoteLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for UpdateQuoteLineItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateQuoteLineItemsPriceDataRecurringInterval {
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
pub enum UpdateQuoteLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateQuoteLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for UpdateQuoteLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateQuoteLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
/// When creating a subscription or subscription schedule, the specified configuration data will be used.
///
/// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
/// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateQuoteSubscriptionData<'a> {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'a str>,
    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    ///
    /// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
    /// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
    /// The `effective_date` is ignored if it is in the past when the quote is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<UpdateQuoteSubscriptionDataEffectiveDate>,
    /// Integer representing the number of trial period days before the customer is charged for the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}
impl<'a> UpdateQuoteSubscriptionData<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
///
/// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
/// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
/// The `effective_date` is ignored if it is in the past when the quote is accepted.
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum UpdateQuoteSubscriptionDataEffectiveDate {
    CurrentPeriodEnd,
    Timestamp(crate::Timestamp),
}
/// The data with which to automatically create a Transfer for each of the invoices.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateQuoteTransferData<'a> {
    /// The amount that will be transferred automatically when the invoice is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,
    /// ID of an existing, connected Stripe account.
    pub destination: &'a str,
}
impl<'a> UpdateQuoteTransferData<'a> {
    pub fn new(destination: &'a str) -> Self {
        Self { amount: Default::default(), amount_percent: Default::default(), destination }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CancelQuote<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CancelQuote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct FinalizeQuoteQuote<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A future timestamp on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::Timestamp>,
}
impl<'a> FinalizeQuoteQuote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct AcceptQuote<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> AcceptQuote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListQuote<'a> {
    /// The ID of the customer whose quotes will be retrieved.
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
    pub starting_after: Option<String>,
    /// The status of the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListQuoteStatus>,
    /// Provides a list of quotes that are associated with the specified test clock.
    ///
    /// The response will not include quotes with test clocks if this and the customer parameter is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<&'a str>,
}
impl<'a> ListQuote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The status of the quote.
#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListQuoteStatus {
    Accepted,
    Canceled,
    Draft,
    Open,
}

impl ListQuoteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Canceled => "canceled",
            Self::Draft => "draft",
            Self::Open => "open",
        }
    }
}

impl AsRef<str> for ListQuoteStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListQuoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListLineItemsQuote<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
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
    pub starting_after: Option<String>,
}
impl<'a> ListLineItemsQuote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListComputedUpfrontLineItemsQuote<'a> {
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
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
    pub starting_after: Option<String>,
}
impl<'a> ListComputedUpfrontLineItemsQuote<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
