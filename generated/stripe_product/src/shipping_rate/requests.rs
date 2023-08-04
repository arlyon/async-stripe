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
impl<'a> ListShippingRate<'a> {
    /// Returns a list of your shipping rates.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::ShippingRate>> {
        client.get_query("/shipping_rates", self)
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
impl<'a> RetrieveShippingRate<'a> {
    /// Returns the shipping rate object with the given ID.
    pub fn send(
        &self,
        client: &stripe::Client,
        shipping_rate_token: &stripe_types::shipping_rate::ShippingRateId,
    ) -> stripe::Response<stripe_types::ShippingRate> {
        client.get_query(
            &format!(
                "/shipping_rates/{shipping_rate_token}",
                shipping_rate_token = shipping_rate_token
            ),
            self,
        )
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
    pub tax_behavior: Option<TaxBehavior>,
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
    pub maximum: Option<DeliveryEstimateBound>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<DeliveryEstimateBound>,
}
impl CreateShippingRateDeliveryEstimate {
    pub fn new() -> Self {
        Self::default()
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
    pub tax_behavior: Option<TaxBehavior>,
}
impl CreateShippingRateFixedAmountCurrencyOptions {
    pub fn new(amount: i64) -> Self {
        Self { amount, tax_behavior: Default::default() }
    }
}
/// The type of calculation to use on the shipping rate.
///
/// Can only be `fixed_amount` for now.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateShippingRateType {
    FixedAmount,
}

impl CreateShippingRateType {
    pub fn as_str(self) -> &'static str {
        use CreateShippingRateType::*;
        match self {
            FixedAmount => "fixed_amount",
        }
    }
}

impl std::str::FromStr for CreateShippingRateType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateShippingRateType::*;
        match s {
            "fixed_amount" => Ok(FixedAmount),
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
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateShippingRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl<'a> CreateShippingRate<'a> {
    /// Creates a new shipping rate object.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_types::ShippingRate> {
        client.send_form("/shipping_rates", self, http_types::Method::Post)
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
    pub tax_behavior: Option<TaxBehavior>,
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
    pub tax_behavior: Option<TaxBehavior>,
}
impl UpdateShippingRateFixedAmountCurrencyOptions {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateShippingRate<'a> {
    /// Updates an existing shipping rate object.
    pub fn send(
        &self,
        client: &stripe::Client,
        shipping_rate_token: &stripe_types::shipping_rate::ShippingRateId,
    ) -> stripe::Response<stripe_types::ShippingRate> {
        client.send_form(
            &format!(
                "/shipping_rates/{shipping_rate_token}",
                shipping_rate_token = shipping_rate_token
            ),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Unit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl Unit {
    pub fn as_str(self) -> &'static str {
        use Unit::*;
        match self {
            BusinessDay => "business_day",
            Day => "day",
            Hour => "hour",
            Month => "month",
            Week => "week",
        }
    }
}

impl std::str::FromStr for Unit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Unit::*;
        match s {
            "business_day" => Ok(BusinessDay),
            "day" => Ok(Day),
            "hour" => Ok(Hour),
            "month" => Ok(Month),
            "week" => Ok(Week),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for Unit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl TaxBehavior {
    pub fn as_str(self) -> &'static str {
        use TaxBehavior::*;
        match self {
            Exclusive => "exclusive",
            Inclusive => "inclusive",
            Unspecified => "unspecified",
        }
    }
}

impl std::str::FromStr for TaxBehavior {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxBehavior::*;
        match s {
            "exclusive" => Ok(Exclusive),
            "inclusive" => Ok(Inclusive),
            "unspecified" => Ok(Unspecified),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for TaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for TaxBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DeliveryEstimateBound {
    /// A unit of time.
    pub unit: Unit,
    /// Must be greater than 0.
    pub value: i64,
}
impl DeliveryEstimateBound {
    pub fn new(unit: Unit, value: i64) -> Self {
        Self { unit, value }
    }
}
