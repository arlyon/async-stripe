// ======================================
// This file was automatically generated.
// ======================================

use crate::client::{Client, Response};
use crate::ids::{ShippingRateId, TaxCodeId};
use crate::params::{
    CurrencyMap, Expand, Expandable, List, Metadata, Object, Paginable, RangeQuery, Timestamp,
};
use crate::resources::{Currency, TaxCode};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ShippingRate".
///
/// For more details see <https://stripe.com/docs/api/shipping_rates/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingRate {
    /// Unique identifier for the object.
    pub id: ShippingRateId,

    /// Whether the shipping rate can be used for new purchases.
    ///
    /// Defaults to `true`.
    pub active: bool,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub delivery_estimate: Option<ShippingRateDeliveryEstimate>,

    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<ShippingRateFixedAmount>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: Option<ShippingRateTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    pub tax_code: Option<Expandable<TaxCode>>,

    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    pub type_: ShippingRateType,
}

impl ShippingRate {
    /// Returns a list of your shipping rates.
    pub fn list(client: &Client, params: &ListShippingRates<'_>) -> Response<List<ShippingRate>> {
        client.get_query("/shipping_rates", params)
    }

    /// Creates a new shipping rate object.
    pub fn create(client: &Client, params: CreateShippingRate<'_>) -> Response<ShippingRate> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form("/shipping_rates", &params)
    }

    /// Returns the shipping rate object with the given ID.
    pub fn retrieve(
        client: &Client,
        id: &ShippingRateId,
        expand: &[&str],
    ) -> Response<ShippingRate> {
        client.get_query(&format!("/shipping_rates/{}", id), Expand { expand })
    }

    /// Updates an existing shipping rate object.
    pub fn update(
        client: &Client,
        id: &ShippingRateId,
        params: UpdateShippingRate<'_>,
    ) -> Response<ShippingRate> {
        #[allow(clippy::needless_borrows_for_generic_args)]
        client.post_form(&format!("/shipping_rates/{}", id), &params)
    }
}

impl Object for ShippingRate {
    type Id = ShippingRateId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "shipping_rate"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingRateDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    pub maximum: Option<ShippingRateDeliveryEstimateBound>,

    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    pub minimum: Option<ShippingRateDeliveryEstimateBound>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingRateDeliveryEstimateBound {
    /// A unit of time.
    pub unit: ShippingRateDeliveryEstimateBoundUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingRateFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CurrencyMap<ShippingRateCurrencyOption>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ShippingRateCurrencyOption {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    pub tax_behavior: ShippingRateCurrencyOptionTaxBehavior,
}

/// The parameters for `ShippingRate::create`.
#[derive(Clone, Debug, Serialize)]
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
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<CreateShippingRateFixedAmount>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<ShippingRateTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<TaxCodeId>,

    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<ShippingRateType>,
}

impl<'a> CreateShippingRate<'a> {
    pub fn new(display_name: &'a str) -> Self {
        CreateShippingRate {
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

/// The parameters for `ShippingRate::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListShippingRates<'a> {
    /// Only return shipping rates that are active or inactive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A filter on the list, based on the object `created` field.
    ///
    /// The value can be a string with an integer Unix timestamp, or it can be a dictionary with a number of different query options.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return shipping rates for the given currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<ShippingRateId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<ShippingRateId>,
}

impl<'a> ListShippingRates<'a> {
    pub fn new() -> Self {
        ListShippingRates {
            active: Default::default(),
            created: Default::default(),
            currency: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
        }
    }
}
impl Paginable for ListShippingRates<'_> {
    type O = ShippingRate;
    fn set_last(&mut self, item: Self::O) {
        self.starting_after = Some(item.id());
    }
}
/// The parameters for `ShippingRate::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateShippingRate<'a> {
    /// Whether the shipping rate can be used for new purchases.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<UpdateShippingRateFixedAmount>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<ShippingRateTaxBehavior>,
}

impl<'a> UpdateShippingRate<'a> {
    pub fn new() -> Self {
        UpdateShippingRate {
            active: Default::default(),
            expand: Default::default(),
            fixed_amount: Default::default(),
            metadata: Default::default(),
            tax_behavior: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateShippingRateFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Currency,

    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CurrencyMap<CreateShippingRateFixedAmountCurrencyOptions>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UpdateShippingRateFixedAmount {
    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<CurrencyMap<UpdateShippingRateFixedAmountCurrencyOptions>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateShippingRateDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: CreateShippingRateDeliveryEstimateMaximumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateShippingRateDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: CreateShippingRateDeliveryEstimateMinimumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CreateShippingRateFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

/// An enum representing the possible values of an `CreateShippingRateDeliveryEstimateMaximum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
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
            CreateShippingRateDeliveryEstimateMaximumUnit::BusinessDay => "business_day",
            CreateShippingRateDeliveryEstimateMaximumUnit::Day => "day",
            CreateShippingRateDeliveryEstimateMaximumUnit::Hour => "hour",
            CreateShippingRateDeliveryEstimateMaximumUnit::Month => "month",
            CreateShippingRateDeliveryEstimateMaximumUnit::Week => "week",
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
impl std::default::Default for CreateShippingRateDeliveryEstimateMaximumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `CreateShippingRateDeliveryEstimateMinimum`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
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
            CreateShippingRateDeliveryEstimateMinimumUnit::BusinessDay => "business_day",
            CreateShippingRateDeliveryEstimateMinimumUnit::Day => "day",
            CreateShippingRateDeliveryEstimateMinimumUnit::Hour => "hour",
            CreateShippingRateDeliveryEstimateMinimumUnit::Month => "month",
            CreateShippingRateDeliveryEstimateMinimumUnit::Week => "week",
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
impl std::default::Default for CreateShippingRateDeliveryEstimateMinimumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `CreateShippingRateFixedAmountCurrencyOptions`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior::Exclusive => "exclusive",
            CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior::Inclusive => "inclusive",
            CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior::Unspecified => "unspecified",
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
impl std::default::Default for CreateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `ShippingRateCurrencyOption`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateCurrencyOptionTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl ShippingRateCurrencyOptionTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            ShippingRateCurrencyOptionTaxBehavior::Exclusive => "exclusive",
            ShippingRateCurrencyOptionTaxBehavior::Inclusive => "inclusive",
            ShippingRateCurrencyOptionTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for ShippingRateCurrencyOptionTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateCurrencyOptionTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ShippingRateCurrencyOptionTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `ShippingRateDeliveryEstimateBound`'s `unit` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateDeliveryEstimateBoundUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl ShippingRateDeliveryEstimateBoundUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            ShippingRateDeliveryEstimateBoundUnit::BusinessDay => "business_day",
            ShippingRateDeliveryEstimateBoundUnit::Day => "day",
            ShippingRateDeliveryEstimateBoundUnit::Hour => "hour",
            ShippingRateDeliveryEstimateBoundUnit::Month => "month",
            ShippingRateDeliveryEstimateBoundUnit::Week => "week",
        }
    }
}

impl AsRef<str> for ShippingRateDeliveryEstimateBoundUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateDeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ShippingRateDeliveryEstimateBoundUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

/// An enum representing the possible values of an `ShippingRate`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl ShippingRateTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            ShippingRateTaxBehavior::Exclusive => "exclusive",
            ShippingRateTaxBehavior::Inclusive => "inclusive",
            ShippingRateTaxBehavior::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for ShippingRateTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ShippingRateTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

/// An enum representing the possible values of an `ShippingRate`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateType {
    FixedAmount,
}

impl ShippingRateType {
    pub fn as_str(self) -> &'static str {
        match self {
            ShippingRateType::FixedAmount => "fixed_amount",
        }
    }
}

impl AsRef<str> for ShippingRateType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ShippingRateType {
    fn default() -> Self {
        Self::FixedAmount
    }
}

/// An enum representing the possible values of an `UpdateShippingRateFixedAmountCurrencyOptions`'s `tax_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior::Exclusive => "exclusive",
            UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior::Inclusive => "inclusive",
            UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior::Unspecified => "unspecified",
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
impl std::default::Default for UpdateShippingRateFixedAmountCurrencyOptionsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
