use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListRadarEarlyFraudWarningBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    charge: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    payment_intent: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListRadarEarlyFraudWarningBuilder<'a> {
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
pub struct ListRadarEarlyFraudWarning<'a> {
    inner: ListRadarEarlyFraudWarningBuilder<'a>,
}
impl<'a> ListRadarEarlyFraudWarning<'a> {
    /// Construct a new `ListRadarEarlyFraudWarning`.
    pub fn new() -> Self {
        Self { inner: ListRadarEarlyFraudWarningBuilder::new() }
    }
    /// Only return early fraud warnings for the charge specified by this charge ID.
    pub fn charge(mut self, charge: &'a str) -> Self {
        self.inner.charge = Some(charge);
        self
    }
    /// Only return early fraud warnings that were created during the given date interval.
    pub fn created(mut self, created: stripe_types::RangeQueryTs) -> Self {
        self.inner.created = Some(created);
        self
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
    /// Only return early fraud warnings for charges that were created by the PaymentIntent specified by this PaymentIntent ID.
    pub fn payment_intent(mut self, payment_intent: &'a str) -> Self {
        self.inner.payment_intent = Some(payment_intent);
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListRadarEarlyFraudWarning<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListRadarEarlyFraudWarning<'_> {
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
        stripe_client_core::ListPaginator::new_list("/radar/early_fraud_warnings", self.inner)
    }
}

impl StripeRequest for ListRadarEarlyFraudWarning<'_> {
    type Output = stripe_types::List<stripe_fraud::RadarEarlyFraudWarning>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/radar/early_fraud_warnings").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveRadarEarlyFraudWarningBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveRadarEarlyFraudWarningBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an early fraud warning that has previously been created.
///
/// Please refer to the [early fraud warning](https://stripe.com/docs/api#early_fraud_warning_object) object reference for more details.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveRadarEarlyFraudWarning<'a> {
    inner: RetrieveRadarEarlyFraudWarningBuilder<'a>,
    early_fraud_warning: &'a stripe_fraud::RadarEarlyFraudWarningId,
}
impl<'a> RetrieveRadarEarlyFraudWarning<'a> {
    /// Construct a new `RetrieveRadarEarlyFraudWarning`.
    pub fn new(early_fraud_warning: &'a stripe_fraud::RadarEarlyFraudWarningId) -> Self {
        Self { early_fraud_warning, inner: RetrieveRadarEarlyFraudWarningBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveRadarEarlyFraudWarning<'_> {
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

impl StripeRequest for RetrieveRadarEarlyFraudWarning<'_> {
    type Output = stripe_fraud::RadarEarlyFraudWarning;

    fn build(&self) -> RequestBuilder {
        let early_fraud_warning = self.early_fraud_warning;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/radar/early_fraud_warnings/{early_fraud_warning}"),
        )
        .query(&self.inner)
    }
}
