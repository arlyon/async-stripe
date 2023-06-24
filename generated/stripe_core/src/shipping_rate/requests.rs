use stripe::{Client, Response};

impl stripe_core::shipping_rate::ShippingRate {
    /// Returns a list of your shipping rates.
    pub fn list(
        client: &Client,
        params: ListShippingRate,
    ) -> Response<stripe_types::List<stripe_core::shipping_rate::ShippingRate>> {
        client.get_query("/shipping_rates", params)
    }
    /// Returns the shipping rate object with the given ID.
    pub fn retrieve(
        client: &Client,
        shipping_rate_token: &str,
        params: RetrieveShippingRate,
    ) -> Response<stripe_core::shipping_rate::ShippingRate> {
        client.get_query(
            &format!(
                "/shipping_rates/{shipping_rate_token}",
                shipping_rate_token = shipping_rate_token
            ),
            params,
        )
    }
    /// Creates a new shipping rate object.
    pub fn create(
        client: &Client,
        params: CreateShippingRate,
    ) -> Response<stripe_core::shipping_rate::ShippingRate> {
        client.send_form("/shipping_rates", params, http_types::Method::Post)
    }
    /// Updates an existing shipping rate object.
    pub fn update(
        client: &Client,
        shipping_rate_token: &str,
        params: UpdateShippingRate,
    ) -> Response<stripe_core::shipping_rate::ShippingRate> {
        client.send_form(
            &format!(
                "/shipping_rates/{shipping_rate_token}",
                shipping_rate_token = shipping_rate_token
            ),
            params,
            http_types::Method::Post,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListShippingRate<'a> {
    /// Only return shipping rates that are active or inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// Only return shipping rates for the given currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<stripe_types::Currency>,
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
impl<'a> ListShippingRate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveShippingRate<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveShippingRate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateShippingRate<'a> {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<CreateShippingRateDeliveryEstimate>,
    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateShippingRateFixedAmount<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateShippingRateTaxBehavior>,
    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<&'a str>,
    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateShippingRateType>,
}
impl<'a> CreateShippingRate<'a> {
    pub fn new(display_name: &'a str) -> Self {
        Self {
            delivery_estimate: Default::default(),
            display_name,
            expand: Default::default(),
            fixed_amount: Default::default(),
            metadata: Default::default(),
            tax_behavior: Default::default(),
            tax_code: Default::default(),
            type_: Default::default(),
        }
    }
}
/// The estimated range for how long shipping will take, meant to be displayable to the customer.
///
/// This will appear on CheckoutSessions.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct CreateShippingRateDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<CreateShippingRateDeliveryEstimateMaximum>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<CreateShippingRateDeliveryEstimateMinimum>,
}
impl CreateShippingRateDeliveryEstimate {
    pub fn new() -> Self {
        Self::default()
    }
}
/// The upper bound of the estimated range.
///
/// If empty, represents no upper bound i.e., infinite.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateShippingRateDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateShippingRateDeliveryEstimateMaximumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateShippingRateDeliveryEstimateMaximum {
    pub fn new(unit: CreateShippingRateDeliveryEstimateMaximumUnit, value: i64) -> Self {
        Self { unit, value }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateShippingRateDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateShippingRateDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl std::str::FromStr for CreateShippingRateDeliveryEstimateMaximumUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business_day" => Ok(Self::BusinessDay),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateShippingRateDeliveryEstimateMaximumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateShippingRateDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// The lower bound of the estimated range.
///
/// If empty, represents no lower bound.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateShippingRateDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateShippingRateDeliveryEstimateMinimumUnit,
    /// Must be greater than 0.
    pub value: i64,
}
impl CreateShippingRateDeliveryEstimateMinimum {
    pub fn new(unit: CreateShippingRateDeliveryEstimateMinimumUnit, value: i64) -> Self {
        Self { unit, value }
    }
}
/// A unit of time.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateShippingRateDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl CreateShippingRateDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl std::str::FromStr for CreateShippingRateDeliveryEstimateMinimumUnit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "business_day" => Ok(Self::BusinessDay),
            "day" => Ok(Self::Day),
            "hour" => Ok(Self::Hour),
            "month" => Ok(Self::Month),
            "week" => Ok(Self::Week),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateShippingRateDeliveryEstimateMinimumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateShippingRateDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
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
/// Describes a fixed amount to charge for shipping.
///
/// Must be present if type is `fixed_amount`.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateShippingRateFixedAmount<'a> {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        &'a std::collections::HashMap<
            stripe_types::Currency,
            CreateShippingRateFixedAmountCurrencyOptions,
        >,
    >,
}
impl<'a> CreateShippingRateFixedAmount<'a> {
    pub fn new(amount: i64, currency: stripe_types::Currency) -> Self {
        Self { amount, currency, currency_options: Default::default() }
    }
}
/// Shipping rates defined in each available currency option.
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateShippingRateFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior>,
}
impl CreateShippingRateFixedAmountCurrencyOptions {
    pub fn new(amount: i64) -> Self {
        Self { amount, tax_behavior: Default::default() }
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateShippingRateTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateShippingRateTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for CreateShippingRateTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateShippingRateTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateShippingRateTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateShippingRateTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// The type of calculation to use on the shipping rate.
///
/// Can only be `fixed_amount` for now.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateShippingRateType {
    FixedAmount,
}

impl CreateShippingRateType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for CreateShippingRateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fixed_amount" => Ok(Self::FixedAmount),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateShippingRateType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateShippingRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateShippingRateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateShippingRate<'a> {
    /// Whether the shipping rate can be used for new purchases.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<UpdateShippingRateFixedAmount<'a>>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateShippingRateTaxBehavior>,
}
impl<'a> UpdateShippingRate<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Describes a fixed amount to charge for shipping.
///
/// Must be present if type is `fixed_amount`.
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateShippingRateFixedAmount<'a> {
    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<
        &'a std::collections::HashMap<
            stripe_types::Currency,
            UpdateShippingRateFixedAmountCurrencyOptions,
        >,
    >,
}
impl<'a> UpdateShippingRateFixedAmount<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Shipping rates defined in each available currency option.
///
/// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateShippingRateFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior>,
}
impl UpdateShippingRateFixedAmountCurrencyOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
/// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
///
/// One of `inclusive`, `exclusive`, or `unspecified`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UpdateShippingRateTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateShippingRateTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for UpdateShippingRateTaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exclusive" => Ok(Self::Exclusive),
            "inclusive" => Ok(Self::Inclusive),
            "unspecified" => Ok(Self::Unspecified),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateShippingRateTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateShippingRateTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for UpdateShippingRateTaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
