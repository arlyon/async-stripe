#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrievePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrievePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrievePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// Retrieves the details of an existing payment method domain.
    pub fn send(
        &self,
        client: &stripe::Client,
        payment_method_domain:&stripe_payment::payment_method_domain_resource_payment_method_domain::PaymentMethodDomainId,
    ) -> stripe::Response<stripe_payment::PaymentMethodDomainResourcePaymentMethodDomain> {
        client.get_query(
            &format!(
                "/payment_method_domains/{payment_method_domain}",
                payment_method_domain = payment_method_domain
            ),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListPaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// The domain name that this payment method domain object represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<&'a str>,
    /// Whether this payment method domain is enabled.
    ///
    /// If the domain is not enabled, payment methods will not appear in Elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
}
impl<'a> ListPaymentMethodDomainResourcePaymentMethodDomain<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListPaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// Lists the details of existing payment method domains.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<
        stripe_types::List<stripe_payment::PaymentMethodDomainResourcePaymentMethodDomain>,
    > {
        client.get_query("/payment_method_domains", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_payment::PaymentMethodDomainResourcePaymentMethodDomain> {
        stripe::ListPaginator::from_params("/payment_method_domains", self)
    }
}
impl<'a> stripe::PaginationParams for ListPaymentMethodDomainResourcePaymentMethodDomain<'a> {}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreatePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// The domain name that this payment method domain object represents.
    pub domain_name: &'a str,
    /// Whether this payment method domain is enabled.
    ///
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> CreatePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    pub fn new(domain_name: &'a str) -> Self {
        Self { domain_name, enabled: Default::default(), expand: Default::default() }
    }
}
impl<'a> CreatePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// Creates a payment method domain.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_payment::PaymentMethodDomainResourcePaymentMethodDomain> {
        client.send_form("/payment_method_domains", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdatePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// Whether this payment method domain is enabled.
    ///
    /// If the domain is not enabled, payment methods that require a payment method domain will not appear in Elements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> UpdatePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdatePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// Updates an existing payment method domain.
    pub fn send(
        &self,
        client: &stripe::Client,
        payment_method_domain:&stripe_payment::payment_method_domain_resource_payment_method_domain::PaymentMethodDomainId,
    ) -> stripe::Response<stripe_payment::PaymentMethodDomainResourcePaymentMethodDomain> {
        client.send_form(
            &format!(
                "/payment_method_domains/{payment_method_domain}",
                payment_method_domain = payment_method_domain
            ),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ValidatePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> ValidatePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ValidatePaymentMethodDomainResourcePaymentMethodDomain<'a> {
    /// Some payment methods such as Apple Pay require additional steps to verify a domain.
    ///
    /// If the requirements weren’t satisfied when the domain was created, the payment method will be inactive on the domain. The payment method doesn’t appear in Elements for this domain until it is active.  To activate a payment method on an existing payment method domain, complete the required validation steps specific to the payment method, and then validate the payment method domain with this endpoint.  Related guides: [Payment method domains](https://stripe.com/docs/payments/payment-methods/pmd-registration).
    pub fn send(
        &self,
        client: &stripe::Client,
        payment_method_domain:&stripe_payment::payment_method_domain_resource_payment_method_domain::PaymentMethodDomainId,
    ) -> stripe::Response<stripe_payment::PaymentMethodDomainResourcePaymentMethodDomain> {
        client.send_form(
            &format!(
                "/payment_method_domains/{payment_method_domain}/validate",
                payment_method_domain = payment_method_domain
            ),
            self,
            http_types::Method::Post,
        )
    }
}
