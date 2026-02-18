use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListBillingAlertBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    alert_type: Option<stripe_billing::BillingAlertAlertType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    meter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListBillingAlertBuilder {
    fn new() -> Self {
        Self {
            alert_type: None,
            ending_before: None,
            expand: None,
            limit: None,
            meter: None,
            starting_after: None,
        }
    }
}
/// Lists billing active and inactive alerts
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListBillingAlert {
    inner: ListBillingAlertBuilder,
}
impl ListBillingAlert {
    /// Construct a new `ListBillingAlert`.
    pub fn new() -> Self {
        Self { inner: ListBillingAlertBuilder::new() }
    }
    /// Filter results to only include this type of alert.
    pub fn alert_type(
        mut self,
        alert_type: impl Into<stripe_billing::BillingAlertAlertType>,
    ) -> Self {
        self.inner.alert_type = Some(alert_type.into());
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
    /// Filter results to only include alerts with the given meter.
    pub fn meter(mut self, meter: impl Into<String>) -> Self {
        self.inner.meter = Some(meter.into());
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
impl Default for ListBillingAlert {
    fn default() -> Self {
        Self::new()
    }
}
impl ListBillingAlert {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_billing::BillingAlert>> {
        stripe_client_core::ListPaginator::new_list("/billing/alerts", &self.inner)
    }
}

impl StripeRequest for ListBillingAlert {
    type Output = stripe_types::List<stripe_billing::BillingAlert>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/billing/alerts").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveBillingAlertBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveBillingAlertBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a billing alert given an ID
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveBillingAlert {
    inner: RetrieveBillingAlertBuilder,
    id: stripe_billing::BillingAlertId,
}
impl RetrieveBillingAlert {
    /// Construct a new `RetrieveBillingAlert`.
    pub fn new(id: impl Into<stripe_billing::BillingAlertId>) -> Self {
        Self { id: id.into(), inner: RetrieveBillingAlertBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveBillingAlert {
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

impl StripeRequest for RetrieveBillingAlert {
    type Output = stripe_billing::BillingAlert;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/billing/alerts/{id}")).query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct CreateBillingAlertBuilder {
    alert_type: stripe_billing::BillingAlertAlertType,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    usage_threshold: Option<CreateBillingAlertUsageThreshold>,
}
impl CreateBillingAlertBuilder {
    fn new(
        alert_type: impl Into<stripe_billing::BillingAlertAlertType>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            alert_type: alert_type.into(),
            expand: None,
            title: title.into(),
            usage_threshold: None,
        }
    }
}
/// The configuration of the usage threshold.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateBillingAlertUsageThreshold {
    /// The filters allows limiting the scope of this usage alert.
    /// You can only specify up to one filter at this time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<CreateBillingAlertUsageThresholdFilters>>,
    /// Defines at which value the alert will fire.
    pub gte: i64,
    /// The [Billing Meter](/api/billing/meter) ID whose usage is monitored.
    pub meter: String,
    /// Defines how the alert will behave.
    pub recurrence: CreateBillingAlertUsageThresholdRecurrence,
}
impl CreateBillingAlertUsageThreshold {
    pub fn new(
        gte: impl Into<i64>,
        meter: impl Into<String>,
        recurrence: impl Into<CreateBillingAlertUsageThresholdRecurrence>,
    ) -> Self {
        Self { filters: None, gte: gte.into(), meter: meter.into(), recurrence: recurrence.into() }
    }
}
/// The filters allows limiting the scope of this usage alert.
/// You can only specify up to one filter at this time.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct CreateBillingAlertUsageThresholdFilters {
    /// Limit the scope to this usage alert only to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    /// What type of filter is being applied to this usage alert.
    #[serde(rename = "type")]
    pub type_: CreateBillingAlertUsageThresholdFiltersType,
}
impl CreateBillingAlertUsageThresholdFilters {
    pub fn new(type_: impl Into<CreateBillingAlertUsageThresholdFiltersType>) -> Self {
        Self { customer: None, type_: type_.into() }
    }
}
/// What type of filter is being applied to this usage alert.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingAlertUsageThresholdFiltersType {
    Customer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingAlertUsageThresholdFiltersType {
    pub fn as_str(&self) -> &str {
        use CreateBillingAlertUsageThresholdFiltersType::*;
        match self {
            Customer => "customer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateBillingAlertUsageThresholdFiltersType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingAlertUsageThresholdFiltersType::*;
        match s {
            "customer" => Ok(Customer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingAlertUsageThresholdFiltersType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateBillingAlertUsageThresholdFiltersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingAlertUsageThresholdFiltersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingAlertUsageThresholdFiltersType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingAlertUsageThresholdFiltersType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Defines how the alert will behave.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateBillingAlertUsageThresholdRecurrence {
    OneTime,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateBillingAlertUsageThresholdRecurrence {
    pub fn as_str(&self) -> &str {
        use CreateBillingAlertUsageThresholdRecurrence::*;
        match self {
            OneTime => "one_time",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateBillingAlertUsageThresholdRecurrence {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateBillingAlertUsageThresholdRecurrence::*;
        match s {
            "one_time" => Ok(OneTime),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateBillingAlertUsageThresholdRecurrence"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateBillingAlertUsageThresholdRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateBillingAlertUsageThresholdRecurrence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for CreateBillingAlertUsageThresholdRecurrence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateBillingAlertUsageThresholdRecurrence {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Creates a billing alert
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateBillingAlert {
    inner: CreateBillingAlertBuilder,
}
impl CreateBillingAlert {
    /// Construct a new `CreateBillingAlert`.
    pub fn new(
        alert_type: impl Into<stripe_billing::BillingAlertAlertType>,
        title: impl Into<String>,
    ) -> Self {
        Self { inner: CreateBillingAlertBuilder::new(alert_type.into(), title.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// The configuration of the usage threshold.
    pub fn usage_threshold(
        mut self,
        usage_threshold: impl Into<CreateBillingAlertUsageThreshold>,
    ) -> Self {
        self.inner.usage_threshold = Some(usage_threshold.into());
        self
    }
}
impl CreateBillingAlert {
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

impl StripeRequest for CreateBillingAlert {
    type Output = stripe_billing::BillingAlert;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/billing/alerts").form(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ActivateBillingAlertBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ActivateBillingAlertBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Reactivates this alert, allowing it to trigger again.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ActivateBillingAlert {
    inner: ActivateBillingAlertBuilder,
    id: stripe_billing::BillingAlertId,
}
impl ActivateBillingAlert {
    /// Construct a new `ActivateBillingAlert`.
    pub fn new(id: impl Into<stripe_billing::BillingAlertId>) -> Self {
        Self { id: id.into(), inner: ActivateBillingAlertBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ActivateBillingAlert {
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

impl StripeRequest for ActivateBillingAlert {
    type Output = stripe_billing::BillingAlert;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/alerts/{id}/activate"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ArchiveBillingAlertBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ArchiveBillingAlertBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Archives this alert, removing it from the list view and APIs. This is non-reversible.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ArchiveBillingAlert {
    inner: ArchiveBillingAlertBuilder,
    id: stripe_billing::BillingAlertId,
}
impl ArchiveBillingAlert {
    /// Construct a new `ArchiveBillingAlert`.
    pub fn new(id: impl Into<stripe_billing::BillingAlertId>) -> Self {
        Self { id: id.into(), inner: ArchiveBillingAlertBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ArchiveBillingAlert {
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

impl StripeRequest for ArchiveBillingAlert {
    type Output = stripe_billing::BillingAlert;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/alerts/{id}/archive"))
            .form(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct DeactivateBillingAlertBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl DeactivateBillingAlertBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Deactivates this alert, preventing it from triggering.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeactivateBillingAlert {
    inner: DeactivateBillingAlertBuilder,
    id: stripe_billing::BillingAlertId,
}
impl DeactivateBillingAlert {
    /// Construct a new `DeactivateBillingAlert`.
    pub fn new(id: impl Into<stripe_billing::BillingAlertId>) -> Self {
        Self { id: id.into(), inner: DeactivateBillingAlertBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl DeactivateBillingAlert {
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

impl StripeRequest for DeactivateBillingAlert {
    type Output = stripe_billing::BillingAlert;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Post, format!("/billing/alerts/{id}/deactivate"))
            .form(&self.inner)
    }
}
