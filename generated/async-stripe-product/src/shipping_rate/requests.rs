use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListShippingRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListShippingRateBuilder {
    fn new() -> Self {
        Self {
            active: None,
            created: None,
            currency: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of your shipping rates.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListShippingRate {
    inner: ListShippingRateBuilder,
}
impl ListShippingRate {
    /// Construct a new `ListShippingRate`.
    pub fn new() -> Self {
        Self { inner: ListShippingRateBuilder::new() }
    }
    /// Only return shipping rates that are active or inactive.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// A filter on the list, based on the object `created` field.
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return shipping rates for the given currency.
    pub fn currency(mut self, currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.currency = Some(currency.into());
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
}
impl Default for ListShippingRate {
    fn default() -> Self {
        Self::new()
    }
}
impl ListShippingRate {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::ShippingRate>> {
        stripe_client_core::ListPaginator::new_list("/shipping_rates", &self.inner)
    }
}

impl StripeRequest for ListShippingRate {
    type Output = stripe_types::List<stripe_shared::ShippingRate>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/shipping_rates").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveShippingRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveShippingRateBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Returns the shipping rate object with the given ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveShippingRate {
    inner: RetrieveShippingRateBuilder,
    shipping_rate_token: stripe_shared::ShippingRateId,
}
impl RetrieveShippingRate {
    /// Construct a new `RetrieveShippingRate`.
    pub fn new(shipping_rate_token: impl Into<stripe_shared::ShippingRateId>) -> Self {
        Self {
            shipping_rate_token: shipping_rate_token.into(),
            inner: RetrieveShippingRateBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveShippingRate {
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

impl StripeRequest for RetrieveShippingRate {
    type Output = stripe_shared::ShippingRate;

    fn build(&self) -> RequestBuilder {
        let shipping_rate_token = &self.shipping_rate_token;
        RequestBuilder::new(StripeMethod::Get, format!("/shipping_rates/{shipping_rate_token}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateShippingRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    delivery_estimate: Option<CreateShippingRateDeliveryEstimate>,
    display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_amount: Option<CreateShippingRateFixedAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<stripe_shared::ShippingRateTaxBehavior>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_code: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::ShippingRateType>,
}
impl CreateShippingRateBuilder {
    fn new(display_name: impl Into<String>) -> Self {
        Self {
            delivery_estimate: None,
            display_name: display_name.into(),
            expand: None,
            fixed_amount: None,
            metadata: None,
            tax_behavior: None,
            tax_code: None,
            type_: None,
        }
    }
}
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateShippingRateDeliveryEstimate {
    /// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<CreateShippingRateDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range. If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<CreateShippingRateDeliveryEstimateMinimum>,
}
impl CreateShippingRateDeliveryEstimate {
    pub fn new() -> Self {
        Self { maximum: None, minimum: None }
    }
}
impl Default for CreateShippingRateDeliveryEstimate {
    fn default() -> Self {
        Self::new()
    }
}
/// The upper bound of the estimated range. If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateShippingRateDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateShippingRateDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateShippingRateDeliveryEstimateMaximum {
    pub fn new(
        unit: impl Into<CreateShippingRateDeliveryEstimateMaximumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateShippingRateDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl CreateShippingRateDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        use CreateShippingRateDeliveryEstimateMaximumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for CreateShippingRateDeliveryEstimateMaximumUnit {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateShippingRateDeliveryEstimateMaximumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateShippingRateDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateShippingRateDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateShippingRateDeliveryEstimateMaximumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateShippingRateDeliveryEstimateMaximumUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateShippingRateDeliveryEstimateMaximumUnit",
            )
        })
    }
}
/// The lower bound of the estimated range. If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateShippingRateDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateShippingRateDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateShippingRateDeliveryEstimateMinimum {
    pub fn new(
        unit: impl Into<CreateShippingRateDeliveryEstimateMinimumUnit>,
        value: impl Into<i64>,
    ) -> Self {
        Self { unit: unit.into(), value: value.into() }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateShippingRateDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}
impl CreateShippingRateDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        use CreateShippingRateDeliveryEstimateMinimumUnit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for CreateShippingRateDeliveryEstimateMinimumUnit {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateShippingRateDeliveryEstimateMinimumUnit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateShippingRateDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateShippingRateDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateShippingRateDeliveryEstimateMinimumUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateShippingRateDeliveryEstimateMinimumUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateShippingRateDeliveryEstimateMinimumUnit",
            )
        })
    }
}
/// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateShippingRateFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            CreateShippingRateFixedAmountCurrencyOptions,
        >,
    >,
}
impl CreateShippingRateFixedAmount {
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { amount: amount.into(), currency: currency.into(), currency_options: None }
    }
}
/// Shipping rates defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateShippingRateFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::ShippingRateTaxBehavior>,
}
impl CreateShippingRateFixedAmountCurrencyOptions {
    pub fn new(amount: impl Into<i64>) -> Self {
        Self { amount: amount.into(), tax_behavior: None }
    }
}
/// Creates a new shipping rate object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateShippingRate {
    inner: CreateShippingRateBuilder,
}
impl CreateShippingRate {
    /// Construct a new `CreateShippingRate`.
    pub fn new(display_name: impl Into<String>) -> Self {
        Self { inner: CreateShippingRateBuilder::new(display_name.into()) }
    }
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    /// This will appear on CheckoutSessions.
    pub fn delivery_estimate(
        mut self,
        delivery_estimate: impl Into<CreateShippingRateDeliveryEstimate>,
    ) -> Self {
        self.inner.delivery_estimate = Some(delivery_estimate.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
    pub fn fixed_amount(mut self, fixed_amount: impl Into<CreateShippingRateFixedAmount>) -> Self {
        self.inner.fixed_amount = Some(fixed_amount.into());
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
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub fn tax_behavior(
        mut self,
        tax_behavior: impl Into<stripe_shared::ShippingRateTaxBehavior>,
    ) -> Self {
        self.inner.tax_behavior = Some(tax_behavior.into());
        self
    }
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    /// The Shipping tax code is `txcd_92010001`.
    pub fn tax_code(mut self, tax_code: impl Into<String>) -> Self {
        self.inner.tax_code = Some(tax_code.into());
        self
    }
    /// The type of calculation to use on the shipping rate.
    pub fn type_(mut self, type_: impl Into<stripe_shared::ShippingRateType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl CreateShippingRate {
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

impl StripeRequest for CreateShippingRate {
    type Output = stripe_shared::ShippingRate;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/shipping_rates").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateShippingRateBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fixed_amount: Option<UpdateShippingRateFixedAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_behavior: Option<stripe_shared::ShippingRateTaxBehavior>,
}
impl UpdateShippingRateBuilder {
    fn new() -> Self {
        Self { active: None, expand: None, fixed_amount: None, metadata: None, tax_behavior: None }
    }
}
/// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateShippingRateFixedAmount {
    /// Shipping rates defined in each available currency option.
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        std::collections::HashMap<
            stripe_types::Currency,
            UpdateShippingRateFixedAmountCurrencyOptions,
        >,
    >,
}
impl UpdateShippingRateFixedAmount {
    pub fn new() -> Self {
        Self { currency_options: None }
    }
}
impl Default for UpdateShippingRateFixedAmount {
    fn default() -> Self {
        Self::new()
    }
}
/// Shipping rates defined in each available currency option.
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateShippingRateFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<stripe_shared::ShippingRateTaxBehavior>,
}
impl UpdateShippingRateFixedAmountCurrencyOptions {
    pub fn new() -> Self {
        Self { amount: None, tax_behavior: None }
    }
}
impl Default for UpdateShippingRateFixedAmountCurrencyOptions {
    fn default() -> Self {
        Self::new()
    }
}
/// Updates an existing shipping rate object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateShippingRate {
    inner: UpdateShippingRateBuilder,
    shipping_rate_token: stripe_shared::ShippingRateId,
}
impl UpdateShippingRate {
    /// Construct a new `UpdateShippingRate`.
    pub fn new(shipping_rate_token: impl Into<stripe_shared::ShippingRateId>) -> Self {
        Self {
            shipping_rate_token: shipping_rate_token.into(),
            inner: UpdateShippingRateBuilder::new(),
        }
    }
    /// Whether the shipping rate can be used for new purchases. Defaults to `true`.
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.inner.active = Some(active.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Describes a fixed amount to charge for shipping. Must be present if type is `fixed_amount`.
    pub fn fixed_amount(mut self, fixed_amount: impl Into<UpdateShippingRateFixedAmount>) -> Self {
        self.inner.fixed_amount = Some(fixed_amount.into());
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
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub fn tax_behavior(
        mut self,
        tax_behavior: impl Into<stripe_shared::ShippingRateTaxBehavior>,
    ) -> Self {
        self.inner.tax_behavior = Some(tax_behavior.into());
        self
    }
}
impl UpdateShippingRate {
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

impl StripeRequest for UpdateShippingRate {
    type Output = stripe_shared::ShippingRate;

    fn build(&self) -> RequestBuilder {
        let shipping_rate_token = &self.shipping_rate_token;
        RequestBuilder::new(StripeMethod::Post, format!("/shipping_rates/{shipping_rate_token}"))
            .form(&self.inner)
    }
}
