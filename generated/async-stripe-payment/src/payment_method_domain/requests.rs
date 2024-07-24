use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListPaymentMethodDomainBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    domain_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListPaymentMethodDomainBuilder<'a> {
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
pub struct ListPaymentMethodDomain<'a> {
    inner: ListPaymentMethodDomainBuilder<'a>,
}
impl<'a> ListPaymentMethodDomain<'a> {
    /// Construct a new `ListPaymentMethodDomain`.
    pub fn new() -> Self {
        Self { inner: ListPaymentMethodDomainBuilder::new() }
    }
    /// The domain name that this payment method domain object represents.
    pub fn domain_name(mut self, domain_name: &'a str) -> Self {
        self.inner.domain_name = Some(domain_name);
        self
    }
    /// Whether this payment method domain is enabled.
    /// If the domain is not enabled, payment methods will not appear in Elements.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.inner.enabled = Some(enabled);
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListPaymentMethodDomain<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListPaymentMethodDomain<'_> {
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
        stripe_client_core::ListPaginator::new_list("/payment_method_domains", self.inner)
    }
}

impl StripeRequest for ListPaymentMethodDomain<'_> {
    type Output = stripe_types::List<stripe_payment::PaymentMethodDomain>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/payment_method_domains").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrievePaymentMethodDomainBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentMethodDomainBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an existing payment method domain.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrievePaymentMethodDomain<'a> {
    inner: RetrievePaymentMethodDomainBuilder<'a>,
    payment_method_domain: &'a stripe_payment::PaymentMethodDomainId,
}
impl<'a> RetrievePaymentMethodDomain<'a> {
    /// Construct a new `RetrievePaymentMethodDomain`.
    pub fn new(payment_method_domain: &'a stripe_payment::PaymentMethodDomainId) -> Self {
        Self { payment_method_domain, inner: RetrievePaymentMethodDomainBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrievePaymentMethodDomain<'_> {
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

impl StripeRequest for RetrievePaymentMethodDomain<'_> {
    type Output = stripe_payment::PaymentMethodDomain;

    fn build(&self) -> RequestBuilder {
        let payment_method_domain = self.payment_method_domain;
        RequestBuilder::new(
            StripeMethod::Get,
            format!("/payment_method_domains/{payment_method_domain}"),
        )
        .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreatePaymentMethodDomainBuilder<'a> {
    domain_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> CreatePaymentMethodDomainBuilder<'a> {
    fn new(domain_name: &'a str) -> Self {
        Self { domain_name, enabled: None, expand: None }
    }
}
/// Creates a payment method domain.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodDomain<'a> {
    inner: CreatePaymentMethodDomainBuilder<'a>,
}
impl<'a> CreatePaymentMethodDomain<'a> {
    /// Construct a new `CreatePaymentMethodDomain`.
    pub fn new(domain_name: &'a str) -> Self {
        Self { inner: CreatePaymentMethodDomainBuilder::new(domain_name) }
    }
    /// Whether this payment method domain is enabled.
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.inner.enabled = Some(enabled);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CreatePaymentMethodDomain<'_> {
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

impl StripeRequest for CreatePaymentMethodDomain<'_> {
    type Output = stripe_payment::PaymentMethodDomain;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/payment_method_domains").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdatePaymentMethodDomainBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> UpdatePaymentMethodDomainBuilder<'a> {
    fn new() -> Self {
        Self { enabled: None, expand: None }
    }
}
/// Updates an existing payment method domain.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdatePaymentMethodDomain<'a> {
    inner: UpdatePaymentMethodDomainBuilder<'a>,
    payment_method_domain: &'a stripe_payment::PaymentMethodDomainId,
}
impl<'a> UpdatePaymentMethodDomain<'a> {
    /// Construct a new `UpdatePaymentMethodDomain`.
    pub fn new(payment_method_domain: &'a stripe_payment::PaymentMethodDomainId) -> Self {
        Self { payment_method_domain, inner: UpdatePaymentMethodDomainBuilder::new() }
    }
    /// Whether this payment method domain is enabled.
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.inner.enabled = Some(enabled);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl UpdatePaymentMethodDomain<'_> {
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

impl StripeRequest for UpdatePaymentMethodDomain<'_> {
    type Output = stripe_payment::PaymentMethodDomain;

    fn build(&self) -> RequestBuilder {
        let payment_method_domain = self.payment_method_domain;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_method_domains/{payment_method_domain}"),
        )
        .form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ValidatePaymentMethodDomainBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> ValidatePaymentMethodDomainBuilder<'a> {
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
pub struct ValidatePaymentMethodDomain<'a> {
    inner: ValidatePaymentMethodDomainBuilder<'a>,
    payment_method_domain: &'a stripe_payment::PaymentMethodDomainId,
}
impl<'a> ValidatePaymentMethodDomain<'a> {
    /// Construct a new `ValidatePaymentMethodDomain`.
    pub fn new(payment_method_domain: &'a stripe_payment::PaymentMethodDomainId) -> Self {
        Self { payment_method_domain, inner: ValidatePaymentMethodDomainBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl ValidatePaymentMethodDomain<'_> {
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

impl StripeRequest for ValidatePaymentMethodDomain<'_> {
    type Output = stripe_payment::PaymentMethodDomain;

    fn build(&self) -> RequestBuilder {
        let payment_method_domain = self.payment_method_domain;
        RequestBuilder::new(
            StripeMethod::Post,
            format!("/payment_method_domains/{payment_method_domain}/validate"),
        )
        .form(&self.inner)
    }
}
