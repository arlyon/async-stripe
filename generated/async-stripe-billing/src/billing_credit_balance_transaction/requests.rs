use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListBillingCreditBalanceTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    credit_grant: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    customer_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListBillingCreditBalanceTransactionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListBillingCreditBalanceTransactionBuilder").finish_non_exhaustive()
    }
}
impl ListBillingCreditBalanceTransactionBuilder {
    fn new() -> Self {
        Self {
            credit_grant: None,
            customer: None,
            customer_account: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Retrieve a list of credit balance transactions.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListBillingCreditBalanceTransaction {
    inner: ListBillingCreditBalanceTransactionBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListBillingCreditBalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListBillingCreditBalanceTransaction").finish_non_exhaustive()
    }
}
impl ListBillingCreditBalanceTransaction {
    /// Construct a new `ListBillingCreditBalanceTransaction`.
    pub fn new() -> Self {
        Self { inner: ListBillingCreditBalanceTransactionBuilder::new() }
    }
    /// The credit grant for which to fetch credit balance transactions.
    pub fn credit_grant(mut self, credit_grant: impl Into<String>) -> Self {
        self.inner.credit_grant = Some(credit_grant.into());
        self
    }
    /// The customer whose credit balance transactions you're retrieving.
    pub fn customer(mut self, customer: impl Into<String>) -> Self {
        self.inner.customer = Some(customer.into());
        self
    }
    /// The account representing the customer whose credit balance transactions you're retrieving.
    pub fn customer_account(mut self, customer_account: impl Into<String>) -> Self {
        self.inner.customer_account = Some(customer_account.into());
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
impl Default for ListBillingCreditBalanceTransaction {
    fn default() -> Self {
        Self::new()
    }
}
impl ListBillingCreditBalanceTransaction {
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
    ) -> stripe_client_core::ListPaginator<
        stripe_types::List<stripe_shared::BillingCreditBalanceTransaction>,
    > {
        stripe_client_core::ListPaginator::new_list(
            "/billing/credit_balance_transactions",
            &self.inner,
        )
    }
}

impl StripeRequest for ListBillingCreditBalanceTransaction {
    type Output = stripe_types::List<stripe_shared::BillingCreditBalanceTransaction>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/billing/credit_balance_transactions")
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveBillingCreditBalanceTransactionBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveBillingCreditBalanceTransactionBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveBillingCreditBalanceTransactionBuilder").finish_non_exhaustive()
    }
}
impl RetrieveBillingCreditBalanceTransactionBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a credit balance transaction.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveBillingCreditBalanceTransaction {
    inner: RetrieveBillingCreditBalanceTransactionBuilder,
    id: stripe_shared::BillingCreditBalanceTransactionId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveBillingCreditBalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveBillingCreditBalanceTransaction").finish_non_exhaustive()
    }
}
impl RetrieveBillingCreditBalanceTransaction {
    /// Construct a new `RetrieveBillingCreditBalanceTransaction`.
    pub fn new(id: impl Into<stripe_shared::BillingCreditBalanceTransactionId>) -> Self {
        Self { id: id.into(), inner: RetrieveBillingCreditBalanceTransactionBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveBillingCreditBalanceTransaction {
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

impl StripeRequest for RetrieveBillingCreditBalanceTransaction {
    type Output = stripe_shared::BillingCreditBalanceTransaction;

    fn build(&self) -> RequestBuilder {
        let id = &self.id;
        RequestBuilder::new(StripeMethod::Get, format!("/billing/credit_balance_transactions/{id}"))
            .query(&self.inner)
    }
}
