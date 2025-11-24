use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListBillingMeterBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_billing::BillingMeterStatus>,
}
impl ListBillingMeterBuilder {
    fn new() -> Self {
        Self { ending_before: None, expand: None, limit: None, starting_after: None, status: None }
    }
}
/// Retrieve a list of billing meters.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListBillingMeter {
    inner: ListBillingMeterBuilder,
}
impl ListBillingMeter {
    /// Construct a new `ListBillingMeter`.
    pub fn new() -> Self {
        Self { inner: ListBillingMeterBuilder::new() }
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
    /// Filter results to only include meters with the given status.
    pub fn status(mut self, status: impl Into<stripe_billing::BillingMeterStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl Default for ListBillingMeter {
    fn default() -> Self {
        Self::new()
    }
}
impl ListBillingMeter {
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
        stripe_client_core::ListPaginator::new_list("/billing/meters", &self.inner)
    }
}

impl StripeRequest for ListBillingMeter {
    type Output = stripe_types::List<stripe_billing::BillingMeter>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/billing/meters").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveBillingMeterBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveBillingMeterBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a billing meter given an ID.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveBillingMeter {
    inner: RetrieveBillingMeterBuilder,
    id: stripe_billing::BillingMeterId,
}
impl RetrieveBillingMeter {
    /// Construct a new `RetrieveBillingMeter`.
    pub fn new(id: impl Into<stripe_billing::BillingMeterId>) -> Self {
        Self { id: id.into(), inner: RetrieveBillingMeterBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveBillingMeter {
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

impl StripeRequest for RetrieveBillingMeter {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/billing/meters/{id}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateBillingMeterBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_mapping: Option<CreateBillingMeterCustomerMapping>,
    default_aggregation: CreateBillingMeterDefaultAggregation,
    display_name: String,
    event_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_time_window: Option<stripe_billing::BillingMeterEventTimeWindow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_settings: Option<CreateBillingMeterValueSettings>,
}
impl CreateBillingMeterBuilder {
    fn new(
        default_aggregation: impl Into<CreateBillingMeterDefaultAggregation>,
        display_name: impl Into<String>,
        event_name: impl Into<String>,
    ) -> Self {
        Self {
            customer_mapping: None,
            default_aggregation: default_aggregation.into(),
            display_name: display_name.into(),
            event_name: event_name.into(),
            event_time_window: None,
            expand: None,
            value_settings: None,
        }
    }
}
/// Fields that specify how to map a meter event to a customer.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterCustomerMapping {
    /// The key in the meter event payload to use for mapping the event to a customer.
    pub event_payload_key: String,
    /// The method for mapping a meter event to a customer. Must be `by_id`.
    #[serde(rename = "type")]
    pub type_: CreateBillingMeterCustomerMappingType,
}
impl CreateBillingMeterCustomerMapping {
    pub fn new(
        event_payload_key: impl Into<String>,
        type_: impl Into<CreateBillingMeterCustomerMappingType>,
    ) -> Self {
        Self { event_payload_key: event_payload_key.into(), type_: type_.into() }
    }
}
/// The method for mapping a meter event to a customer. Must be `by_id`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingMeterCustomerMappingType {
    ById,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingMeterCustomerMappingType {
    pub fn as_str(&self) -> &str {
        use CreateBillingMeterCustomerMappingType::*;
        match self {
            ById => "by_id",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateBillingMeterCustomerMappingType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingMeterCustomerMappingType::*;
        match s {
            "by_id" => Ok(ById),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingMeterCustomerMappingType"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The default settings to aggregate a meter's events with.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterDefaultAggregation {
    /// Specifies how events are aggregated.
    /// Allowed values are `count` to count the number of events, `sum` to sum each event's value and `last` to take the last event's value in the window.
    pub formula: CreateBillingMeterDefaultAggregationFormula,
}
impl CreateBillingMeterDefaultAggregation {
    pub fn new(formula: impl Into<CreateBillingMeterDefaultAggregationFormula>) -> Self {
        Self { formula: formula.into() }
    }
}
/// Specifies how events are aggregated.
/// Allowed values are `count` to count the number of events, `sum` to sum each event's value and `last` to take the last event's value in the window.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingMeterDefaultAggregationFormula {
    Count,
    Last,
    Sum,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingMeterDefaultAggregationFormula {
    pub fn as_str(&self) -> &str {
        use CreateBillingMeterDefaultAggregationFormula::*;
        match self {
            Count => "count",
            Last => "last",
            Sum => "sum",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateBillingMeterDefaultAggregationFormula {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingMeterDefaultAggregationFormula::*;
        match s {
            "count" => Ok(Count),
            "last" => Ok(Last),
            "sum" => Ok(Sum),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingMeterDefaultAggregationFormula"
                );
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Fields that specify how to calculate a meter event's value.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeterValueSettings {
    /// The key in the usage event payload to use as the value for this meter.
    /// For example, if the event payload contains usage on a `bytes_used` field, then set the event_payload_key to "bytes_used".
    pub event_payload_key: String,
}
impl CreateBillingMeterValueSettings {
    pub fn new(event_payload_key: impl Into<String>) -> Self {
        Self { event_payload_key: event_payload_key.into() }
    }
}
/// Creates a billing meter.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingMeter {
    inner: CreateBillingMeterBuilder,
}
impl CreateBillingMeter {
    /// Construct a new `CreateBillingMeter`.
    pub fn new(
        default_aggregation: impl Into<CreateBillingMeterDefaultAggregation>,
        display_name: impl Into<String>,
        event_name: impl Into<String>,
    ) -> Self {
        Self {
            inner: CreateBillingMeterBuilder::new(
                default_aggregation.into(),
                display_name.into(),
                event_name.into(),
            ),
        }
    }
    /// Fields that specify how to map a meter event to a customer.
    pub fn customer_mapping(
        mut self,
        customer_mapping: impl Into<CreateBillingMeterCustomerMapping>,
    ) -> Self {
        self.inner.customer_mapping = Some(customer_mapping.into());
        self
    }
    /// The time window which meter events have been pre-aggregated for, if any.
    pub fn event_time_window(
        mut self,
        event_time_window: impl Into<stripe_billing::BillingMeterEventTimeWindow>,
    ) -> Self {
        self.inner.event_time_window = Some(event_time_window.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Fields that specify how to calculate a meter event's value.
    pub fn value_settings(
        mut self,
        value_settings: impl Into<CreateBillingMeterValueSettings>,
    ) -> Self {
        self.inner.value_settings = Some(value_settings.into());
        self
    }
}
impl CreateBillingMeter {
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

impl StripeRequest for CreateBillingMeter {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing/meters").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateBillingMeterBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl UpdateBillingMeterBuilder {
    fn new() -> Self {
        Self { display_name: None, expand: None }
    }
}
/// Updates a billing meter.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateBillingMeter {
    inner: UpdateBillingMeterBuilder,
    id: stripe_billing::BillingMeterId,
}
impl UpdateBillingMeter {
    /// Construct a new `UpdateBillingMeter`.
    pub fn new(id: impl Into<stripe_billing::BillingMeterId>) -> Self {
        Self { id: id.into(), inner: UpdateBillingMeterBuilder::new() }
    }
    /// The meter’s name. Not visible to the customer.
    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.inner.display_name = Some(display_name.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl UpdateBillingMeter {
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

impl StripeRequest for UpdateBillingMeter {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/meters/{id}")).form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct DeactivateBillingMeterBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl DeactivateBillingMeterBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// When a meter is deactivated, no more meter events will be accepted for this meter.
/// You can’t attach a deactivated meter to a price.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeactivateBillingMeter {
    inner: DeactivateBillingMeterBuilder,
    id: stripe_billing::BillingMeterId,
}
impl DeactivateBillingMeter {
    /// Construct a new `DeactivateBillingMeter`.
    pub fn new(id: impl Into<stripe_billing::BillingMeterId>) -> Self {
        Self { id: id.into(), inner: DeactivateBillingMeterBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DeactivateBillingMeter {
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

impl StripeRequest for DeactivateBillingMeter {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/meters/{id}/deactivate"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct ReactivateBillingMeterBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ReactivateBillingMeterBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// When a meter is reactivated, events for this meter can be accepted and you can attach the meter to a price.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ReactivateBillingMeter {
    inner: ReactivateBillingMeterBuilder,
    id: stripe_billing::BillingMeterId,
}
impl ReactivateBillingMeter {
    /// Construct a new `ReactivateBillingMeter`.
    pub fn new(id: impl Into<stripe_billing::BillingMeterId>) -> Self {
        Self { id: id.into(), inner: ReactivateBillingMeterBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ReactivateBillingMeter {
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

impl StripeRequest for ReactivateBillingMeter {
    type Output = stripe_billing::BillingMeter;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/meters/{id}/reactivate"))
            .form(&self.inner)
    }
}
