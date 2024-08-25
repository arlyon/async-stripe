use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListPaymentMethodDomainBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
impl ListPaymentMethodDomainBuilder {
    fn new() -> Self {
        Self {
            domain_name: None,
            enabled: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Lists the details of existing payment method domains.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListPaymentMethodDomain {
    inner: ListPaymentMethodDomainBuilder,
}
impl ListPaymentMethodDomain {
    /// Construct a new `ListPaymentMethodDomain`.
    pub fn new() -> Self {
        Self { inner: ListPaymentMethodDomainBuilder::new() }
    }
    /// The domain name that this payment method domain object represents.
    pub fn domain_name(mut self, domain_name: impl Into<String>) -> Self {
        self.inner.domain_name = Some(domain_name.into());
        self
    }
    /// Whether this payment method domain is enabled.
    /// If the domain is not enabled, payment methods will not appear in Elements.
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.inner.enabled = Some(enabled.into());
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
impl Default for ListPaymentMethodDomain {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPaymentMethodDomain {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_payment::PaymentMethodDomain>>
    {
        stripe_client_core::ListPaginator::new_list("/payment_method_domains", &self.inner)
    }
}

impl StripeRequest for ListPaymentMethodDomain {
    type Output = stripe_types::List<stripe_payment::PaymentMethodDomain>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/payment_method_domains").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrievePaymentMethodDomainBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrievePaymentMethodDomainBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing payment method domain.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrievePaymentMethodDomain {
    inner: RetrievePaymentMethodDomainBuilder,
    payment_method_domain: stripe_payment::PaymentMethodDomainId,
}
impl RetrievePaymentMethodDomain {
    /// Construct a new `RetrievePaymentMethodDomain`.
    pub fn new(payment_method_domain: impl Into<stripe_payment::PaymentMethodDomainId>) -> Self {
        Self {
            payment_method_domain: payment_method_domain.into(),
            inner: RetrievePaymentMethodDomainBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrievePaymentMethodDomain {
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

impl StripeRequest for RetrievePaymentMethodDomain {
    type Output = stripe_payment::PaymentMethodDomain;

    fn build(&self) -> RequestBuilder {
        let payment_method_domain = &self.payment_method_domain;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/payment_method_domains/{payment_method_domain}"),
        )
        .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreatePaymentMethodDomainBuilder {
    domain_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CreatePaymentMethodDomainBuilder {
    fn new(domain_name: impl Into<String>) -> Self {
        Self { domain_name: domain_name.into(), enabled: None, expand: None }
    }
}
/// Creates a payment method domain.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreatePaymentMethodDomain {
    inner: CreatePaymentMethodDomainBuilder,
}
impl CreatePaymentMethodDomain {
    /// Construct a new `CreatePaymentMethodDomain`.
    pub fn new(domain_name: impl Into<String>) -> Self {
        Self { inner: CreatePaymentMethodDomainBuilder::new(domain_name.into()) }
    }
    /// Whether this payment method domain is enabled.
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.inner.enabled = Some(enabled.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreatePaymentMethodDomain {
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

impl StripeRequest for CreatePaymentMethodDomain {
    type Output = stripe_payment::PaymentMethodDomain;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/payment_method_domains").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct UpdatePaymentMethodDomainBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl UpdatePaymentMethodDomainBuilder {
    fn new() -> Self {
        Self { enabled: None, expand: None }
    }
}
/// Updates an existing payment method domain.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct UpdatePaymentMethodDomain {
    inner: UpdatePaymentMethodDomainBuilder,
    payment_method_domain: stripe_payment::PaymentMethodDomainId,
}
impl UpdatePaymentMethodDomain {
    /// Construct a new `UpdatePaymentMethodDomain`.
    pub fn new(payment_method_domain: impl Into<stripe_payment::PaymentMethodDomainId>) -> Self {
        Self {
            payment_method_domain: payment_method_domain.into(),
            inner: UpdatePaymentMethodDomainBuilder::new(),
        }
    }
    /// Whether this payment method domain is enabled.
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.inner.enabled = Some(enabled.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl UpdatePaymentMethodDomain {
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

impl StripeRequest for UpdatePaymentMethodDomain {
    type Output = stripe_payment::PaymentMethodDomain;

    fn build(&self) -> RequestBuilder {
        let payment_method_domain = &self.payment_method_domain;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_method_domains/{payment_method_domain}"),
        )
        .form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ValidatePaymentMethodDomainBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl ValidatePaymentMethodDomainBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Some payment methods such as Apple Pay require additional steps to verify a domain.
/// If the requirements weren’t satisfied when the domain was created, the payment method will be inactive on the domain.
/// The payment method doesn’t appear in Elements for this domain until it is active.
///
/// To activate a payment method on an existing payment method domain, complete the required validation steps specific to the payment method, and then validate the payment method domain with this endpoint.
///
/// Related guides: [Payment method domains](https://stripe.com/docs/payments/payment-methods/pmd-registration).
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ValidatePaymentMethodDomain {
    inner: ValidatePaymentMethodDomainBuilder,
    payment_method_domain: stripe_payment::PaymentMethodDomainId,
}
impl ValidatePaymentMethodDomain {
    /// Construct a new `ValidatePaymentMethodDomain`.
    pub fn new(payment_method_domain: impl Into<stripe_payment::PaymentMethodDomainId>) -> Self {
        Self {
            payment_method_domain: payment_method_domain.into(),
            inner: ValidatePaymentMethodDomainBuilder::new(),
        }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl ValidatePaymentMethodDomain {
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

impl StripeRequest for ValidatePaymentMethodDomain {
    type Output = stripe_payment::PaymentMethodDomain;

    fn build(&self) -> RequestBuilder {
        let payment_method_domain = &self.payment_method_domain;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_method_domains/{payment_method_domain}/validate"),
        )
        .form(&self.inner)
    }
}
