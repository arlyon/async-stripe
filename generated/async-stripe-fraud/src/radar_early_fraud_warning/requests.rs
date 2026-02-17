use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListRadarEarlyFraudWarningBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListRadarEarlyFraudWarningBuilder {
    fn new() -> Self {
        Self {
            charge: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            payment_intent: None,
            starting_after: None,
        }
    }
}
/// Returns a list of early fraud warnings.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListRadarEarlyFraudWarning {
    inner: ListRadarEarlyFraudWarningBuilder,
}
impl ListRadarEarlyFraudWarning {
    /// Construct a new `ListRadarEarlyFraudWarning`.
    pub fn new() -> Self {
        Self { inner: ListRadarEarlyFraudWarningBuilder::new() }
    }
    /// Only return early fraud warnings for the charge specified by this charge ID.
    pub fn charge(mut self, charge: impl Into<String>) -> Self {
        self.inner.charge = Some(charge.into());
        self
    }
    /// Only return early fraud warnings that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
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
    /// Only return early fraud warnings for charges that were created by the PaymentIntent specified by this PaymentIntent ID.
    pub fn payment_intent(mut self, payment_intent: impl Into<String>) -> Self {
        self.inner.payment_intent = Some(payment_intent.into());
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
impl Default for ListRadarEarlyFraudWarning {
    fn default() -> Self {
        Self::new()
    }
}
impl ListRadarEarlyFraudWarning {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_fraud::RadarEarlyFraudWarning>>
    {
        stripe_client_core::ListPaginator::new_list("/radar/early_fraud_warnings", &self.inner)
    }
}

impl StripeRequest for ListRadarEarlyFraudWarning {
    type Output = stripe_types::List<stripe_fraud::RadarEarlyFraudWarning>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/radar/early_fraud_warnings").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveRadarEarlyFraudWarningBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveRadarEarlyFraudWarningBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an early fraud warning that has previously been created.
///
/// Please refer to the [early fraud warning](https://stripe.com/docs/api#early_fraud_warning_object) object reference for more details.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveRadarEarlyFraudWarning {
    inner: RetrieveRadarEarlyFraudWarningBuilder,
    early_fraud_warning: stripe_fraud::RadarEarlyFraudWarningId,
}
impl RetrieveRadarEarlyFraudWarning {
    /// Construct a new `RetrieveRadarEarlyFraudWarning`.
    pub fn new(early_fraud_warning: impl Into<stripe_fraud::RadarEarlyFraudWarningId>) -> Self {
        Self {
            early_fraud_warning: early_fraud_warning.into(),
            inner: RetrieveRadarEarlyFraudWarningBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveRadarEarlyFraudWarning {
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

impl StripeRequest for RetrieveRadarEarlyFraudWarning {
    type Output = stripe_fraud::RadarEarlyFraudWarning;

    fn build(&self) -> RequestBuilder {
        let early_fraud_warning = &self.early_fraud_warning;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/radar/early_fraud_warnings/{early_fraud_warning}"),
        )
        .query(&self.inner)
    }
}
