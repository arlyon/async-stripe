use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListBillingMeterBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_billing::BillingMeterStatus>,
}
impl<'a> ListBillingMeterBuilder<'a> {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None, status: None }
    }
}
/// Retrieve a list of billing meters.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListBillingMeter<'a> {
    inner: ListBillingMeterBuilder<'a>,
}
impl<'a> ListBillingMeter<'a> {
    /// Construct a new `ListBillingMeter`.
    pub fn new() -> Self {
        Self { inner: ListBillingMeterBuilder::new() }
    }
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    pub fn ending_before(mut self, ending_before: &'a str) -> Self {
        self.inner.ending_before = Some(ending_before);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    pub fn limit(mut self, limit: i64) -> Self {
        self.inner.limit = Some(limit);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
    /// Filter results to only include meters with the given status.
    pub fn status(mut self, status: stripe_billing::BillingMeterStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
}
impl<'a> Default for ListBillingMeter<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListBillingMeter<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_billing::BillingMeter>> {
        stripe_client_core::ListPaginator::new_list("/billing/meters", self.inner)
    }
}

impl StripeRequest for ListBillingMeter<'_> {
    type Output = stripe_types::List<stripe_billing::BillingMeter>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/billing/meters").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveBillingMeterBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveBillingMeterBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a billing meter given an ID
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveBillingMeter<'a> {
    inner: RetrieveBillingMeterBuilder<'a>,
    id: &'a stripe_billing::BillingMeterId,
}
impl<'a> RetrieveBillingMeter<'a> {
    /// Construct a new `RetrieveBillingMeter`.
    pub fn new(id: &'a stripe_billing::BillingMeterId) -> Self {
        Self { id, inner: RetrieveBillingMeterBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveBillingMeter<'_> {
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

impl StripeRequest for RetrieveBillingMeter<'_> {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/billing/meters/{id}")).query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateBillingMeterBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_mapping: Option<CreateBillingMeterCustomerMapping<'a>>,
    default_aggregation: CreateBillingMeterDefaultAggregation,
    display_name: &'a str,
    event_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_time_window: Option<stripe_billing::BillingMeterEventTimeWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_settings: Option<CreateBillingMeterValueSettings<'a>>,
}
impl<'a> CreateBillingMeterBuilder<'a> {
    fn new(
        default_aggregation: CreateBillingMeterDefaultAggregation,
        display_name: &'a str,
        event_name: &'a str,
    ) -> Self {
        Self {
            customer_mapping: None,
            default_aggregation,
            display_name,
            event_name,
            event_time_window: None,
            expand: None,
            value_settings: None,
        }
    }
}
/// Fields that specify how to map a meter event to a customer.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterCustomerMapping<'a> {
    /// The key in the usage event payload to use for mapping the event to a customer.
    pub event_payload_key: &'a str,
    /// The method for mapping a meter event to a customer. Must be `by_id`.
    #[serde(rename = "type")]
    pub type_: CreateBillingMeterCustomerMappingType,
}
impl<'a> CreateBillingMeterCustomerMapping<'a> {
    pub fn new(event_payload_key: &'a str, type_: CreateBillingMeterCustomerMappingType) -> Self {
        Self { event_payload_key, type_ }
    }
}
/// The method for mapping a meter event to a customer. Must be `by_id`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingMeterCustomerMappingType {
    ById,
}
impl CreateBillingMeterCustomerMappingType {
    pub fn as_str(self) -> &'static str {
        use CreateBillingMeterCustomerMappingType::*;
        match self {
            ById => "by_id",
        }
    }
}

impl std::str::FromStr for CreateBillingMeterCustomerMappingType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingMeterCustomerMappingType::*;
        match s {
            "by_id" => Ok(ById),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateBillingMeterCustomerMappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingMeterCustomerMappingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingMeterCustomerMappingType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingMeterCustomerMappingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateBillingMeterCustomerMappingType")
        })
    }
}
/// The default settings to aggregate a meter's events with.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterDefaultAggregation {
    /// Specifies how events are aggregated.
    /// Allowed values are `count` to count the number of events and `sum` to sum each event's value.
    pub formula: CreateBillingMeterDefaultAggregationFormula,
}
impl CreateBillingMeterDefaultAggregation {
    pub fn new(formula: CreateBillingMeterDefaultAggregationFormula) -> Self {
        Self { formula }
    }
}
/// Specifies how events are aggregated.
/// Allowed values are `count` to count the number of events and `sum` to sum each event's value.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateBillingMeterDefaultAggregationFormula {
    Count,
    Sum,
}
impl CreateBillingMeterDefaultAggregationFormula {
    pub fn as_str(self) -> &'static str {
        use CreateBillingMeterDefaultAggregationFormula::*;
        match self {
            Count => "count",
            Sum => "sum",
        }
    }
}

impl std::str::FromStr for CreateBillingMeterDefaultAggregationFormula {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingMeterDefaultAggregationFormula::*;
        match s {
            "count" => Ok(Count),
            "sum" => Ok(Sum),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateBillingMeterDefaultAggregationFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingMeterDefaultAggregationFormula {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingMeterDefaultAggregationFormula {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingMeterDefaultAggregationFormula {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateBillingMeterDefaultAggregationFormula",
            )
        })
    }
}
/// Fields that specify how to calculate a meter event's value.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterValueSettings<'a> {
    /// The key in the usage event payload to use as the value for this meter.
    /// For example, if the event payload contains usage on a `bytes_used` field, then set the event_payload_key to "bytes_used".
    pub event_payload_key: &'a str,
}
impl<'a> CreateBillingMeterValueSettings<'a> {
    pub fn new(event_payload_key: &'a str) -> Self {
        Self { event_payload_key }
    }
}
/// Creates a billing meter
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeter<'a> {
    inner: CreateBillingMeterBuilder<'a>,
}
impl<'a> CreateBillingMeter<'a> {
    /// Construct a new `CreateBillingMeter`.
    pub fn new(
        default_aggregation: CreateBillingMeterDefaultAggregation,
        display_name: &'a str,
        event_name: &'a str,
    ) -> Self {
        Self {
            inner: CreateBillingMeterBuilder::new(default_aggregation, display_name, event_name),
        }
    }
    /// Fields that specify how to map a meter event to a customer.
    pub fn customer_mapping(
        mut self,
        customer_mapping: CreateBillingMeterCustomerMapping<'a>,
    ) -> Self {
        self.inner.customer_mapping = Some(customer_mapping);
        self
    }
    /// The time window to pre-aggregate meter events for, if any.
    pub fn event_time_window(
        mut self,
        event_time_window: stripe_billing::BillingMeterEventTimeWindow,
    ) -> Self {
        self.inner.event_time_window = Some(event_time_window);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Fields that specify how to calculate a meter event's value.
    pub fn value_settings(mut self, value_settings: CreateBillingMeterValueSettings<'a>) -> Self {
        self.inner.value_settings = Some(value_settings);
        self
    }
}
impl CreateBillingMeter<'_> {
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

impl StripeRequest for CreateBillingMeter<'_> {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing/meters").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateBillingMeterBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> UpdateBillingMeterBuilder<'a> {
    fn new() -> Self {
        Self { display_name: None, expand: None }
    }
}
/// Updates a billing meter
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingMeter<'a> {
    inner: UpdateBillingMeterBuilder<'a>,
    id: &'a stripe_billing::BillingMeterId,
}
impl<'a> UpdateBillingMeter<'a> {
    /// Construct a new `UpdateBillingMeter`.
    pub fn new(id: &'a stripe_billing::BillingMeterId) -> Self {
        Self { id, inner: UpdateBillingMeterBuilder::new() }
    }
    /// The meter's name.
    pub fn display_name(mut self, display_name: &'a str) -> Self {
        self.inner.display_name = Some(display_name);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl UpdateBillingMeter<'_> {
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

impl StripeRequest for UpdateBillingMeter<'_> {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/meters/{id}")).form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct DeactivateBillingMeterBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> DeactivateBillingMeterBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Deactivates a billing meter
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeactivateBillingMeter<'a> {
    inner: DeactivateBillingMeterBuilder<'a>,
    id: &'a stripe_billing::BillingMeterId,
}
impl<'a> DeactivateBillingMeter<'a> {
    /// Construct a new `DeactivateBillingMeter`.
    pub fn new(id: &'a stripe_billing::BillingMeterId) -> Self {
        Self { id, inner: DeactivateBillingMeterBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl DeactivateBillingMeter<'_> {
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

impl StripeRequest for DeactivateBillingMeter<'_> {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/meters/{id}/deactivate"))
            .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ReactivateBillingMeterBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> ReactivateBillingMeterBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Reactivates a billing meter
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReactivateBillingMeter<'a> {
    inner: ReactivateBillingMeterBuilder<'a>,
    id: &'a stripe_billing::BillingMeterId,
}
impl<'a> ReactivateBillingMeter<'a> {
    /// Construct a new `ReactivateBillingMeter`.
    pub fn new(id: &'a stripe_billing::BillingMeterId) -> Self {
        Self { id, inner: ReactivateBillingMeterBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl ReactivateBillingMeter<'_> {
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

impl StripeRequest for ReactivateBillingMeter<'_> {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        let id = self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/meters/{id}/reactivate"))
            .form(&self.inner)
    }
}
