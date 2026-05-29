use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// With <a href="/connect">Connect</a>, you can delete accounts you manage.
///
/// Test-mode accounts can be deleted at any time.
///
/// Live-mode accounts that have access to the standard dashboard and Stripe is responsible for negative account balances cannot be deleted, which includes Standard accounts.
/// All other Live-mode accounts, can be deleted when all <a href="/api/balance/balance_object">balances</a> are zero.
///
/// If you want to delete your own account, use the [account information tab in your account settings](https://dashboard.stripe.com/settings/account) instead.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeleteAccount {
    account: stripe_shared::AccountId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeleteAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeleteAccount").finish_non_exhaustive()
    }
}
impl DeleteAccount {
    /// Construct a new `DeleteAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into() }
    }
}
impl DeleteAccount {
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

impl StripeRequest for DeleteAccount {
    type Output = stripe_shared::DeletedAccount;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Delete, format!("/accounts/{account}"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveForMyAccountAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveForMyAccountAccountBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveForMyAccountAccountBuilder").finish_non_exhaustive()
    }
}
impl RetrieveForMyAccountAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an account.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveForMyAccountAccount {
    inner: RetrieveForMyAccountAccountBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveForMyAccountAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveForMyAccountAccount").finish_non_exhaustive()
    }
}
impl RetrieveForMyAccountAccount {
    /// Construct a new `RetrieveForMyAccountAccount`.
    pub fn new() -> Self {
        Self { inner: RetrieveForMyAccountAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl Default for RetrieveForMyAccountAccount {
    fn default() -> Self {
        Self::new()
    }
}
impl RetrieveForMyAccountAccount {
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

impl StripeRequest for RetrieveForMyAccountAccount {
    type Output = stripe_shared::Account;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/account").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
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
impl std::fmt::Debug for ListAccountBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListAccountBuilder").finish_non_exhaustive()
    }
}
impl ListAccountBuilder {
    fn new() -> Self {
        Self { created: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of accounts connected to your platform via [Connect](https://stripe.com/docs/connect).
/// If you’re not a platform, the list is empty.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListAccount {
    inner: ListAccountBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListAccount").finish_non_exhaustive()
    }
}
impl ListAccount {
    /// Construct a new `ListAccount`.
    pub fn new() -> Self {
        Self { inner: ListAccountBuilder::new() }
    }
    /// Only return connected accounts that were created during the given date interval.
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
}
impl Default for ListAccount {
    fn default() -> Self {
        Self::new()
    }
}
impl ListAccount {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Account>> {
        stripe_client_core::ListPaginator::new_list("/accounts", &self.inner)
    }
}

impl StripeRequest for ListAccount {
    type Output = stripe_types::List<stripe_shared::Account>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/accounts").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveAccountBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveAccountBuilder").finish_non_exhaustive()
    }
}
impl RetrieveAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an account.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveAccount {
    inner: RetrieveAccountBuilder,
    account: stripe_shared::AccountId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveAccount").finish_non_exhaustive()
    }
}
impl RetrieveAccount {
    /// Construct a new `RetrieveAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into(), inner: RetrieveAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveAccount {
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

impl StripeRequest for RetrieveAccount {
    type Output = stripe_shared::Account;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}")).query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CapabilitiesAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CapabilitiesAccountBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CapabilitiesAccountBuilder").finish_non_exhaustive()
    }
}
impl CapabilitiesAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Returns a list of capabilities associated with the account.
/// The capabilities are returned sorted by creation date, with the most recent capability appearing first.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CapabilitiesAccount {
    inner: CapabilitiesAccountBuilder,
    account: stripe_shared::AccountId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CapabilitiesAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CapabilitiesAccount").finish_non_exhaustive()
    }
}
impl CapabilitiesAccount {
    /// Construct a new `CapabilitiesAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into(), inner: CapabilitiesAccountBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CapabilitiesAccount {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Capability>> {
        let account = &self.account;

        stripe_client_core::ListPaginator::new_list(
            format!("/accounts/{account}/capabilities"),
            &self.inner,
        )
    }
}

impl StripeRequest for CapabilitiesAccount {
    type Output = stripe_types::List<stripe_shared::Capability>;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}/capabilities"))
            .query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct PersonsAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relationship: Option<PersonsAccountRelationship>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonsAccountBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonsAccountBuilder").finish_non_exhaustive()
    }
}
impl PersonsAccountBuilder {
    fn new() -> Self {
        Self {
            ending_before: None,
            expand: None,
            limit: None,
            relationship: None,
            starting_after: None,
        }
    }
}
/// Filters on the list of people returned based on the person's relationship to the account's company.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PersonsAccountRelationship {
    /// A filter on the list of people returned based on whether these people are authorizers of the account's representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer: Option<bool>,
    /// A filter on the list of people returned based on whether these people are directors of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// A filter on the list of people returned based on whether these people are executives of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// A filter on the list of people returned based on whether these people are legal guardians of the account's representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_guardian: Option<bool>,
    /// A filter on the list of people returned based on whether these people are owners of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// A filter on the list of people returned based on whether these people are the representative of the account's company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonsAccountRelationship {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonsAccountRelationship").finish_non_exhaustive()
    }
}
impl PersonsAccountRelationship {
    pub fn new() -> Self {
        Self {
            authorizer: None,
            director: None,
            executive: None,
            legal_guardian: None,
            owner: None,
            representative: None,
        }
    }
}
impl Default for PersonsAccountRelationship {
    fn default() -> Self {
        Self::new()
    }
}
/// Returns a list of people associated with the account’s legal entity.
/// The people are returned sorted by creation date, with the most recent people appearing first.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PersonsAccount {
    inner: PersonsAccountBuilder,
    account: stripe_shared::AccountId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonsAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonsAccount").finish_non_exhaustive()
    }
}
impl PersonsAccount {
    /// Construct a new `PersonsAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into(), inner: PersonsAccountBuilder::new() }
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
    /// Filters on the list of people returned based on the person's relationship to the account's company.
    pub fn relationship(mut self, relationship: impl Into<PersonsAccountRelationship>) -> Self {
        self.inner.relationship = Some(relationship.into());
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
impl PersonsAccount {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::Person>> {
        let account = &self.account;

        stripe_client_core::ListPaginator::new_list(
            format!("/accounts/{account}/persons"),
            &self.inner,
        )
    }
}

impl StripeRequest for PersonsAccount {
    type Output = stripe_types::List<stripe_shared::Person>;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Get, format!("/accounts/{account}/persons"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile: Option<CreateAccountBusinessProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_type: Option<stripe_shared::AccountBusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<CreateAccountCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company: Option<CreateAccountCompany>,
    #[serde(skip_serializing_if = "Option::is_none")]
    controller: Option<CreateAccountController>,
    #[serde(skip_serializing_if = "Option::is_none")]
    country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<DocumentsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<AccountGroupsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    individual: Option<CreateAccountIndividual>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<CreateAccountSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tos_acceptance: Option<TosAcceptanceSpecs>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<CreateAccountType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountBuilder").finish_non_exhaustive()
    }
}
impl CreateAccountBuilder {
    fn new() -> Self {
        Self {
            account_token: None,
            business_profile: None,
            business_type: None,
            capabilities: None,
            company: None,
            controller: None,
            country: None,
            default_currency: None,
            documents: None,
            email: None,
            expand: None,
            external_account: None,
            groups: None,
            individual: None,
            metadata: None,
            settings: None,
            tos_acceptance: None,
            type_: None,
        }
    }
}
/// Business information about the account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountBusinessProfile {
    /// The applicant's gross annual revenue for its preceding fiscal year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<AnnualRevenueSpecs>,
    /// An estimated upper bound of employees, contractors, vendors, etc.
    /// currently working for the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_worker_count: Option<u64>,
    /// [The merchant category code for the account](/connect/setting-mcc).
    /// MCCs are used to classify businesses based on the goods or services they provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    /// Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minority_owned_business_designation:
        Option<Vec<CreateAccountBusinessProfileMinorityOwnedBusinessDesignation>>,
    /// An estimate of the monthly revenue of the business. Only accepted for accounts in Brazil and India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_estimated_revenue: Option<MonthlyEstimatedRevenueSpecs>,
    /// The customer-facing business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Internal-only description of the product sold by, or service provided by, the business.
    /// Used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// A publicly available mailing address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_address: Option<CreateAccountBusinessProfileSupportAddress>,
    /// A publicly available email address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// A publicly available phone number to call with support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<String>,
    /// A publicly available website for handling support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    /// The business's publicly available website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountBusinessProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountBusinessProfile").finish_non_exhaustive()
    }
}
impl CreateAccountBusinessProfile {
    pub fn new() -> Self {
        Self {
            annual_revenue: None,
            estimated_worker_count: None,
            mcc: None,
            minority_owned_business_designation: None,
            monthly_estimated_revenue: None,
            name: None,
            product_description: None,
            support_address: None,
            support_email: None,
            support_phone: None,
            support_url: None,
            url: None,
        }
    }
}
impl Default for CreateAccountBusinessProfile {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    LgbtqiOwnedBusiness,
    MinorityOwnedBusiness,
    NoneOfTheseApply,
    PreferNotToAnswer,
    WomenOwnedBusiness,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    pub fn as_str(&self) -> &str {
        use CreateAccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match self {
            LgbtqiOwnedBusiness => "lgbtqi_owned_business",
            MinorityOwnedBusiness => "minority_owned_business",
            NoneOfTheseApply => "none_of_these_apply",
            PreferNotToAnswer => "prefer_not_to_answer",
            WomenOwnedBusiness => "women_owned_business",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match s {
            "lgbtqi_owned_business" => Ok(LgbtqiOwnedBusiness),
            "minority_owned_business" => Ok(MinorityOwnedBusiness),
            "none_of_these_apply" => Ok(NoneOfTheseApply),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "women_owned_business" => Ok(WomenOwnedBusiness),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountBusinessProfileMinorityOwnedBusinessDesignation"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountBusinessProfileMinorityOwnedBusinessDesignation))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// A publicly available mailing address for sending support issues to.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountBusinessProfileSupportAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountBusinessProfileSupportAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountBusinessProfileSupportAddress").finish_non_exhaustive()
    }
}
impl CreateAccountBusinessProfileSupportAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateAccountBusinessProfileSupportAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Each key of the dictionary represents a capability, and each capability
/// maps to its settings (for example, whether it has been requested or not). Each
/// capability is inactive until you have provided its specific
/// requirements and Stripe has verified them. An account might have some
/// of its requested capabilities be active and some be inactive.
///
/// Required when [account.controller.stripe_dashboard.type](/api/accounts/create#create_account-controller-dashboard-type).
/// is `none`, which includes Custom accounts.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilities {
    /// The acss_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<CreateAccountCapabilitiesAcssDebitPayments>,
    /// The affirm_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<CreateAccountCapabilitiesAffirmPayments>,
    /// The afterpay_clearpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<CreateAccountCapabilitiesAfterpayClearpayPayments>,
    /// The alma_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma_payments: Option<CreateAccountCapabilitiesAlmaPayments>,
    /// The amazon_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay_payments: Option<CreateAccountCapabilitiesAmazonPayPayments>,
    /// The app_distribution capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_distribution: Option<CreateAccountCapabilitiesAppDistribution>,
    /// The au_becs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<CreateAccountCapabilitiesAuBecsDebitPayments>,
    /// The bacs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<CreateAccountCapabilitiesBacsDebitPayments>,
    /// The bancontact_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<CreateAccountCapabilitiesBancontactPayments>,
    /// The bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<CreateAccountCapabilitiesBankTransferPayments>,
    /// The billie_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie_payments: Option<CreateAccountCapabilitiesBilliePayments>,
    /// The bizum_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bizum_payments: Option<CreateAccountCapabilitiesBizumPayments>,
    /// The blik_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<CreateAccountCapabilitiesBlikPayments>,
    /// The boleto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<CreateAccountCapabilitiesBoletoPayments>,
    /// The card_issuing capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CreateAccountCapabilitiesCardIssuing>,
    /// The card_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CreateAccountCapabilitiesCardPayments>,
    /// The cartes_bancaires_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<CreateAccountCapabilitiesCartesBancairesPayments>,
    /// The cashapp_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<CreateAccountCapabilitiesCashappPayments>,
    /// The crypto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_payments: Option<CreateAccountCapabilitiesCryptoPayments>,
    /// The eps_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<CreateAccountCapabilitiesEpsPayments>,
    /// The fpx_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<CreateAccountCapabilitiesFpxPayments>,
    /// The gb_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer_payments: Option<CreateAccountCapabilitiesGbBankTransferPayments>,
    /// The giropay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<CreateAccountCapabilitiesGiropayPayments>,
    /// The grabpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<CreateAccountCapabilitiesGrabpayPayments>,
    /// The ideal_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<CreateAccountCapabilitiesIdealPayments>,
    /// The india_international_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<CreateAccountCapabilitiesIndiaInternationalPayments>,
    /// The jcb_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<CreateAccountCapabilitiesJcbPayments>,
    /// The jp_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer_payments: Option<CreateAccountCapabilitiesJpBankTransferPayments>,
    /// The kakao_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kakao_pay_payments: Option<CreateAccountCapabilitiesKakaoPayPayments>,
    /// The klarna_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<CreateAccountCapabilitiesKlarnaPayments>,
    /// The konbini_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<CreateAccountCapabilitiesKonbiniPayments>,
    /// The kr_card_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_card_payments: Option<CreateAccountCapabilitiesKrCardPayments>,
    /// The legacy_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<CreateAccountCapabilitiesLegacyPayments>,
    /// The link_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<CreateAccountCapabilitiesLinkPayments>,
    /// The mb_way_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mb_way_payments: Option<CreateAccountCapabilitiesMbWayPayments>,
    /// The mobilepay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay_payments: Option<CreateAccountCapabilitiesMobilepayPayments>,
    /// The multibanco_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco_payments: Option<CreateAccountCapabilitiesMultibancoPayments>,
    /// The mx_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer_payments: Option<CreateAccountCapabilitiesMxBankTransferPayments>,
    /// The naver_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay_payments: Option<CreateAccountCapabilitiesNaverPayPayments>,
    /// The nz_bank_account_becs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account_becs_debit_payments:
        Option<CreateAccountCapabilitiesNzBankAccountBecsDebitPayments>,
    /// The oxxo_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<CreateAccountCapabilitiesOxxoPayments>,
    /// The p24_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<CreateAccountCapabilitiesP24Payments>,
    /// The pay_by_bank_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank_payments: Option<CreateAccountCapabilitiesPayByBankPayments>,
    /// The payco_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payco_payments: Option<CreateAccountCapabilitiesPaycoPayments>,
    /// The paynow_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<CreateAccountCapabilitiesPaynowPayments>,
    /// The payto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto_payments: Option<CreateAccountCapabilitiesPaytoPayments>,
    /// The pix_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix_payments: Option<CreateAccountCapabilitiesPixPayments>,
    /// The promptpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<CreateAccountCapabilitiesPromptpayPayments>,
    /// The revolut_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments: Option<CreateAccountCapabilitiesRevolutPayPayments>,
    /// The samsung_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay_payments: Option<CreateAccountCapabilitiesSamsungPayPayments>,
    /// The satispay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay_payments: Option<CreateAccountCapabilitiesSatispayPayments>,
    /// The scalapay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalapay_payments: Option<CreateAccountCapabilitiesScalapayPayments>,
    /// The sepa_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_bank_transfer_payments: Option<CreateAccountCapabilitiesSepaBankTransferPayments>,
    /// The sepa_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<CreateAccountCapabilitiesSepaDebitPayments>,
    /// The sofort_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<CreateAccountCapabilitiesSofortPayments>,
    /// The sunbit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunbit_payments: Option<CreateAccountCapabilitiesSunbitPayments>,
    /// The swish_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish_payments: Option<CreateAccountCapabilitiesSwishPayments>,
    /// The tax_reporting_us_1099_k capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<CreateAccountCapabilitiesTaxReportingUs1099K>,
    /// The tax_reporting_us_1099_misc capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<CreateAccountCapabilitiesTaxReportingUs1099Misc>,
    /// The transfers capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<CreateAccountCapabilitiesTransfers>,
    /// The treasury capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<CreateAccountCapabilitiesTreasury>,
    /// The twint_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint_payments: Option<CreateAccountCapabilitiesTwintPayments>,
    /// The upi_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi_payments: Option<CreateAccountCapabilitiesUpiPayments>,
    /// The us_bank_account_ach_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<CreateAccountCapabilitiesUsBankAccountAchPayments>,
    /// The us_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer_payments: Option<CreateAccountCapabilitiesUsBankTransferPayments>,
    /// The zip_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<CreateAccountCapabilitiesZipPayments>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilities").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilities {
    pub fn new() -> Self {
        Self {
            acss_debit_payments: None,
            affirm_payments: None,
            afterpay_clearpay_payments: None,
            alma_payments: None,
            amazon_pay_payments: None,
            app_distribution: None,
            au_becs_debit_payments: None,
            bacs_debit_payments: None,
            bancontact_payments: None,
            bank_transfer_payments: None,
            billie_payments: None,
            bizum_payments: None,
            blik_payments: None,
            boleto_payments: None,
            card_issuing: None,
            card_payments: None,
            cartes_bancaires_payments: None,
            cashapp_payments: None,
            crypto_payments: None,
            eps_payments: None,
            fpx_payments: None,
            gb_bank_transfer_payments: None,
            giropay_payments: None,
            grabpay_payments: None,
            ideal_payments: None,
            india_international_payments: None,
            jcb_payments: None,
            jp_bank_transfer_payments: None,
            kakao_pay_payments: None,
            klarna_payments: None,
            konbini_payments: None,
            kr_card_payments: None,
            legacy_payments: None,
            link_payments: None,
            mb_way_payments: None,
            mobilepay_payments: None,
            multibanco_payments: None,
            mx_bank_transfer_payments: None,
            naver_pay_payments: None,
            nz_bank_account_becs_debit_payments: None,
            oxxo_payments: None,
            p24_payments: None,
            pay_by_bank_payments: None,
            payco_payments: None,
            paynow_payments: None,
            payto_payments: None,
            pix_payments: None,
            promptpay_payments: None,
            revolut_pay_payments: None,
            samsung_pay_payments: None,
            satispay_payments: None,
            scalapay_payments: None,
            sepa_bank_transfer_payments: None,
            sepa_debit_payments: None,
            sofort_payments: None,
            sunbit_payments: None,
            swish_payments: None,
            tax_reporting_us_1099_k: None,
            tax_reporting_us_1099_misc: None,
            transfers: None,
            treasury: None,
            twint_payments: None,
            upi_payments: None,
            us_bank_account_ach_payments: None,
            us_bank_transfer_payments: None,
            zip_payments: None,
        }
    }
}
impl Default for CreateAccountCapabilities {
    fn default() -> Self {
        Self::new()
    }
}
/// The acss_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesAcssDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesAcssDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesAcssDebitPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesAcssDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesAcssDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The affirm_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesAffirmPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesAffirmPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesAffirmPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesAffirmPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesAffirmPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The afterpay_clearpay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesAfterpayClearpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesAfterpayClearpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesAfterpayClearpayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesAfterpayClearpayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesAfterpayClearpayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The alma_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesAlmaPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesAlmaPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesAlmaPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesAlmaPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesAlmaPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The amazon_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesAmazonPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesAmazonPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesAmazonPayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesAmazonPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesAmazonPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The app_distribution capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesAppDistribution {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesAppDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesAppDistribution").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesAppDistribution {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesAppDistribution {
    fn default() -> Self {
        Self::new()
    }
}
/// The au_becs_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesAuBecsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesAuBecsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesAuBecsDebitPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesAuBecsDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesAuBecsDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The bacs_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesBacsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesBacsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesBacsDebitPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesBacsDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesBacsDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The bancontact_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesBancontactPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesBancontactPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesBancontactPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesBancontactPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesBancontactPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesBankTransferPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The billie_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesBilliePayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesBilliePayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesBilliePayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesBilliePayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesBilliePayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The bizum_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesBizumPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesBizumPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesBizumPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesBizumPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesBizumPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The blik_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesBlikPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesBlikPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesBlikPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesBlikPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesBlikPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The boleto_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesBoletoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesBoletoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesBoletoPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesBoletoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesBoletoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The card_issuing capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesCardIssuing {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesCardIssuing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesCardIssuing").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesCardIssuing {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesCardIssuing {
    fn default() -> Self {
        Self::new()
    }
}
/// The card_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesCardPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesCardPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesCardPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesCardPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesCardPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The cartes_bancaires_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesCartesBancairesPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesCartesBancairesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesCartesBancairesPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesCartesBancairesPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesCartesBancairesPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The cashapp_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesCashappPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesCashappPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesCashappPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesCashappPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesCashappPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The crypto_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesCryptoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesCryptoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesCryptoPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesCryptoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesCryptoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The eps_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesEpsPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesEpsPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesEpsPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesEpsPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesEpsPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The fpx_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesFpxPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesFpxPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesFpxPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesFpxPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesFpxPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The gb_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesGbBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesGbBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesGbBankTransferPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesGbBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesGbBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The giropay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesGiropayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesGiropayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesGiropayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesGiropayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesGiropayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The grabpay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesGrabpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesGrabpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesGrabpayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesGrabpayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesGrabpayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The ideal_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesIdealPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesIdealPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesIdealPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesIdealPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesIdealPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The india_international_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesIndiaInternationalPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesIndiaInternationalPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesIndiaInternationalPayments")
            .finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesIndiaInternationalPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesIndiaInternationalPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The jcb_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesJcbPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesJcbPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesJcbPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesJcbPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesJcbPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The jp_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesJpBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesJpBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesJpBankTransferPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesJpBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesJpBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The kakao_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesKakaoPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesKakaoPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesKakaoPayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesKakaoPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesKakaoPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The klarna_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesKlarnaPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesKlarnaPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesKlarnaPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesKlarnaPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesKlarnaPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The konbini_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesKonbiniPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesKonbiniPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesKonbiniPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesKonbiniPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesKonbiniPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The kr_card_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesKrCardPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesKrCardPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesKrCardPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesKrCardPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesKrCardPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The legacy_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesLegacyPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesLegacyPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesLegacyPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesLegacyPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesLegacyPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The link_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesLinkPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesLinkPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesLinkPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesLinkPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesLinkPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The mb_way_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesMbWayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesMbWayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesMbWayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesMbWayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesMbWayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The mobilepay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesMobilepayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesMobilepayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesMobilepayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesMobilepayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesMobilepayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The multibanco_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesMultibancoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesMultibancoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesMultibancoPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesMultibancoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesMultibancoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The mx_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesMxBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesMxBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesMxBankTransferPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesMxBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesMxBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The naver_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesNaverPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesNaverPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesNaverPayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesNaverPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesNaverPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The nz_bank_account_becs_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesNzBankAccountBecsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesNzBankAccountBecsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesNzBankAccountBecsDebitPayments")
            .finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesNzBankAccountBecsDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesNzBankAccountBecsDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The oxxo_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesOxxoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesOxxoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesOxxoPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesOxxoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesOxxoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The p24_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesP24Payments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesP24Payments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesP24Payments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesP24Payments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesP24Payments {
    fn default() -> Self {
        Self::new()
    }
}
/// The pay_by_bank_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesPayByBankPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesPayByBankPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesPayByBankPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesPayByBankPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesPayByBankPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The payco_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesPaycoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesPaycoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesPaycoPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesPaycoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesPaycoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The paynow_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesPaynowPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesPaynowPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesPaynowPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesPaynowPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesPaynowPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The payto_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesPaytoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesPaytoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesPaytoPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesPaytoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesPaytoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The pix_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesPixPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesPixPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesPixPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesPixPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesPixPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The promptpay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesPromptpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesPromptpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesPromptpayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesPromptpayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesPromptpayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The revolut_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesRevolutPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesRevolutPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesRevolutPayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesRevolutPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesRevolutPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The samsung_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesSamsungPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesSamsungPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesSamsungPayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesSamsungPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesSamsungPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The satispay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesSatispayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesSatispayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesSatispayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesSatispayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesSatispayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The scalapay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesScalapayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesScalapayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesScalapayPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesScalapayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesScalapayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The sepa_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesSepaBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesSepaBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesSepaBankTransferPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesSepaBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesSepaBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The sepa_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesSepaDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesSepaDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesSepaDebitPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesSepaDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesSepaDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The sofort_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesSofortPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesSofortPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesSofortPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesSofortPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesSofortPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The sunbit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesSunbitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesSunbitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesSunbitPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesSunbitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesSunbitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The swish_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesSwishPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesSwishPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesSwishPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesSwishPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesSwishPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The tax_reporting_us_1099_k capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099K {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesTaxReportingUs1099K {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesTaxReportingUs1099K").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesTaxReportingUs1099K {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesTaxReportingUs1099K {
    fn default() -> Self {
        Self::new()
    }
}
/// The tax_reporting_us_1099_misc capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesTaxReportingUs1099Misc {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesTaxReportingUs1099Misc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesTaxReportingUs1099Misc").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesTaxReportingUs1099Misc {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesTaxReportingUs1099Misc {
    fn default() -> Self {
        Self::new()
    }
}
/// The transfers capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesTransfers {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesTransfers").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesTransfers {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesTransfers {
    fn default() -> Self {
        Self::new()
    }
}
/// The treasury capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesTreasury {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesTreasury").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesTreasury {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesTreasury {
    fn default() -> Self {
        Self::new()
    }
}
/// The twint_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesTwintPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesTwintPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesTwintPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesTwintPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesTwintPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The upi_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesUpiPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesUpiPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesUpiPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesUpiPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesUpiPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The us_bank_account_ach_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesUsBankAccountAchPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesUsBankAccountAchPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesUsBankAccountAchPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesUsBankAccountAchPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesUsBankAccountAchPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The us_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesUsBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesUsBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesUsBankTransferPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesUsBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesUsBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The zip_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCapabilitiesZipPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCapabilitiesZipPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCapabilitiesZipPayments").finish_non_exhaustive()
    }
}
impl CreateAccountCapabilitiesZipPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CreateAccountCapabilitiesZipPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about the company or business.
/// This field is available for any `business_type`.
/// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCompany {
    /// The company's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateAccountCompanyAddress>,
    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateAccountCompanyAddressKana>,
    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateAccountCompanyAddressKanji>,
    /// Whether the company's directors have been provided.
    /// Set this Boolean to `true` after creating all the company's directors with [the Persons API](/api/persons) for accounts with a `relationship.director` requirement.
    /// This value is not automatically set to `true` after creating directors, so it needs to be updated to indicate all directors have been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,
    /// This hash is used to attest that the directors information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directorship_declaration: Option<CompanyDirectorshipDeclaration>,
    /// Whether the company's executives have been provided.
    /// Set this Boolean to `true` after creating all the company's executives with [the Persons API](/api/persons) for accounts with a `relationship.executive` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,
    /// The export license ID number of the company, also referred as Import Export Code (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_license_id: Option<String>,
    /// The purpose code to use for export transactions (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_purpose_code: Option<String>,
    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,
    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,
    /// Whether the company's owners have been provided.
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<CompanyOwnershipDeclaration>,
    /// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
    /// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_exemption_reason: Option<CreateAccountCompanyOwnershipExemptionReason>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// When the business was incorporated or registered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<RegistrationDateSpecs>,
    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// This hash is used to attest that the representative is authorized to act as the representative of their legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative_declaration: Option<CompanyRepresentativeDeclaration>,
    /// The category identifying the legal structure of the company or legal entity.
    /// See [Business structure](/connect/identity-verification#business-structure) for more details.
    /// Pass an empty string to unset this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<CreateAccountCompanyStructure>,
    /// The business ID number of the company, as appropriate for the company’s country.
    /// (Examples are an Employer ID Number in the U.S., a Business Number in Canada, or a Company Number in the UK.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,
    /// The VAT number of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<VerificationSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCompany").finish_non_exhaustive()
    }
}
impl CreateAccountCompany {
    pub fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            directors_provided: None,
            directorship_declaration: None,
            executives_provided: None,
            export_license_id: None,
            export_purpose_code: None,
            name: None,
            name_kana: None,
            name_kanji: None,
            owners_provided: None,
            ownership_declaration: None,
            ownership_exemption_reason: None,
            phone: None,
            registration_date: None,
            registration_number: None,
            representative_declaration: None,
            structure: None,
            tax_id: None,
            tax_id_registrar: None,
            vat_id: None,
            verification: None,
        }
    }
}
impl Default for CreateAccountCompany {
    fn default() -> Self {
        Self::new()
    }
}
/// The company's primary address.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCompanyAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCompanyAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCompanyAddress").finish_non_exhaustive()
    }
}
impl CreateAccountCompanyAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateAccountCompanyAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the company's primary address (Japan only).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCompanyAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCompanyAddressKana {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCompanyAddressKana").finish_non_exhaustive()
    }
}
impl CreateAccountCompanyAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateAccountCompanyAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the company's primary address (Japan only).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountCompanyAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCompanyAddressKanji {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountCompanyAddressKanji").finish_non_exhaustive()
    }
}
impl CreateAccountCompanyAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateAccountCompanyAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
/// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountCompanyOwnershipExemptionReason {
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountCompanyOwnershipExemptionReason {
    pub fn as_str(&self) -> &str {
        use CreateAccountCompanyOwnershipExemptionReason::*;
        match self {
            QualifiedEntityExceedsOwnershipThreshold => {
                "qualified_entity_exceeds_ownership_threshold"
            }
            QualifiesAsFinancialInstitution => "qualifies_as_financial_institution",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountCompanyOwnershipExemptionReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountCompanyOwnershipExemptionReason::*;
        match s {
            "qualified_entity_exceeds_ownership_threshold" => {
                Ok(QualifiedEntityExceedsOwnershipThreshold)
            }
            "qualifies_as_financial_institution" => Ok(QualifiesAsFinancialInstitution),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountCompanyOwnershipExemptionReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountCompanyOwnershipExemptionReason))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountCompanyOwnershipExemptionReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountCompanyOwnershipExemptionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The category identifying the legal structure of the company or legal entity.
/// See [Business structure](/connect/identity-verification#business-structure) for more details.
/// Pass an empty string to unset this value.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountCompanyStructure {
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    IncorporatedPartnership,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    RegisteredCharity,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
    UnincorporatedPartnership,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountCompanyStructure {
    pub fn as_str(&self) -> &str {
        use CreateAccountCompanyStructure::*;
        match self {
            FreeZoneEstablishment => "free_zone_establishment",
            FreeZoneLlc => "free_zone_llc",
            GovernmentInstrumentality => "government_instrumentality",
            GovernmentalUnit => "governmental_unit",
            IncorporatedNonProfit => "incorporated_non_profit",
            IncorporatedPartnership => "incorporated_partnership",
            LimitedLiabilityPartnership => "limited_liability_partnership",
            Llc => "llc",
            MultiMemberLlc => "multi_member_llc",
            PrivateCompany => "private_company",
            PrivateCorporation => "private_corporation",
            PrivatePartnership => "private_partnership",
            PublicCompany => "public_company",
            PublicCorporation => "public_corporation",
            PublicPartnership => "public_partnership",
            RegisteredCharity => "registered_charity",
            SingleMemberLlc => "single_member_llc",
            SoleEstablishment => "sole_establishment",
            SoleProprietorship => "sole_proprietorship",
            TaxExemptGovernmentInstrumentality => "tax_exempt_government_instrumentality",
            UnincorporatedAssociation => "unincorporated_association",
            UnincorporatedNonProfit => "unincorporated_non_profit",
            UnincorporatedPartnership => "unincorporated_partnership",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountCompanyStructure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountCompanyStructure::*;
        match s {
            "free_zone_establishment" => Ok(FreeZoneEstablishment),
            "free_zone_llc" => Ok(FreeZoneLlc),
            "government_instrumentality" => Ok(GovernmentInstrumentality),
            "governmental_unit" => Ok(GovernmentalUnit),
            "incorporated_non_profit" => Ok(IncorporatedNonProfit),
            "incorporated_partnership" => Ok(IncorporatedPartnership),
            "limited_liability_partnership" => Ok(LimitedLiabilityPartnership),
            "llc" => Ok(Llc),
            "multi_member_llc" => Ok(MultiMemberLlc),
            "private_company" => Ok(PrivateCompany),
            "private_corporation" => Ok(PrivateCorporation),
            "private_partnership" => Ok(PrivatePartnership),
            "public_company" => Ok(PublicCompany),
            "public_corporation" => Ok(PublicCorporation),
            "public_partnership" => Ok(PublicPartnership),
            "registered_charity" => Ok(RegisteredCharity),
            "single_member_llc" => Ok(SingleMemberLlc),
            "sole_establishment" => Ok(SoleEstablishment),
            "sole_proprietorship" => Ok(SoleProprietorship),
            "tax_exempt_government_instrumentality" => Ok(TaxExemptGovernmentInstrumentality),
            "unincorporated_association" => Ok(UnincorporatedAssociation),
            "unincorporated_non_profit" => Ok(UnincorporatedNonProfit),
            "unincorporated_partnership" => Ok(UnincorporatedPartnership),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountCompanyStructure"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountCompanyStructure)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountCompanyStructure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountCompanyStructure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// A hash of configuration describing the account controller's attributes.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountController {
    /// A hash of configuration for who pays Stripe fees for product usage on this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<CreateAccountControllerFees>,
    /// A hash of configuration for products that have negative balance liability, and whether Stripe or a Connect application is responsible for them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub losses: Option<CreateAccountControllerLosses>,
    /// A value indicating responsibility for collecting updated information when requirements on the account are due or change.
    /// Defaults to `stripe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirement_collection: Option<CreateAccountControllerRequirementCollection>,
    /// A hash of configuration for Stripe-hosted dashboards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_dashboard: Option<CreateAccountControllerStripeDashboard>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountController {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountController").finish_non_exhaustive()
    }
}
impl CreateAccountController {
    pub fn new() -> Self {
        Self { fees: None, losses: None, requirement_collection: None, stripe_dashboard: None }
    }
}
impl Default for CreateAccountController {
    fn default() -> Self {
        Self::new()
    }
}
/// A hash of configuration for who pays Stripe fees for product usage on this account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountControllerFees {
    /// A value indicating the responsible payer of Stripe fees on this account.
    /// Defaults to `account`.
    /// Learn more about [fee behavior on connected accounts](https://docs.stripe.com/connect/direct-charges-fee-payer-behavior).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<CreateAccountControllerFeesPayer>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountControllerFees {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountControllerFees").finish_non_exhaustive()
    }
}
impl CreateAccountControllerFees {
    pub fn new() -> Self {
        Self { payer: None }
    }
}
impl Default for CreateAccountControllerFees {
    fn default() -> Self {
        Self::new()
    }
}
/// A value indicating the responsible payer of Stripe fees on this account.
/// Defaults to `account`.
/// Learn more about [fee behavior on connected accounts](https://docs.stripe.com/connect/direct-charges-fee-payer-behavior).
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountControllerFeesPayer {
    Account,
    Application,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountControllerFeesPayer {
    pub fn as_str(&self) -> &str {
        use CreateAccountControllerFeesPayer::*;
        match self {
            Account => "account",
            Application => "application",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountControllerFeesPayer {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountControllerFeesPayer::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountControllerFeesPayer"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountControllerFeesPayer)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountControllerFeesPayer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountControllerFeesPayer {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// A hash of configuration for products that have negative balance liability, and whether Stripe or a Connect application is responsible for them.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountControllerLosses {
    /// A value indicating who is liable when this account can't pay back negative balances resulting from payments.
    /// Defaults to `stripe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<CreateAccountControllerLossesPayments>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountControllerLosses {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountControllerLosses").finish_non_exhaustive()
    }
}
impl CreateAccountControllerLosses {
    pub fn new() -> Self {
        Self { payments: None }
    }
}
impl Default for CreateAccountControllerLosses {
    fn default() -> Self {
        Self::new()
    }
}
/// A value indicating who is liable when this account can't pay back negative balances resulting from payments.
/// Defaults to `stripe`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountControllerLossesPayments {
    Application,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountControllerLossesPayments {
    pub fn as_str(&self) -> &str {
        use CreateAccountControllerLossesPayments::*;
        match self {
            Application => "application",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountControllerLossesPayments {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountControllerLossesPayments::*;
        match s {
            "application" => Ok(Application),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountControllerLossesPayments"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountControllerLossesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountControllerLossesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountControllerLossesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountControllerLossesPayments)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountControllerLossesPayments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountControllerLossesPayments {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// A value indicating responsibility for collecting updated information when requirements on the account are due or change.
/// Defaults to `stripe`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountControllerRequirementCollection {
    Application,
    Stripe,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountControllerRequirementCollection {
    pub fn as_str(&self) -> &str {
        use CreateAccountControllerRequirementCollection::*;
        match self {
            Application => "application",
            Stripe => "stripe",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountControllerRequirementCollection {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountControllerRequirementCollection::*;
        match s {
            "application" => Ok(Application),
            "stripe" => Ok(Stripe),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountControllerRequirementCollection"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountControllerRequirementCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountControllerRequirementCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountControllerRequirementCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountControllerRequirementCollection))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountControllerRequirementCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountControllerRequirementCollection {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// A hash of configuration for Stripe-hosted dashboards.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountControllerStripeDashboard {
    /// Whether this account should have access to the full Stripe Dashboard (`full`), to the Express Dashboard (`express`), or to no Stripe-hosted dashboard (`none`).
    /// Defaults to `full`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateAccountControllerStripeDashboardType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountControllerStripeDashboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountControllerStripeDashboard").finish_non_exhaustive()
    }
}
impl CreateAccountControllerStripeDashboard {
    pub fn new() -> Self {
        Self { type_: None }
    }
}
impl Default for CreateAccountControllerStripeDashboard {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether this account should have access to the full Stripe Dashboard (`full`), to the Express Dashboard (`express`), or to no Stripe-hosted dashboard (`none`).
/// Defaults to `full`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountControllerStripeDashboardType {
    Express,
    Full,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountControllerStripeDashboardType {
    pub fn as_str(&self) -> &str {
        use CreateAccountControllerStripeDashboardType::*;
        match self {
            Express => "express",
            Full => "full",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountControllerStripeDashboardType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountControllerStripeDashboardType::*;
        match s {
            "express" => Ok(Express),
            "full" => Ok(Full),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountControllerStripeDashboardType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountControllerStripeDashboardType))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountControllerStripeDashboardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountControllerStripeDashboardType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information about the person represented by the account.
/// This field is null unless `business_type` is set to `individual`.
/// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountIndividual {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateAccountIndividualAddress>,
    /// The Kana variation of the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<CreateAccountIndividualAddressKana>,
    /// The Kanji variation of the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<CreateAccountIndividualAddressKanji>,
    /// The individual's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirthSpecs>,
    /// The individual's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The individual's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The Kana variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,
    /// The Kanji variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,
    /// A list of alternate names or aliases that the individual is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,
    /// The individual's gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// The government-issued ID number of the individual, as appropriate for the representative's country.
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<String>,
    /// The individual's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The Kana variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,
    /// The Kanji variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,
    /// The individual's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The individual's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<CreateAccountIndividualPoliticalExposure>,
    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<CreateAccountIndividualRegisteredAddress>,
    /// Describes the person’s relationship to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<IndividualRelationshipSpecs>,
    /// The last four digits of the individual's Social Security Number (U.S. only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,
    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountIndividual").finish_non_exhaustive()
    }
}
impl CreateAccountIndividual {
    pub fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            dob: None,
            email: None,
            first_name: None,
            first_name_kana: None,
            first_name_kanji: None,
            full_name_aliases: None,
            gender: None,
            id_number: None,
            id_number_secondary: None,
            last_name: None,
            last_name_kana: None,
            last_name_kanji: None,
            maiden_name: None,
            metadata: None,
            phone: None,
            political_exposure: None,
            registered_address: None,
            relationship: None,
            ssn_last_4: None,
            verification: None,
        }
    }
}
impl Default for CreateAccountIndividual {
    fn default() -> Self {
        Self::new()
    }
}
/// The individual's primary address.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountIndividualAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountIndividualAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountIndividualAddress").finish_non_exhaustive()
    }
}
impl CreateAccountIndividualAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateAccountIndividualAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the individual's primary address (Japan only).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountIndividualAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountIndividualAddressKana {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountIndividualAddressKana").finish_non_exhaustive()
    }
}
impl CreateAccountIndividualAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateAccountIndividualAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the individual's primary address (Japan only).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountIndividualAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountIndividualAddressKanji {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountIndividualAddressKanji").finish_non_exhaustive()
    }
}
impl CreateAccountIndividualAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for CreateAccountIndividualAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountIndividualPoliticalExposure {
    Existing,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountIndividualPoliticalExposure {
    pub fn as_str(&self) -> &str {
        use CreateAccountIndividualPoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountIndividualPoliticalExposure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountIndividualPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountIndividualPoliticalExposure"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountIndividualPoliticalExposure)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountIndividualPoliticalExposure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountIndividualPoliticalExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The individual's registered address.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountIndividualRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountIndividualRegisteredAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountIndividualRegisteredAddress").finish_non_exhaustive()
    }
}
impl CreateAccountIndividualRegisteredAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for CreateAccountIndividualRegisteredAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Options for customizing how the account functions within Stripe.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountSettings {
    /// Settings specific to Bacs Direct Debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<BacsDebitPaymentsSpecs>,
    /// Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<BrandingSettingsSpecs>,
    /// Settings specific to the account's use of the Card Issuing product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CardIssuingSettingsSpecs>,
    /// Settings specific to card charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CardPaymentsSettingsSpecs>,
    /// Settings specific to the account’s use of Invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<CreateAccountSettingsInvoices>,
    /// Settings that apply across payment methods for charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<PaymentsSettingsSpecs>,
    /// Settings specific to the account's payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<CreateAccountSettingsPayouts>,
    /// Settings specific to the account's Treasury FinancialAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<TreasurySettingsSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountSettings").finish_non_exhaustive()
    }
}
impl CreateAccountSettings {
    pub fn new() -> Self {
        Self {
            bacs_debit_payments: None,
            branding: None,
            card_issuing: None,
            card_payments: None,
            invoices: None,
            payments: None,
            payouts: None,
            treasury: None,
        }
    }
}
impl Default for CreateAccountSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Settings specific to the account’s use of Invoices.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountSettingsInvoices {
    /// Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when the customer already has a default payment method on the hosted invoice page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_payment_method_save: Option<CreateAccountSettingsInvoicesHostedPaymentMethodSave>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountSettingsInvoices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountSettingsInvoices").finish_non_exhaustive()
    }
}
impl CreateAccountSettingsInvoices {
    pub fn new() -> Self {
        Self { hosted_payment_method_save: None }
    }
}
impl Default for CreateAccountSettingsInvoices {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when the customer already has a default payment method on the hosted invoice page.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    Always,
    Never,
    Offer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    pub fn as_str(&self) -> &str {
        use CreateAccountSettingsInvoicesHostedPaymentMethodSave::*;
        match self {
            Always => "always",
            Never => "never",
            Offer => "offer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountSettingsInvoicesHostedPaymentMethodSave::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            "offer" => Ok(Offer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountSettingsInvoicesHostedPaymentMethodSave"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountSettingsInvoicesHostedPaymentMethodSave))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Settings specific to the account's payouts.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountSettingsPayouts {
    /// A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account.
    /// For details, see [Understanding Connect Account Balances](/connect/account-balances).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,
    /// Details on when funds from charges are available, and when they are paid out to an external account.
    /// For details, see our [Setting Bank and Debit Card Payouts](/connect/bank-transfers#payout-information) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<CreateAccountSettingsPayoutsSchedule>,
    /// The text that appears on the bank account statement for payouts.
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountSettingsPayouts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountSettingsPayouts").finish_non_exhaustive()
    }
}
impl CreateAccountSettingsPayouts {
    pub fn new() -> Self {
        Self { debit_negative_balances: None, schedule: None, statement_descriptor: None }
    }
}
impl Default for CreateAccountSettingsPayouts {
    fn default() -> Self {
        Self::new()
    }
}
/// Details on when funds from charges are available, and when they are paid out to an external account.
/// For details, see our [Setting Bank and Debit Card Payouts](/connect/bank-transfers#payout-information) documentation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccountSettingsPayoutsSchedule {
    /// The number of days charge funds are held before being paid out.
    /// May also be set to `minimum`, representing the lowest available value for the account country.
    /// Default is `minimum`.
    /// The `delay_days` parameter remains at the last configured value if `interval` is `manual`.
    /// [Learn more about controlling payout delay days](/connect/manage-payout-schedule).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<CreateAccountSettingsPayoutsScheduleDelayDays>,
    /// How frequently available funds are paid out.
    /// One of: `daily`, `manual`, `weekly`, or `monthly`.
    /// Default is `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<CreateAccountSettingsPayoutsScheduleInterval>,
    /// The day of the month when available funds are paid out, specified as a number between 1--31.
    /// Payouts nominally scheduled between the 29th and 31st of the month are instead sent on the last day of a shorter month.
    /// Required and applicable only if `interval` is `monthly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u8>,
    /// The days of the month when available funds are paid out, specified as an array of numbers between 1--31.
    /// Payouts nominally scheduled between the 29th and 31st of the month are instead sent on the last day of a shorter month.
    /// Required and applicable only if `interval` is `monthly` and `monthly_anchor` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_payout_days: Option<Vec<u32>>,
    /// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
    /// Required and applicable only if `interval` is `weekly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<CreateAccountSettingsPayoutsScheduleWeeklyAnchor>,
    /// The days of the week when available funds are paid out, specified as an array, e.g., [`monday`, `tuesday`].
    /// Required and applicable only if `interval` is `weekly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_payout_days: Option<Vec<CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountSettingsPayoutsSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountSettingsPayoutsSchedule").finish_non_exhaustive()
    }
}
impl CreateAccountSettingsPayoutsSchedule {
    pub fn new() -> Self {
        Self {
            delay_days: None,
            interval: None,
            monthly_anchor: None,
            monthly_payout_days: None,
            weekly_anchor: None,
            weekly_payout_days: None,
        }
    }
}
impl Default for CreateAccountSettingsPayoutsSchedule {
    fn default() -> Self {
        Self::new()
    }
}
/// The number of days charge funds are held before being paid out.
/// May also be set to `minimum`, representing the lowest available value for the account country.
/// Default is `minimum`.
/// The `delay_days` parameter remains at the last configured value if `interval` is `manual`.
/// [Learn more about controlling payout delay days](/connect/manage-payout-schedule).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountSettingsPayoutsScheduleDelayDays {
    Minimum,
    #[serde(untagged)]
    U32(u32),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountSettingsPayoutsScheduleDelayDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccountSettingsPayoutsScheduleDelayDays").finish_non_exhaustive()
    }
}
/// How frequently available funds are paid out.
/// One of: `daily`, `manual`, `weekly`, or `monthly`.
/// Default is `daily`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountSettingsPayoutsScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountSettingsPayoutsScheduleInterval {
    pub fn as_str(&self) -> &str {
        use CreateAccountSettingsPayoutsScheduleInterval::*;
        match self {
            Daily => "daily",
            Manual => "manual",
            Monthly => "monthly",
            Weekly => "weekly",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountSettingsPayoutsScheduleInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountSettingsPayoutsScheduleInterval::*;
        match s {
            "daily" => Ok(Daily),
            "manual" => Ok(Manual),
            "monthly" => Ok(Monthly),
            "weekly" => Ok(Weekly),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountSettingsPayoutsScheduleInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountSettingsPayoutsScheduleInterval))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountSettingsPayoutsScheduleInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountSettingsPayoutsScheduleInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
/// Required and applicable only if `interval` is `weekly`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    pub fn as_str(&self) -> &str {
        use CreateAccountSettingsPayoutsScheduleWeeklyAnchor::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Saturday => "saturday",
            Sunday => "sunday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountSettingsPayoutsScheduleWeeklyAnchor::*;
        match s {
            "friday" => Ok(Friday),
            "monday" => Ok(Monday),
            "saturday" => Ok(Saturday),
            "sunday" => Ok(Sunday),
            "thursday" => Ok(Thursday),
            "tuesday" => Ok(Tuesday),
            "wednesday" => Ok(Wednesday),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountSettingsPayoutsScheduleWeeklyAnchor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountSettingsPayoutsScheduleWeeklyAnchor))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The days of the week when available funds are paid out, specified as an array, e.g., [`monday`, `tuesday`].
/// Required and applicable only if `interval` is `weekly`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    Friday,
    Monday,
    Thursday,
    Tuesday,
    Wednesday,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    pub fn as_str(&self) -> &str {
        use CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays::*;
        match s {
            "friday" => Ok(Friday),
            "monday" => Ok(Monday),
            "thursday" => Ok(Thursday),
            "tuesday" => Ok(Tuesday),
            "wednesday" => Ok(Wednesday),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The `type` parameter is deprecated.
/// Use [`controller`](/api/accounts/create#create_account-controller) instead to configure dashboard access, fee payer, loss liability, and requirement collection.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum CreateAccountType {
    Custom,
    Express,
    Standard,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl CreateAccountType {
    pub fn as_str(&self) -> &str {
        use CreateAccountType::*;
        match self {
            Custom => "custom",
            Express => "express",
            Standard => "standard",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for CreateAccountType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountType::*;
        match s {
            "custom" => Ok(Custom),
            "express" => Ok(Express),
            "standard" => Ok(Standard),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "CreateAccountType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for CreateAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for CreateAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(CreateAccountType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for CreateAccountType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for CreateAccountType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// With [Connect](https://stripe.com/docs/connect), you can create Stripe accounts for your users.
/// To do this, you’ll first need to [register your platform](https://dashboard.stripe.com/account/applications/settings).
///
/// If you’ve already collected information for your connected accounts, you [can prefill that information](https://stripe.com/docs/connect/best-practices#onboarding) when.
/// creating the account.
/// Connect Onboarding won’t ask for the prefilled information during account onboarding.
/// You can prefill any information on the account.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateAccount {
    inner: CreateAccountBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateAccount").finish_non_exhaustive()
    }
}
impl CreateAccount {
    /// Construct a new `CreateAccount`.
    pub fn new() -> Self {
        Self { inner: CreateAccountBuilder::new() }
    }
    /// An [account token](https://api.stripe.com#create_account_token), used to securely provide details to the account.
    pub fn account_token(mut self, account_token: impl Into<String>) -> Self {
        self.inner.account_token = Some(account_token.into());
        self
    }
    /// Business information about the account.
    pub fn business_profile(
        mut self,
        business_profile: impl Into<CreateAccountBusinessProfile>,
    ) -> Self {
        self.inner.business_profile = Some(business_profile.into());
        self
    }
    /// The business type.
    /// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub fn business_type(
        mut self,
        business_type: impl Into<stripe_shared::AccountBusinessType>,
    ) -> Self {
        self.inner.business_type = Some(business_type.into());
        self
    }
    /// Each key of the dictionary represents a capability, and each capability
    /// maps to its settings (for example, whether it has been requested or not). Each
    /// capability is inactive until you have provided its specific
    /// requirements and Stripe has verified them. An account might have some
    /// of its requested capabilities be active and some be inactive.
    ///
    /// Required when [account.controller.stripe_dashboard.type](/api/accounts/create#create_account-controller-dashboard-type).
    /// is `none`, which includes Custom accounts.
    pub fn capabilities(mut self, capabilities: impl Into<CreateAccountCapabilities>) -> Self {
        self.inner.capabilities = Some(capabilities.into());
        self
    }
    /// Information about the company or business.
    /// This field is available for any `business_type`.
    /// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub fn company(mut self, company: impl Into<CreateAccountCompany>) -> Self {
        self.inner.company = Some(company.into());
        self
    }
    /// A hash of configuration describing the account controller's attributes.
    pub fn controller(mut self, controller: impl Into<CreateAccountController>) -> Self {
        self.inner.controller = Some(controller.into());
        self
    }
    /// The country in which the account holder resides, or in which the business is legally established.
    /// This should be an ISO 3166-1 alpha-2 country code.
    /// For example, if you are in the United States and the business for which you're creating an account is legally represented in Canada, you would use `CA` as the country for the account being created.
    /// Available countries include [Stripe's global markets](https://stripe.com/global) as well as countries where [cross-border payouts](https://stripe.com/docs/connect/cross-border-payouts) are supported.
    pub fn country(mut self, country: impl Into<String>) -> Self {
        self.inner.country = Some(country.into());
        self
    }
    /// Three-letter ISO currency code representing the default currency for the account.
    /// This must be a currency that [Stripe supports in the account's country](https://docs.stripe.com/payouts).
    pub fn default_currency(mut self, default_currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.default_currency = Some(default_currency.into());
        self
    }
    /// Documents that may be submitted to satisfy various informational requests.
    pub fn documents(mut self, documents: impl Into<DocumentsSpecs>) -> Self {
        self.inner.documents = Some(documents.into());
        self
    }
    /// The email address of the account holder.
    /// This is only to make the account easier to identify to you.
    /// If [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts, Stripe doesn't email the account without your consent.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.inner.email = Some(email.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A card or bank account to attach to the account for receiving [payouts](/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups).
    /// You can provide either a token, like the ones returned by [Stripe.js](/js), or a dictionary, as documented in the `external_account` parameter for [bank account](/api#account_create_bank_account) creation.
    ///
    ///
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the [bank account](/api#account_create_bank_account) or [card creation](/api#account_create_card) APIs.
    /// After you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub fn external_account(mut self, external_account: impl Into<String>) -> Self {
        self.inner.external_account = Some(external_account.into());
        self
    }
    /// A hash of account group type to tokens. These are account groups this account should be added to.
    pub fn groups(mut self, groups: impl Into<AccountGroupsSpecs>) -> Self {
        self.inner.groups = Some(groups.into());
        self
    }
    /// Information about the person represented by the account.
    /// This field is null unless `business_type` is set to `individual`.
    /// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub fn individual(mut self, individual: impl Into<CreateAccountIndividual>) -> Self {
        self.inner.individual = Some(individual.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Options for customizing how the account functions within Stripe.
    pub fn settings(mut self, settings: impl Into<CreateAccountSettings>) -> Self {
        self.inner.settings = Some(settings.into());
        self
    }
    /// Details on the account's acceptance of the [Stripe Services Agreement](/connect/updating-accounts#tos-acceptance).
    /// This property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    /// This property defaults to a `full` service agreement when empty.
    pub fn tos_acceptance(mut self, tos_acceptance: impl Into<TosAcceptanceSpecs>) -> Self {
        self.inner.tos_acceptance = Some(tos_acceptance.into());
        self
    }
    /// The `type` parameter is deprecated.
    /// Use [`controller`](/api/accounts/create#create_account-controller) instead to configure dashboard access, fee payer, loss liability, and requirement collection.
    pub fn type_(mut self, type_: impl Into<CreateAccountType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl Default for CreateAccount {
    fn default() -> Self {
        Self::new()
    }
}
impl CreateAccount {
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

impl StripeRequest for CreateAccount {
    type Output = stripe_shared::Account;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/accounts").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile: Option<UpdateAccountBusinessProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_type: Option<stripe_shared::AccountBusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<UpdateAccountCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company: Option<UpdateAccountCompany>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_currency: Option<stripe_types::Currency>,
    #[serde(skip_serializing_if = "Option::is_none")]
    documents: Option<DocumentsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<AccountGroupsSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    individual: Option<UpdateAccountIndividual>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    settings: Option<UpdateAccountSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tos_acceptance: Option<TosAcceptanceSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountBuilder").finish_non_exhaustive()
    }
}
impl UpdateAccountBuilder {
    fn new() -> Self {
        Self {
            account_token: None,
            business_profile: None,
            business_type: None,
            capabilities: None,
            company: None,
            default_currency: None,
            documents: None,
            email: None,
            expand: None,
            external_account: None,
            groups: None,
            individual: None,
            metadata: None,
            settings: None,
            tos_acceptance: None,
        }
    }
}
/// Business information about the account.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountBusinessProfile {
    /// The applicant's gross annual revenue for its preceding fiscal year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_revenue: Option<AnnualRevenueSpecs>,
    /// An estimated upper bound of employees, contractors, vendors, etc.
    /// currently working for the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_worker_count: Option<u64>,
    /// [The merchant category code for the account](/connect/setting-mcc).
    /// MCCs are used to classify businesses based on the goods or services they provide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<String>,
    /// Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minority_owned_business_designation:
        Option<Vec<UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation>>,
    /// An estimate of the monthly revenue of the business. Only accepted for accounts in Brazil and India.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_estimated_revenue: Option<MonthlyEstimatedRevenueSpecs>,
    /// The customer-facing business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Internal-only description of the product sold by, or service provided by, the business.
    /// Used by Stripe for risk and underwriting purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// A publicly available mailing address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_address: Option<UpdateAccountBusinessProfileSupportAddress>,
    /// A publicly available email address for sending support issues to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// A publicly available phone number to call with support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_phone: Option<String>,
    /// A publicly available website for handling support issues.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
    /// The business's publicly available website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountBusinessProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountBusinessProfile").finish_non_exhaustive()
    }
}
impl UpdateAccountBusinessProfile {
    pub fn new() -> Self {
        Self {
            annual_revenue: None,
            estimated_worker_count: None,
            mcc: None,
            minority_owned_business_designation: None,
            monthly_estimated_revenue: None,
            name: None,
            product_description: None,
            support_address: None,
            support_email: None,
            support_phone: None,
            support_url: None,
            url: None,
        }
    }
}
impl Default for UpdateAccountBusinessProfile {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether the business is a minority-owned, women-owned, and/or LGBTQI+ -owned business.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    LgbtqiOwnedBusiness,
    MinorityOwnedBusiness,
    NoneOfTheseApply,
    PreferNotToAnswer,
    WomenOwnedBusiness,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    pub fn as_str(&self) -> &str {
        use UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match self {
            LgbtqiOwnedBusiness => "lgbtqi_owned_business",
            MinorityOwnedBusiness => "minority_owned_business",
            NoneOfTheseApply => "none_of_these_apply",
            PreferNotToAnswer => "prefer_not_to_answer",
            WomenOwnedBusiness => "women_owned_business",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match s {
            "lgbtqi_owned_business" => Ok(LgbtqiOwnedBusiness),
            "minority_owned_business" => Ok(MinorityOwnedBusiness),
            "none_of_these_apply" => Ok(NoneOfTheseApply),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "women_owned_business" => Ok(WomenOwnedBusiness),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// A publicly available mailing address for sending support issues to.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountBusinessProfileSupportAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountBusinessProfileSupportAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountBusinessProfileSupportAddress").finish_non_exhaustive()
    }
}
impl UpdateAccountBusinessProfileSupportAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for UpdateAccountBusinessProfileSupportAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Each key of the dictionary represents a capability, and each capability
/// maps to its settings (for example, whether it has been requested or not). Each
/// capability is inactive until you have provided its specific
/// requirements and Stripe has verified them. An account might have some
/// of its requested capabilities be active and some be inactive.
///
/// Required when [account.controller.stripe_dashboard.type](/api/accounts/create#create_account-controller-dashboard-type).
/// is `none`, which includes Custom accounts.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilities {
    /// The acss_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<UpdateAccountCapabilitiesAcssDebitPayments>,
    /// The affirm_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<UpdateAccountCapabilitiesAffirmPayments>,
    /// The afterpay_clearpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<UpdateAccountCapabilitiesAfterpayClearpayPayments>,
    /// The alma_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma_payments: Option<UpdateAccountCapabilitiesAlmaPayments>,
    /// The amazon_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay_payments: Option<UpdateAccountCapabilitiesAmazonPayPayments>,
    /// The app_distribution capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_distribution: Option<UpdateAccountCapabilitiesAppDistribution>,
    /// The au_becs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<UpdateAccountCapabilitiesAuBecsDebitPayments>,
    /// The bacs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<UpdateAccountCapabilitiesBacsDebitPayments>,
    /// The bancontact_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<UpdateAccountCapabilitiesBancontactPayments>,
    /// The bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<UpdateAccountCapabilitiesBankTransferPayments>,
    /// The billie_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie_payments: Option<UpdateAccountCapabilitiesBilliePayments>,
    /// The bizum_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bizum_payments: Option<UpdateAccountCapabilitiesBizumPayments>,
    /// The blik_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<UpdateAccountCapabilitiesBlikPayments>,
    /// The boleto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<UpdateAccountCapabilitiesBoletoPayments>,
    /// The card_issuing capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<UpdateAccountCapabilitiesCardIssuing>,
    /// The card_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<UpdateAccountCapabilitiesCardPayments>,
    /// The cartes_bancaires_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<UpdateAccountCapabilitiesCartesBancairesPayments>,
    /// The cashapp_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<UpdateAccountCapabilitiesCashappPayments>,
    /// The crypto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crypto_payments: Option<UpdateAccountCapabilitiesCryptoPayments>,
    /// The eps_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<UpdateAccountCapabilitiesEpsPayments>,
    /// The fpx_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<UpdateAccountCapabilitiesFpxPayments>,
    /// The gb_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer_payments: Option<UpdateAccountCapabilitiesGbBankTransferPayments>,
    /// The giropay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<UpdateAccountCapabilitiesGiropayPayments>,
    /// The grabpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<UpdateAccountCapabilitiesGrabpayPayments>,
    /// The ideal_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<UpdateAccountCapabilitiesIdealPayments>,
    /// The india_international_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<UpdateAccountCapabilitiesIndiaInternationalPayments>,
    /// The jcb_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<UpdateAccountCapabilitiesJcbPayments>,
    /// The jp_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer_payments: Option<UpdateAccountCapabilitiesJpBankTransferPayments>,
    /// The kakao_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kakao_pay_payments: Option<UpdateAccountCapabilitiesKakaoPayPayments>,
    /// The klarna_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<UpdateAccountCapabilitiesKlarnaPayments>,
    /// The konbini_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<UpdateAccountCapabilitiesKonbiniPayments>,
    /// The kr_card_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_card_payments: Option<UpdateAccountCapabilitiesKrCardPayments>,
    /// The legacy_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<UpdateAccountCapabilitiesLegacyPayments>,
    /// The link_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<UpdateAccountCapabilitiesLinkPayments>,
    /// The mb_way_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mb_way_payments: Option<UpdateAccountCapabilitiesMbWayPayments>,
    /// The mobilepay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay_payments: Option<UpdateAccountCapabilitiesMobilepayPayments>,
    /// The multibanco_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco_payments: Option<UpdateAccountCapabilitiesMultibancoPayments>,
    /// The mx_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer_payments: Option<UpdateAccountCapabilitiesMxBankTransferPayments>,
    /// The naver_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay_payments: Option<UpdateAccountCapabilitiesNaverPayPayments>,
    /// The nz_bank_account_becs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account_becs_debit_payments:
        Option<UpdateAccountCapabilitiesNzBankAccountBecsDebitPayments>,
    /// The oxxo_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<UpdateAccountCapabilitiesOxxoPayments>,
    /// The p24_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<UpdateAccountCapabilitiesP24Payments>,
    /// The pay_by_bank_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank_payments: Option<UpdateAccountCapabilitiesPayByBankPayments>,
    /// The payco_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payco_payments: Option<UpdateAccountCapabilitiesPaycoPayments>,
    /// The paynow_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<UpdateAccountCapabilitiesPaynowPayments>,
    /// The payto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payto_payments: Option<UpdateAccountCapabilitiesPaytoPayments>,
    /// The pix_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix_payments: Option<UpdateAccountCapabilitiesPixPayments>,
    /// The promptpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<UpdateAccountCapabilitiesPromptpayPayments>,
    /// The revolut_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments: Option<UpdateAccountCapabilitiesRevolutPayPayments>,
    /// The samsung_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay_payments: Option<UpdateAccountCapabilitiesSamsungPayPayments>,
    /// The satispay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay_payments: Option<UpdateAccountCapabilitiesSatispayPayments>,
    /// The scalapay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scalapay_payments: Option<UpdateAccountCapabilitiesScalapayPayments>,
    /// The sepa_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_bank_transfer_payments: Option<UpdateAccountCapabilitiesSepaBankTransferPayments>,
    /// The sepa_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<UpdateAccountCapabilitiesSepaDebitPayments>,
    /// The sofort_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<UpdateAccountCapabilitiesSofortPayments>,
    /// The sunbit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunbit_payments: Option<UpdateAccountCapabilitiesSunbitPayments>,
    /// The swish_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish_payments: Option<UpdateAccountCapabilitiesSwishPayments>,
    /// The tax_reporting_us_1099_k capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<UpdateAccountCapabilitiesTaxReportingUs1099K>,
    /// The tax_reporting_us_1099_misc capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<UpdateAccountCapabilitiesTaxReportingUs1099Misc>,
    /// The transfers capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<UpdateAccountCapabilitiesTransfers>,
    /// The treasury capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<UpdateAccountCapabilitiesTreasury>,
    /// The twint_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint_payments: Option<UpdateAccountCapabilitiesTwintPayments>,
    /// The upi_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upi_payments: Option<UpdateAccountCapabilitiesUpiPayments>,
    /// The us_bank_account_ach_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<UpdateAccountCapabilitiesUsBankAccountAchPayments>,
    /// The us_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer_payments: Option<UpdateAccountCapabilitiesUsBankTransferPayments>,
    /// The zip_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<UpdateAccountCapabilitiesZipPayments>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilities").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilities {
    pub fn new() -> Self {
        Self {
            acss_debit_payments: None,
            affirm_payments: None,
            afterpay_clearpay_payments: None,
            alma_payments: None,
            amazon_pay_payments: None,
            app_distribution: None,
            au_becs_debit_payments: None,
            bacs_debit_payments: None,
            bancontact_payments: None,
            bank_transfer_payments: None,
            billie_payments: None,
            bizum_payments: None,
            blik_payments: None,
            boleto_payments: None,
            card_issuing: None,
            card_payments: None,
            cartes_bancaires_payments: None,
            cashapp_payments: None,
            crypto_payments: None,
            eps_payments: None,
            fpx_payments: None,
            gb_bank_transfer_payments: None,
            giropay_payments: None,
            grabpay_payments: None,
            ideal_payments: None,
            india_international_payments: None,
            jcb_payments: None,
            jp_bank_transfer_payments: None,
            kakao_pay_payments: None,
            klarna_payments: None,
            konbini_payments: None,
            kr_card_payments: None,
            legacy_payments: None,
            link_payments: None,
            mb_way_payments: None,
            mobilepay_payments: None,
            multibanco_payments: None,
            mx_bank_transfer_payments: None,
            naver_pay_payments: None,
            nz_bank_account_becs_debit_payments: None,
            oxxo_payments: None,
            p24_payments: None,
            pay_by_bank_payments: None,
            payco_payments: None,
            paynow_payments: None,
            payto_payments: None,
            pix_payments: None,
            promptpay_payments: None,
            revolut_pay_payments: None,
            samsung_pay_payments: None,
            satispay_payments: None,
            scalapay_payments: None,
            sepa_bank_transfer_payments: None,
            sepa_debit_payments: None,
            sofort_payments: None,
            sunbit_payments: None,
            swish_payments: None,
            tax_reporting_us_1099_k: None,
            tax_reporting_us_1099_misc: None,
            transfers: None,
            treasury: None,
            twint_payments: None,
            upi_payments: None,
            us_bank_account_ach_payments: None,
            us_bank_transfer_payments: None,
            zip_payments: None,
        }
    }
}
impl Default for UpdateAccountCapabilities {
    fn default() -> Self {
        Self::new()
    }
}
/// The acss_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesAcssDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesAcssDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesAcssDebitPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesAcssDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesAcssDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The affirm_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesAffirmPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesAffirmPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesAffirmPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesAffirmPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesAffirmPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The afterpay_clearpay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesAfterpayClearpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesAfterpayClearpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesAfterpayClearpayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesAfterpayClearpayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesAfterpayClearpayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The alma_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesAlmaPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesAlmaPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesAlmaPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesAlmaPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesAlmaPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The amazon_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesAmazonPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesAmazonPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesAmazonPayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesAmazonPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesAmazonPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The app_distribution capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesAppDistribution {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesAppDistribution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesAppDistribution").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesAppDistribution {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesAppDistribution {
    fn default() -> Self {
        Self::new()
    }
}
/// The au_becs_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesAuBecsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesAuBecsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesAuBecsDebitPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesAuBecsDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesAuBecsDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The bacs_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesBacsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesBacsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesBacsDebitPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesBacsDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesBacsDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The bancontact_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesBancontactPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesBancontactPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesBancontactPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesBancontactPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesBancontactPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesBankTransferPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The billie_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesBilliePayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesBilliePayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesBilliePayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesBilliePayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesBilliePayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The bizum_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesBizumPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesBizumPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesBizumPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesBizumPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesBizumPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The blik_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesBlikPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesBlikPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesBlikPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesBlikPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesBlikPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The boleto_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesBoletoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesBoletoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesBoletoPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesBoletoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesBoletoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The card_issuing capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesCardIssuing {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesCardIssuing {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesCardIssuing").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesCardIssuing {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesCardIssuing {
    fn default() -> Self {
        Self::new()
    }
}
/// The card_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesCardPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesCardPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesCardPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesCardPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesCardPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The cartes_bancaires_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesCartesBancairesPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesCartesBancairesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesCartesBancairesPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesCartesBancairesPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesCartesBancairesPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The cashapp_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesCashappPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesCashappPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesCashappPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesCashappPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesCashappPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The crypto_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesCryptoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesCryptoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesCryptoPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesCryptoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesCryptoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The eps_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesEpsPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesEpsPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesEpsPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesEpsPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesEpsPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The fpx_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesFpxPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesFpxPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesFpxPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesFpxPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesFpxPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The gb_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesGbBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesGbBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesGbBankTransferPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesGbBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesGbBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The giropay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesGiropayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesGiropayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesGiropayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesGiropayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesGiropayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The grabpay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesGrabpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesGrabpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesGrabpayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesGrabpayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesGrabpayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The ideal_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesIdealPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesIdealPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesIdealPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesIdealPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesIdealPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The india_international_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesIndiaInternationalPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesIndiaInternationalPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesIndiaInternationalPayments")
            .finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesIndiaInternationalPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesIndiaInternationalPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The jcb_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesJcbPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesJcbPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesJcbPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesJcbPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesJcbPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The jp_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesJpBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesJpBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesJpBankTransferPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesJpBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesJpBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The kakao_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesKakaoPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesKakaoPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesKakaoPayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesKakaoPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesKakaoPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The klarna_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesKlarnaPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesKlarnaPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesKlarnaPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesKlarnaPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesKlarnaPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The konbini_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesKonbiniPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesKonbiniPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesKonbiniPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesKonbiniPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesKonbiniPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The kr_card_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesKrCardPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesKrCardPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesKrCardPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesKrCardPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesKrCardPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The legacy_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesLegacyPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesLegacyPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesLegacyPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesLegacyPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesLegacyPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The link_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesLinkPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesLinkPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesLinkPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesLinkPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesLinkPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The mb_way_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesMbWayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesMbWayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesMbWayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesMbWayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesMbWayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The mobilepay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesMobilepayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesMobilepayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesMobilepayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesMobilepayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesMobilepayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The multibanco_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesMultibancoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesMultibancoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesMultibancoPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesMultibancoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesMultibancoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The mx_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesMxBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesMxBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesMxBankTransferPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesMxBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesMxBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The naver_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesNaverPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesNaverPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesNaverPayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesNaverPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesNaverPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The nz_bank_account_becs_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesNzBankAccountBecsDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesNzBankAccountBecsDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesNzBankAccountBecsDebitPayments")
            .finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesNzBankAccountBecsDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesNzBankAccountBecsDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The oxxo_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesOxxoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesOxxoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesOxxoPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesOxxoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesOxxoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The p24_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesP24Payments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesP24Payments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesP24Payments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesP24Payments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesP24Payments {
    fn default() -> Self {
        Self::new()
    }
}
/// The pay_by_bank_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesPayByBankPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesPayByBankPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesPayByBankPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesPayByBankPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesPayByBankPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The payco_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesPaycoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesPaycoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesPaycoPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesPaycoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesPaycoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The paynow_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesPaynowPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesPaynowPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesPaynowPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesPaynowPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesPaynowPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The payto_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesPaytoPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesPaytoPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesPaytoPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesPaytoPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesPaytoPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The pix_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesPixPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesPixPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesPixPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesPixPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesPixPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The promptpay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesPromptpayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesPromptpayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesPromptpayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesPromptpayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesPromptpayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The revolut_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesRevolutPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesRevolutPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesRevolutPayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesRevolutPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesRevolutPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The samsung_pay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesSamsungPayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesSamsungPayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesSamsungPayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesSamsungPayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesSamsungPayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The satispay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesSatispayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesSatispayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesSatispayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesSatispayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesSatispayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The scalapay_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesScalapayPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesScalapayPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesScalapayPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesScalapayPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesScalapayPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The sepa_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesSepaBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesSepaBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesSepaBankTransferPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesSepaBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesSepaBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The sepa_debit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesSepaDebitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesSepaDebitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesSepaDebitPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesSepaDebitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesSepaDebitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The sofort_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesSofortPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesSofortPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesSofortPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesSofortPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesSofortPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The sunbit_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesSunbitPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesSunbitPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesSunbitPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesSunbitPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesSunbitPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The swish_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesSwishPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesSwishPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesSwishPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesSwishPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesSwishPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The tax_reporting_us_1099_k capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099K {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesTaxReportingUs1099K {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesTaxReportingUs1099K").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesTaxReportingUs1099K {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesTaxReportingUs1099K {
    fn default() -> Self {
        Self::new()
    }
}
/// The tax_reporting_us_1099_misc capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesTaxReportingUs1099Misc").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesTaxReportingUs1099Misc {
    fn default() -> Self {
        Self::new()
    }
}
/// The transfers capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesTransfers {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesTransfers {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesTransfers").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesTransfers {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesTransfers {
    fn default() -> Self {
        Self::new()
    }
}
/// The treasury capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesTreasury {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesTreasury {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesTreasury").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesTreasury {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesTreasury {
    fn default() -> Self {
        Self::new()
    }
}
/// The twint_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesTwintPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesTwintPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesTwintPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesTwintPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesTwintPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The upi_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesUpiPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesUpiPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesUpiPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesUpiPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesUpiPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The us_bank_account_ach_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesUsBankAccountAchPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesUsBankAccountAchPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesUsBankAccountAchPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesUsBankAccountAchPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesUsBankAccountAchPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The us_bank_transfer_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesUsBankTransferPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesUsBankTransferPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesUsBankTransferPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesUsBankTransferPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesUsBankTransferPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// The zip_payments capability.
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCapabilitiesZipPayments {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCapabilitiesZipPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCapabilitiesZipPayments").finish_non_exhaustive()
    }
}
impl UpdateAccountCapabilitiesZipPayments {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for UpdateAccountCapabilitiesZipPayments {
    fn default() -> Self {
        Self::new()
    }
}
/// Information about the company or business.
/// This field is available for any `business_type`.
/// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCompany {
    /// The company's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateAccountCompanyAddress>,
    /// The Kana variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<UpdateAccountCompanyAddressKana>,
    /// The Kanji variation of the company's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<UpdateAccountCompanyAddressKanji>,
    /// Whether the company's directors have been provided.
    /// Set this Boolean to `true` after creating all the company's directors with [the Persons API](/api/persons) for accounts with a `relationship.director` requirement.
    /// This value is not automatically set to `true` after creating directors, so it needs to be updated to indicate all directors have been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directors_provided: Option<bool>,
    /// This hash is used to attest that the directors information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directorship_declaration: Option<CompanyDirectorshipDeclaration>,
    /// Whether the company's executives have been provided.
    /// Set this Boolean to `true` after creating all the company's executives with [the Persons API](/api/persons) for accounts with a `relationship.executive` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executives_provided: Option<bool>,
    /// The export license ID number of the company, also referred as Import Export Code (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_license_id: Option<String>,
    /// The purpose code to use for export transactions (India only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_purpose_code: Option<String>,
    /// The company's legal name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The Kana variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kana: Option<String>,
    /// The Kanji variation of the company's legal name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_kanji: Option<String>,
    /// Whether the company's owners have been provided.
    /// Set this Boolean to `true` after creating all the company's owners with [the Persons API](/api/persons) for accounts with a `relationship.owner` requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners_provided: Option<bool>,
    /// This hash is used to attest that the beneficial owner information provided to Stripe is both current and correct.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_declaration: Option<CompanyOwnershipDeclaration>,
    /// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
    /// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_exemption_reason: Option<UpdateAccountCompanyOwnershipExemptionReason>,
    /// The company's phone number (used for verification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<RegistrationDateSpecs>,
    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// This hash is used to attest that the representative is authorized to act as the representative of their legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative_declaration: Option<CompanyRepresentativeDeclaration>,
    /// The category identifying the legal structure of the company or legal entity.
    /// See [Business structure](/connect/identity-verification#business-structure) for more details.
    /// Pass an empty string to unset this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<UpdateAccountCompanyStructure>,
    /// The business ID number of the company, as appropriate for the company’s country.
    /// (Examples are an Employer ID Number in the U.S., a Business Number in Canada, or a Company Number in the UK.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    /// The jurisdiction in which the `tax_id` is registered (Germany-based companies only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_registrar: Option<String>,
    /// The VAT number of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_id: Option<String>,
    /// Information on the verification state of the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<VerificationSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCompany").finish_non_exhaustive()
    }
}
impl UpdateAccountCompany {
    pub fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            directors_provided: None,
            directorship_declaration: None,
            executives_provided: None,
            export_license_id: None,
            export_purpose_code: None,
            name: None,
            name_kana: None,
            name_kanji: None,
            owners_provided: None,
            ownership_declaration: None,
            ownership_exemption_reason: None,
            phone: None,
            registration_date: None,
            registration_number: None,
            representative_declaration: None,
            structure: None,
            tax_id: None,
            tax_id_registrar: None,
            vat_id: None,
            verification: None,
        }
    }
}
impl Default for UpdateAccountCompany {
    fn default() -> Self {
        Self::new()
    }
}
/// The company's primary address.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCompanyAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCompanyAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCompanyAddress").finish_non_exhaustive()
    }
}
impl UpdateAccountCompanyAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for UpdateAccountCompanyAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the company's primary address (Japan only).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCompanyAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCompanyAddressKana {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCompanyAddressKana").finish_non_exhaustive()
    }
}
impl UpdateAccountCompanyAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for UpdateAccountCompanyAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the company's primary address (Japan only).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountCompanyAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCompanyAddressKanji {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountCompanyAddressKanji").finish_non_exhaustive()
    }
}
impl UpdateAccountCompanyAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for UpdateAccountCompanyAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// This value is used to determine if a business is exempt from providing ultimate beneficial owners.
/// See [this support article](https://support.stripe.com/questions/exemption-from-providing-ownership-details) and [changelog](https://docs.stripe.com/changelog/acacia/2025-01-27/ownership-exemption-reason-accounts-api) for more details.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateAccountCompanyOwnershipExemptionReason {
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateAccountCompanyOwnershipExemptionReason {
    pub fn as_str(&self) -> &str {
        use UpdateAccountCompanyOwnershipExemptionReason::*;
        match self {
            QualifiedEntityExceedsOwnershipThreshold => {
                "qualified_entity_exceeds_ownership_threshold"
            }
            QualifiesAsFinancialInstitution => "qualifies_as_financial_institution",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateAccountCompanyOwnershipExemptionReason {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountCompanyOwnershipExemptionReason::*;
        match s {
            "qualified_entity_exceeds_ownership_threshold" => {
                Ok(QualifiedEntityExceedsOwnershipThreshold)
            }
            "qualifies_as_financial_institution" => Ok(QualifiesAsFinancialInstitution),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateAccountCompanyOwnershipExemptionReason"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateAccountCompanyOwnershipExemptionReason))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateAccountCompanyOwnershipExemptionReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountCompanyOwnershipExemptionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The category identifying the legal structure of the company or legal entity.
/// See [Business structure](/connect/identity-verification#business-structure) for more details.
/// Pass an empty string to unset this value.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateAccountCompanyStructure {
    FreeZoneEstablishment,
    FreeZoneLlc,
    GovernmentInstrumentality,
    GovernmentalUnit,
    IncorporatedNonProfit,
    IncorporatedPartnership,
    LimitedLiabilityPartnership,
    Llc,
    MultiMemberLlc,
    PrivateCompany,
    PrivateCorporation,
    PrivatePartnership,
    PublicCompany,
    PublicCorporation,
    PublicPartnership,
    RegisteredCharity,
    SingleMemberLlc,
    SoleEstablishment,
    SoleProprietorship,
    TaxExemptGovernmentInstrumentality,
    UnincorporatedAssociation,
    UnincorporatedNonProfit,
    UnincorporatedPartnership,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateAccountCompanyStructure {
    pub fn as_str(&self) -> &str {
        use UpdateAccountCompanyStructure::*;
        match self {
            FreeZoneEstablishment => "free_zone_establishment",
            FreeZoneLlc => "free_zone_llc",
            GovernmentInstrumentality => "government_instrumentality",
            GovernmentalUnit => "governmental_unit",
            IncorporatedNonProfit => "incorporated_non_profit",
            IncorporatedPartnership => "incorporated_partnership",
            LimitedLiabilityPartnership => "limited_liability_partnership",
            Llc => "llc",
            MultiMemberLlc => "multi_member_llc",
            PrivateCompany => "private_company",
            PrivateCorporation => "private_corporation",
            PrivatePartnership => "private_partnership",
            PublicCompany => "public_company",
            PublicCorporation => "public_corporation",
            PublicPartnership => "public_partnership",
            RegisteredCharity => "registered_charity",
            SingleMemberLlc => "single_member_llc",
            SoleEstablishment => "sole_establishment",
            SoleProprietorship => "sole_proprietorship",
            TaxExemptGovernmentInstrumentality => "tax_exempt_government_instrumentality",
            UnincorporatedAssociation => "unincorporated_association",
            UnincorporatedNonProfit => "unincorporated_non_profit",
            UnincorporatedPartnership => "unincorporated_partnership",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateAccountCompanyStructure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountCompanyStructure::*;
        match s {
            "free_zone_establishment" => Ok(FreeZoneEstablishment),
            "free_zone_llc" => Ok(FreeZoneLlc),
            "government_instrumentality" => Ok(GovernmentInstrumentality),
            "governmental_unit" => Ok(GovernmentalUnit),
            "incorporated_non_profit" => Ok(IncorporatedNonProfit),
            "incorporated_partnership" => Ok(IncorporatedPartnership),
            "limited_liability_partnership" => Ok(LimitedLiabilityPartnership),
            "llc" => Ok(Llc),
            "multi_member_llc" => Ok(MultiMemberLlc),
            "private_company" => Ok(PrivateCompany),
            "private_corporation" => Ok(PrivateCorporation),
            "private_partnership" => Ok(PrivatePartnership),
            "public_company" => Ok(PublicCompany),
            "public_corporation" => Ok(PublicCorporation),
            "public_partnership" => Ok(PublicPartnership),
            "registered_charity" => Ok(RegisteredCharity),
            "single_member_llc" => Ok(SingleMemberLlc),
            "sole_establishment" => Ok(SoleEstablishment),
            "sole_proprietorship" => Ok(SoleProprietorship),
            "tax_exempt_government_instrumentality" => Ok(TaxExemptGovernmentInstrumentality),
            "unincorporated_association" => Ok(UnincorporatedAssociation),
            "unincorporated_non_profit" => Ok(UnincorporatedNonProfit),
            "unincorporated_partnership" => Ok(UnincorporatedPartnership),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateAccountCompanyStructure"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateAccountCompanyStructure)).finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateAccountCompanyStructure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountCompanyStructure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Information about the person represented by the account.
/// This field is null unless `business_type` is set to `individual`.
/// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountIndividual {
    /// The individual's primary address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<UpdateAccountIndividualAddress>,
    /// The Kana variation of the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<UpdateAccountIndividualAddressKana>,
    /// The Kanji variation of the individual's primary address (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<UpdateAccountIndividualAddressKanji>,
    /// The individual's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirthSpecs>,
    /// The individual's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The individual's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The Kana variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,
    /// The Kanji variation of the individual's first name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,
    /// A list of alternate names or aliases that the individual is known by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name_aliases: Option<Vec<String>>,
    /// The individual's gender
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    /// The government-issued ID number of the individual, as appropriate for the representative's country.
    /// (Examples are a Social Security Number in the U.S., or a Social Insurance Number in Canada).
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    /// The government-issued secondary ID number of the individual, as appropriate for the representative's country, will be used for enhanced verification checks.
    /// In Thailand, this would be the laser code found on the back of an ID card.
    /// Instead of the number itself, you can also provide a [PII token created with Stripe.js](/js/tokens/create_token?type=pii).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_secondary: Option<String>,
    /// The individual's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The Kana variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,
    /// The Kanji variation of the individual's last name (Japan only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,
    /// The individual's maiden name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The individual's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub political_exposure: Option<UpdateAccountIndividualPoliticalExposure>,
    /// The individual's registered address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_address: Option<UpdateAccountIndividualRegisteredAddress>,
    /// Describes the person’s relationship to the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<IndividualRelationshipSpecs>,
    /// The last four digits of the individual's Social Security Number (U.S. only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4: Option<String>,
    /// The individual's verification document information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountIndividual {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountIndividual").finish_non_exhaustive()
    }
}
impl UpdateAccountIndividual {
    pub fn new() -> Self {
        Self {
            address: None,
            address_kana: None,
            address_kanji: None,
            dob: None,
            email: None,
            first_name: None,
            first_name_kana: None,
            first_name_kanji: None,
            full_name_aliases: None,
            gender: None,
            id_number: None,
            id_number_secondary: None,
            last_name: None,
            last_name_kana: None,
            last_name_kanji: None,
            maiden_name: None,
            metadata: None,
            phone: None,
            political_exposure: None,
            registered_address: None,
            relationship: None,
            ssn_last_4: None,
            verification: None,
        }
    }
}
impl Default for UpdateAccountIndividual {
    fn default() -> Self {
        Self::new()
    }
}
/// The individual's primary address.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountIndividualAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountIndividualAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountIndividualAddress").finish_non_exhaustive()
    }
}
impl UpdateAccountIndividualAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for UpdateAccountIndividualAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kana variation of the individual's primary address (Japan only).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountIndividualAddressKana {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountIndividualAddressKana {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountIndividualAddressKana").finish_non_exhaustive()
    }
}
impl UpdateAccountIndividualAddressKana {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for UpdateAccountIndividualAddressKana {
    fn default() -> Self {
        Self::new()
    }
}
/// The Kanji variation of the individual's primary address (Japan only).
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountIndividualAddressKanji {
    /// City or ward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Block or building number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Building details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// Postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// Prefecture.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Town or cho-me.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub town: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountIndividualAddressKanji {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountIndividualAddressKanji").finish_non_exhaustive()
    }
}
impl UpdateAccountIndividualAddressKanji {
    pub fn new() -> Self {
        Self {
            city: None,
            country: None,
            line1: None,
            line2: None,
            postal_code: None,
            state: None,
            town: None,
        }
    }
}
impl Default for UpdateAccountIndividualAddressKanji {
    fn default() -> Self {
        Self::new()
    }
}
/// Indicates if the person or any of their representatives, family members, or other closely related persons, declares that they hold or have held an important public job or function, in any jurisdiction.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateAccountIndividualPoliticalExposure {
    Existing,
    None,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateAccountIndividualPoliticalExposure {
    pub fn as_str(&self) -> &str {
        use UpdateAccountIndividualPoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateAccountIndividualPoliticalExposure {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountIndividualPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateAccountIndividualPoliticalExposure"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateAccountIndividualPoliticalExposure)).finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateAccountIndividualPoliticalExposure {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountIndividualPoliticalExposure {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The individual's registered address.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountIndividualRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1, such as the street, PO Box, or company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountIndividualRegisteredAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountIndividualRegisteredAddress").finish_non_exhaustive()
    }
}
impl UpdateAccountIndividualRegisteredAddress {
    pub fn new() -> Self {
        Self { city: None, country: None, line1: None, line2: None, postal_code: None, state: None }
    }
}
impl Default for UpdateAccountIndividualRegisteredAddress {
    fn default() -> Self {
        Self::new()
    }
}
/// Options for customizing how the account functions within Stripe.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountSettings {
    /// Settings specific to Bacs Direct Debit payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<BacsDebitPaymentsSpecs>,
    /// Settings used to apply the account's branding to email receipts, invoices, Checkout, and other products.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branding: Option<BrandingSettingsSpecs>,
    /// Settings specific to the account's use of the Card Issuing product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CardIssuingSettingsSpecs>,
    /// Settings specific to card charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CardPaymentsSettingsSpecs>,
    /// Settings specific to the account's use of Invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<UpdateAccountSettingsInvoices>,
    /// Settings that apply across payment methods for charging on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<PaymentsSettingsSpecs>,
    /// Settings specific to the account's payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<UpdateAccountSettingsPayouts>,
    /// Settings specific to the account's Treasury FinancialAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<TreasurySettingsSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountSettings {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountSettings").finish_non_exhaustive()
    }
}
impl UpdateAccountSettings {
    pub fn new() -> Self {
        Self {
            bacs_debit_payments: None,
            branding: None,
            card_issuing: None,
            card_payments: None,
            invoices: None,
            payments: None,
            payouts: None,
            treasury: None,
        }
    }
}
impl Default for UpdateAccountSettings {
    fn default() -> Self {
        Self::new()
    }
}
/// Settings specific to the account's use of Invoices.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountSettingsInvoices {
    /// The list of default Account Tax IDs to automatically include on invoices.
    /// Account Tax IDs get added when an invoice is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_account_tax_ids: Option<Vec<String>>,
    /// Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when the customer already has a default payment method on the hosted invoice page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_payment_method_save: Option<UpdateAccountSettingsInvoicesHostedPaymentMethodSave>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountSettingsInvoices {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountSettingsInvoices").finish_non_exhaustive()
    }
}
impl UpdateAccountSettingsInvoices {
    pub fn new() -> Self {
        Self { default_account_tax_ids: None, hosted_payment_method_save: None }
    }
}
impl Default for UpdateAccountSettingsInvoices {
    fn default() -> Self {
        Self::new()
    }
}
/// Whether to save the payment method after a payment is completed for a one-time invoice or a subscription invoice when the customer already has a default payment method on the hosted invoice page.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    Always,
    Never,
    Offer,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    pub fn as_str(&self) -> &str {
        use UpdateAccountSettingsInvoicesHostedPaymentMethodSave::*;
        match self {
            Always => "always",
            Never => "never",
            Offer => "offer",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountSettingsInvoicesHostedPaymentMethodSave::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            "offer" => Ok(Offer),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateAccountSettingsInvoicesHostedPaymentMethodSave"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateAccountSettingsInvoicesHostedPaymentMethodSave))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Settings specific to the account's payouts.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountSettingsPayouts {
    /// A Boolean indicating whether Stripe should try to reclaim negative balances from an attached bank account.
    /// For details, see [Understanding Connect Account Balances](/connect/account-balances).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_negative_balances: Option<bool>,
    /// Details on when funds from charges are available, and when they are paid out to an external account.
    /// For details, see our [Setting Bank and Debit Card Payouts](/connect/bank-transfers#payout-information) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<UpdateAccountSettingsPayoutsSchedule>,
    /// The text that appears on the bank account statement for payouts.
    /// If not set, this defaults to the platform's bank descriptor as set in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountSettingsPayouts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountSettingsPayouts").finish_non_exhaustive()
    }
}
impl UpdateAccountSettingsPayouts {
    pub fn new() -> Self {
        Self { debit_negative_balances: None, schedule: None, statement_descriptor: None }
    }
}
impl Default for UpdateAccountSettingsPayouts {
    fn default() -> Self {
        Self::new()
    }
}
/// Details on when funds from charges are available, and when they are paid out to an external account.
/// For details, see our [Setting Bank and Debit Card Payouts](/connect/bank-transfers#payout-information) documentation.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccountSettingsPayoutsSchedule {
    /// The number of days charge funds are held before being paid out.
    /// May also be set to `minimum`, representing the lowest available value for the account country.
    /// Default is `minimum`.
    /// The `delay_days` parameter remains at the last configured value if `interval` is `manual`.
    /// [Learn more about controlling payout delay days](/connect/manage-payout-schedule).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay_days: Option<UpdateAccountSettingsPayoutsScheduleDelayDays>,
    /// How frequently available funds are paid out.
    /// One of: `daily`, `manual`, `weekly`, or `monthly`.
    /// Default is `daily`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<UpdateAccountSettingsPayoutsScheduleInterval>,
    /// The day of the month when available funds are paid out, specified as a number between 1--31.
    /// Payouts nominally scheduled between the 29th and 31st of the month are instead sent on the last day of a shorter month.
    /// Required and applicable only if `interval` is `monthly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_anchor: Option<u8>,
    /// The days of the month when available funds are paid out, specified as an array of numbers between 1--31.
    /// Payouts nominally scheduled between the 29th and 31st of the month are instead sent on the last day of a shorter month.
    /// Required and applicable only if `interval` is `monthly` and `monthly_anchor` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_payout_days: Option<Vec<u32>>,
    /// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
    /// Required and applicable only if `interval` is `weekly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<UpdateAccountSettingsPayoutsScheduleWeeklyAnchor>,
    /// The days of the week when available funds are paid out, specified as an array, e.g., [`monday`, `tuesday`].
    /// Required and applicable only if `interval` is `weekly`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_payout_days: Option<Vec<UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountSettingsPayoutsSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountSettingsPayoutsSchedule").finish_non_exhaustive()
    }
}
impl UpdateAccountSettingsPayoutsSchedule {
    pub fn new() -> Self {
        Self {
            delay_days: None,
            interval: None,
            monthly_anchor: None,
            monthly_payout_days: None,
            weekly_anchor: None,
            weekly_payout_days: None,
        }
    }
}
impl Default for UpdateAccountSettingsPayoutsSchedule {
    fn default() -> Self {
        Self::new()
    }
}
/// The number of days charge funds are held before being paid out.
/// May also be set to `minimum`, representing the lowest available value for the account country.
/// Default is `minimum`.
/// The `delay_days` parameter remains at the last configured value if `interval` is `manual`.
/// [Learn more about controlling payout delay days](/connect/manage-payout-schedule).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAccountSettingsPayoutsScheduleDelayDays {
    Minimum,
    #[serde(untagged)]
    U32(u32),
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountSettingsPayoutsScheduleDelayDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccountSettingsPayoutsScheduleDelayDays").finish_non_exhaustive()
    }
}
/// How frequently available funds are paid out.
/// One of: `daily`, `manual`, `weekly`, or `monthly`.
/// Default is `daily`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateAccountSettingsPayoutsScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateAccountSettingsPayoutsScheduleInterval {
    pub fn as_str(&self) -> &str {
        use UpdateAccountSettingsPayoutsScheduleInterval::*;
        match self {
            Daily => "daily",
            Manual => "manual",
            Monthly => "monthly",
            Weekly => "weekly",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateAccountSettingsPayoutsScheduleInterval {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountSettingsPayoutsScheduleInterval::*;
        match s {
            "daily" => Ok(Daily),
            "manual" => Ok(Manual),
            "monthly" => Ok(Monthly),
            "weekly" => Ok(Weekly),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateAccountSettingsPayoutsScheduleInterval"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateAccountSettingsPayoutsScheduleInterval))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateAccountSettingsPayoutsScheduleInterval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountSettingsPayoutsScheduleInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
/// Required and applicable only if `interval` is `weekly`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    pub fn as_str(&self) -> &str {
        use UpdateAccountSettingsPayoutsScheduleWeeklyAnchor::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Saturday => "saturday",
            Sunday => "sunday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountSettingsPayoutsScheduleWeeklyAnchor::*;
        match s {
            "friday" => Ok(Friday),
            "monday" => Ok(Monday),
            "saturday" => Ok(Saturday),
            "sunday" => Ok(Sunday),
            "thursday" => Ok(Thursday),
            "tuesday" => Ok(Tuesday),
            "wednesday" => Ok(Wednesday),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateAccountSettingsPayoutsScheduleWeeklyAnchor"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateAccountSettingsPayoutsScheduleWeeklyAnchor))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// The days of the week when available funds are paid out, specified as an array, e.g., [`monday`, `tuesday`].
/// Required and applicable only if `interval` is `weekly`.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    Friday,
    Monday,
    Thursday,
    Tuesday,
    Wednesday,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    pub fn as_str(&self) -> &str {
        use UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays::*;
        match s {
            "friday" => Ok(Friday),
            "monday" => Ok(Monday),
            "thursday" => Ok(Thursday),
            "tuesday" => Ok(Tuesday),
            "wednesday" => Ok(Wednesday),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays))
            .finish_non_exhaustive()
    }
}
impl serde::Serialize for UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateAccountSettingsPayoutsScheduleWeeklyPayoutDays {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Updates a <a href="/connect/accounts">connected account</a> by setting the values of the parameters passed.
/// Any parameters not provided are.
/// left unchanged.
///
/// For accounts where <a href="/api/accounts/object#account_object-controller-requirement_collection">controller.requirement_collection</a>.
/// is `application`, which includes Custom accounts, you can update any information on the account.
///
/// For accounts where <a href="/api/accounts/object#account_object-controller-requirement_collection">controller.requirement_collection</a>.
/// is `stripe`, which includes Standard and Express accounts, you can update all information until you create.
/// an <a href="/api/account_links">Account Link</a> or <a href="/api/account_sessions">Account Session</a> to start Connect onboarding,.
/// after which some properties can no longer be updated.
///
/// To update your own account, use the [Dashboard](https://dashboard.stripe.com/settings/account).
/// Refer to our.
/// [Connect](https://stripe.com/docs/connect/updating-accounts) documentation to learn more about updating accounts.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateAccount {
    inner: UpdateAccountBuilder,
    account: stripe_shared::AccountId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateAccount").finish_non_exhaustive()
    }
}
impl UpdateAccount {
    /// Construct a new `UpdateAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into(), inner: UpdateAccountBuilder::new() }
    }
    /// An [account token](https://api.stripe.com#create_account_token), used to securely provide details to the account.
    pub fn account_token(mut self, account_token: impl Into<String>) -> Self {
        self.inner.account_token = Some(account_token.into());
        self
    }
    /// Business information about the account.
    pub fn business_profile(
        mut self,
        business_profile: impl Into<UpdateAccountBusinessProfile>,
    ) -> Self {
        self.inner.business_profile = Some(business_profile.into());
        self
    }
    /// The business type.
    /// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub fn business_type(
        mut self,
        business_type: impl Into<stripe_shared::AccountBusinessType>,
    ) -> Self {
        self.inner.business_type = Some(business_type.into());
        self
    }
    /// Each key of the dictionary represents a capability, and each capability
    /// maps to its settings (for example, whether it has been requested or not). Each
    /// capability is inactive until you have provided its specific
    /// requirements and Stripe has verified them. An account might have some
    /// of its requested capabilities be active and some be inactive.
    ///
    /// Required when [account.controller.stripe_dashboard.type](/api/accounts/create#create_account-controller-dashboard-type).
    /// is `none`, which includes Custom accounts.
    pub fn capabilities(mut self, capabilities: impl Into<UpdateAccountCapabilities>) -> Self {
        self.inner.capabilities = Some(capabilities.into());
        self
    }
    /// Information about the company or business.
    /// This field is available for any `business_type`.
    /// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub fn company(mut self, company: impl Into<UpdateAccountCompany>) -> Self {
        self.inner.company = Some(company.into());
        self
    }
    /// Three-letter ISO currency code representing the default currency for the account.
    /// This must be a currency that [Stripe supports in the account's country](https://docs.stripe.com/payouts).
    pub fn default_currency(mut self, default_currency: impl Into<stripe_types::Currency>) -> Self {
        self.inner.default_currency = Some(default_currency.into());
        self
    }
    /// Documents that may be submitted to satisfy various informational requests.
    pub fn documents(mut self, documents: impl Into<DocumentsSpecs>) -> Self {
        self.inner.documents = Some(documents.into());
        self
    }
    /// The email address of the account holder.
    /// This is only to make the account easier to identify to you.
    /// If [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts, Stripe doesn't email the account without your consent.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.inner.email = Some(email.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// A card or bank account to attach to the account for receiving [payouts](/connect/bank-debit-card-payouts) (you won’t be able to use it for top-ups).
    /// You can provide either a token, like the ones returned by [Stripe.js](/js), or a dictionary, as documented in the `external_account` parameter for [bank account](/api#account_create_bank_account) creation.
    ///
    ///
    /// By default, providing an external account sets it as the new default external account for its currency, and deletes the old default if one exists.
    /// To add additional external accounts without replacing the existing default for the currency, use the [bank account](/api#account_create_bank_account) or [card creation](/api#account_create_card) APIs.
    /// After you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub fn external_account(mut self, external_account: impl Into<String>) -> Self {
        self.inner.external_account = Some(external_account.into());
        self
    }
    /// A hash of account group type to tokens. These are account groups this account should be added to.
    pub fn groups(mut self, groups: impl Into<AccountGroupsSpecs>) -> Self {
        self.inner.groups = Some(groups.into());
        self
    }
    /// Information about the person represented by the account.
    /// This field is null unless `business_type` is set to `individual`.
    /// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    pub fn individual(mut self, individual: impl Into<UpdateAccountIndividual>) -> Self {
        self.inner.individual = Some(individual.into());
        self
    }
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(
        mut self,
        metadata: impl Into<std::collections::HashMap<String, String>>,
    ) -> Self {
        self.inner.metadata = Some(metadata.into());
        self
    }
    /// Options for customizing how the account functions within Stripe.
    pub fn settings(mut self, settings: impl Into<UpdateAccountSettings>) -> Self {
        self.inner.settings = Some(settings.into());
        self
    }
    /// Details on the account's acceptance of the [Stripe Services Agreement](/connect/updating-accounts#tos-acceptance).
    /// This property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
    /// This property defaults to a `full` service agreement when empty.
    pub fn tos_acceptance(mut self, tos_acceptance: impl Into<TosAcceptanceSpecs>) -> Self {
        self.inner.tos_acceptance = Some(tos_acceptance.into());
        self
    }
}
impl UpdateAccount {
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

impl StripeRequest for UpdateAccount {
    type Output = stripe_shared::Account;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Post, format!("/accounts/{account}")).form(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RejectAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    reason: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RejectAccountBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RejectAccountBuilder").finish_non_exhaustive()
    }
}
impl RejectAccountBuilder {
    fn new(reason: impl Into<String>) -> Self {
        Self { expand: None, reason: reason.into() }
    }
}
/// With <a href="/connect">Connect</a>, you can reject accounts that you have flagged as suspicious.
///
/// Only accounts where your platform is liable for negative account balances, which includes Custom and Express accounts, can be rejected.
/// Test-mode accounts can be rejected at any time.
/// Live-mode accounts can only be rejected after all balances are zero.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RejectAccount {
    inner: RejectAccountBuilder,
    account: stripe_shared::AccountId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RejectAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RejectAccount").finish_non_exhaustive()
    }
}
impl RejectAccount {
    /// Construct a new `RejectAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>, reason: impl Into<String>) -> Self {
        Self { account: account.into(), inner: RejectAccountBuilder::new(reason.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RejectAccount {
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

impl StripeRequest for RejectAccount {
    type Output = stripe_shared::Account;

    fn build(&self) -> RequestBuilder {
        let account = &self.account;
        RequestBuilder::new(StripeMethod::Post, format!("/accounts/{account}/reject"))
            .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AnnualRevenueSpecs {
    /// A non-negative integer representing the amount in the [smallest currency unit](/currencies#zero-decimal).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
    /// The close-out date of the preceding fiscal year in ISO 8601 format.
    /// E.g.
    /// 2023-12-31 for the 31st of December, 2023.
    pub fiscal_year_end: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AnnualRevenueSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AnnualRevenueSpecs").finish_non_exhaustive()
    }
}
impl AnnualRevenueSpecs {
    pub fn new(
        amount: impl Into<i64>,
        currency: impl Into<stripe_types::Currency>,
        fiscal_year_end: impl Into<String>,
    ) -> Self {
        Self {
            amount: amount.into(),
            currency: currency.into(),
            fiscal_year_end: fiscal_year_end.into(),
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct MonthlyEstimatedRevenueSpecs {
    /// A non-negative integer representing how much to charge in the [smallest currency unit](/currencies#zero-decimal).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for MonthlyEstimatedRevenueSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MonthlyEstimatedRevenueSpecs").finish_non_exhaustive()
    }
}
impl MonthlyEstimatedRevenueSpecs {
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { amount: amount.into(), currency: currency.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CompanyDirectorshipDeclaration {
    /// The Unix timestamp marking when the directorship declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the directorship declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the directorship declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CompanyDirectorshipDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CompanyDirectorshipDeclaration").finish_non_exhaustive()
    }
}
impl CompanyDirectorshipDeclaration {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for CompanyDirectorshipDeclaration {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CompanyOwnershipDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the beneficial owner attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CompanyOwnershipDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CompanyOwnershipDeclaration").finish_non_exhaustive()
    }
}
impl CompanyOwnershipDeclaration {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for CompanyOwnershipDeclaration {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RegistrationDateSpecs {
    /// The day of registration, between 1 and 31.
    pub day: i64,
    /// The month of registration, between 1 and 12.
    pub month: i64,
    /// The four-digit year of registration.
    pub year: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RegistrationDateSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RegistrationDateSpecs").finish_non_exhaustive()
    }
}
impl RegistrationDateSpecs {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CompanyRepresentativeDeclaration {
    /// The Unix timestamp marking when the representative declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the representative declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the representative declaration attestation was made.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CompanyRepresentativeDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CompanyRepresentativeDeclaration").finish_non_exhaustive()
    }
}
impl CompanyRepresentativeDeclaration {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for CompanyRepresentativeDeclaration {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct VerificationDocumentSpecs {
    /// The back of a document returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `additional_verification`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    /// The front of a document returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `additional_verification`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for VerificationDocumentSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VerificationDocumentSpecs").finish_non_exhaustive()
    }
}
impl VerificationDocumentSpecs {
    pub fn new() -> Self {
        Self { back: None, front: None }
    }
}
impl Default for VerificationDocumentSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DocumentsParam {
    /// One or more document ids returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DocumentsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DocumentsParam").finish_non_exhaustive()
    }
}
impl DocumentsParam {
    pub fn new() -> Self {
        Self { files: None }
    }
}
impl Default for DocumentsParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SignerNameParam {
    /// The token of the person signing the document, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SignerNameParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SignerNameParam").finish_non_exhaustive()
    }
}
impl SignerNameParam {
    pub fn new() -> Self {
        Self { person: None }
    }
}
impl Default for SignerNameParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AccountGroupsSpecs {
    /// The group the account is in to determine their payments pricing, and null if the account is on customized pricing.
    /// [See the Platform pricing tool documentation](https://docs.stripe.com/connect/platform-pricing-tools) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_pricing: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AccountGroupsSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AccountGroupsSpecs").finish_non_exhaustive()
    }
}
impl AccountGroupsSpecs {
    pub fn new() -> Self {
        Self { payments_pricing: None }
    }
}
impl Default for AccountGroupsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DateOfBirthSpecs {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DateOfBirthSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DateOfBirthSpecs").finish_non_exhaustive()
    }
}
impl DateOfBirthSpecs {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct IndividualRelationshipSpecs {
    /// Whether the person is a director of the account's legal entity.
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,
    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,
    /// Whether the person is an owner of the account’s legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    /// The percent owned by the person of the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,
    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IndividualRelationshipSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndividualRelationshipSpecs").finish_non_exhaustive()
    }
}
impl IndividualRelationshipSpecs {
    pub fn new() -> Self {
        Self { director: None, executive: None, owner: None, percent_ownership: None, title: None }
    }
}
impl Default for IndividualRelationshipSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PersonVerificationDocumentSpecs {
    /// The back of an ID returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    /// The front of an ID returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonVerificationDocumentSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonVerificationDocumentSpecs").finish_non_exhaustive()
    }
}
impl PersonVerificationDocumentSpecs {
    pub fn new() -> Self {
        Self { back: None, front: None }
    }
}
impl Default for PersonVerificationDocumentSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BacsDebitPaymentsSpecs {
    /// The Bacs Direct Debit Display Name for this account.
    /// For payments made with Bacs Direct Debit, this name appears on the mandate as the statement descriptor.
    /// Mobile banking apps display it as the name of the business.
    /// To use custom branding, set the Bacs Direct Debit Display Name during or right after creation.
    /// Custom branding incurs an additional monthly fee for the platform.
    /// If you don't set the display name before requesting Bacs capability, it's automatically set as "Stripe" and the account is onboarded to Stripe branding, which is free.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BacsDebitPaymentsSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BacsDebitPaymentsSpecs").finish_non_exhaustive()
    }
}
impl BacsDebitPaymentsSpecs {
    pub fn new() -> Self {
        Self { display_name: None }
    }
}
impl Default for BacsDebitPaymentsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BrandingSettingsSpecs {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) An icon for the account.
    /// Must be square and at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A logo for the account that will be used in Checkout instead of the icon and without the account's name next to it if provided.
    /// Must be at least 128px x 128px.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    /// A CSS hex color value representing the primary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_color: Option<String>,
    /// A CSS hex color value representing the secondary branding color for this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_color: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BrandingSettingsSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BrandingSettingsSpecs").finish_non_exhaustive()
    }
}
impl BrandingSettingsSpecs {
    pub fn new() -> Self {
        Self { icon: None, logo: None, primary_color: None, secondary_color: None }
    }
}
impl Default for BrandingSettingsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SettingsTermsOfServiceSpecs {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SettingsTermsOfServiceSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SettingsTermsOfServiceSpecs").finish_non_exhaustive()
    }
}
impl SettingsTermsOfServiceSpecs {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for SettingsTermsOfServiceSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DeclineChargeOnSpecs {
    /// Whether Stripe automatically declines charges with an incorrect ZIP or postal code.
    /// This setting only applies when a ZIP or postal code is provided and they fail bank verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_failure: Option<bool>,
    /// Whether Stripe automatically declines charges with an incorrect CVC.
    /// This setting only applies when a CVC is provided and it fails bank verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_failure: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DeclineChargeOnSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DeclineChargeOnSpecs").finish_non_exhaustive()
    }
}
impl DeclineChargeOnSpecs {
    pub fn new() -> Self {
        Self { avs_failure: None, cvc_failure: None }
    }
}
impl Default for DeclineChargeOnSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PaymentsSettingsSpecs {
    /// The default text that appears on statements for non-card charges outside of Japan.
    /// For card charges, if you don't set a `statement_descriptor_prefix`, this text is also used as the statement descriptor prefix.
    /// In that case, if concatenating the statement descriptor suffix causes the combined statement descriptor to exceed 22 characters, we truncate the `statement_descriptor` text to limit the full descriptor to 22 characters.
    /// For more information about statement descriptors and their requirements, see the [account settings documentation](https://docs.stripe.com/get-started/account/statement-descriptors).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    /// The Kana variation of `statement_descriptor` used for charges in Japan.
    /// Japanese statement descriptors have [special requirements](https://docs.stripe.com/get-started/account/statement-descriptors#set-japanese-statement-descriptors).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kana: Option<String>,
    /// The Kanji variation of `statement_descriptor` used for charges in Japan.
    /// Japanese statement descriptors have [special requirements](https://docs.stripe.com/get-started/account/statement-descriptors#set-japanese-statement-descriptors).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_kanji: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PaymentsSettingsSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PaymentsSettingsSpecs").finish_non_exhaustive()
    }
}
impl PaymentsSettingsSpecs {
    pub fn new() -> Self {
        Self {
            statement_descriptor: None,
            statement_descriptor_kana: None,
            statement_descriptor_kanji: None,
        }
    }
}
impl Default for PaymentsSettingsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct TosAcceptanceSpecs {
    /// The Unix timestamp marking when the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user's service agreement type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<String>,
    /// The user agent of the browser from which the account representative accepted their service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TosAcceptanceSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TosAcceptanceSpecs").finish_non_exhaustive()
    }
}
impl TosAcceptanceSpecs {
    pub fn new() -> Self {
        Self { date: None, ip: None, service_agreement: None, user_agent: None }
    }
}
impl Default for TosAcceptanceSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct VerificationSpecs {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for VerificationSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("VerificationSpecs").finish_non_exhaustive()
    }
}
impl VerificationSpecs {
    pub fn new() -> Self {
        Self { document: None }
    }
}
impl Default for VerificationSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SignerParam {
    /// One or more document ids returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    /// Information regarding the person signing the document if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer: Option<SignerNameParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SignerParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SignerParam").finish_non_exhaustive()
    }
}
impl SignerParam {
    pub fn new() -> Self {
        Self { files: None, signer: None }
    }
}
impl Default for SignerParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PersonVerificationSpecs {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocumentSpecs>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonVerificationSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonVerificationSpecs").finish_non_exhaustive()
    }
}
impl PersonVerificationSpecs {
    pub fn new() -> Self {
        Self { additional_document: None, document: None }
    }
}
impl Default for PersonVerificationSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CardIssuingSettingsSpecs {
    /// Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](/issuing/connect/tos_acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<SettingsTermsOfServiceSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CardIssuingSettingsSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CardIssuingSettingsSpecs").finish_non_exhaustive()
    }
}
impl CardIssuingSettingsSpecs {
    pub fn new() -> Self {
        Self { tos_acceptance: None }
    }
}
impl Default for CardIssuingSettingsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CardPaymentsSettingsSpecs {
    /// Automatically declines certain charge types regardless of whether the card issuer accepted or declined the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_on: Option<DeclineChargeOnSpecs>,
    /// The default text that appears on credit card statements when a charge is made.
    /// This field prefixes any dynamic `statement_descriptor` specified on the charge.
    /// `statement_descriptor_prefix` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix: Option<String>,
    /// The Kana variation of the default text that appears on credit card statements when a charge is made (Japan only).
    /// This field prefixes any dynamic `statement_descriptor_suffix_kana` specified on the charge.
    /// `statement_descriptor_prefix_kana` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kana: Option<String>,
    /// The Kanji variation of the default text that appears on credit card statements when a charge is made (Japan only).
    /// This field prefixes any dynamic `statement_descriptor_suffix_kanji` specified on the charge.
    /// `statement_descriptor_prefix_kanji` is useful for maximizing descriptor space for the dynamic portion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_prefix_kanji: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CardPaymentsSettingsSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CardPaymentsSettingsSpecs").finish_non_exhaustive()
    }
}
impl CardPaymentsSettingsSpecs {
    pub fn new() -> Self {
        Self {
            decline_on: None,
            statement_descriptor_prefix: None,
            statement_descriptor_prefix_kana: None,
            statement_descriptor_prefix_kanji: None,
        }
    }
}
impl Default for CardPaymentsSettingsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct TreasurySettingsSpecs {
    /// Details on the account's acceptance of the Stripe Treasury Services Agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<SettingsTermsOfServiceSpecs>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TreasurySettingsSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TreasurySettingsSpecs").finish_non_exhaustive()
    }
}
impl TreasurySettingsSpecs {
    pub fn new() -> Self {
        Self { tos_acceptance: None }
    }
}
impl Default for TreasurySettingsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct DocumentsSpecs {
    /// One or more documents that support the [Bank account ownership verification](https://support.stripe.com/questions/bank-account-ownership-verification) requirement.
    /// Must be a document associated with the account’s primary active bank account that displays the last 4 digits of the account number, either a statement or a check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_ownership_verification: Option<DocumentsParam>,
    /// One or more documents that demonstrate proof of a company's license to operate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_license: Option<DocumentsParam>,
    /// One or more documents showing the company's Memorandum of Association.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_memorandum_of_association: Option<DocumentsParam>,
    /// (Certain countries only) One or more documents showing the ministerial decree legalizing the company's establishment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_ministerial_decree: Option<DocumentsParam>,
    /// One or more documents that demonstrate proof of a company's registration with the appropriate local authorities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_registration_verification: Option<DocumentsParam>,
    /// One or more documents that demonstrate proof of a company's tax ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_tax_id_verification: Option<DocumentsParam>,
    /// One or more documents that demonstrate proof of address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_address: Option<DocumentsParam>,
    /// One or more documents showing the company’s proof of registration with the national business registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<SignerParam>,
    /// One or more documents that demonstrate proof of ultimate beneficial ownership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_ultimate_beneficial_ownership: Option<SignerParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for DocumentsSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("DocumentsSpecs").finish_non_exhaustive()
    }
}
impl DocumentsSpecs {
    pub fn new() -> Self {
        Self {
            bank_account_ownership_verification: None,
            company_license: None,
            company_memorandum_of_association: None,
            company_ministerial_decree: None,
            company_registration_verification: None,
            company_tax_id_verification: None,
            proof_of_address: None,
            proof_of_registration: None,
            proof_of_ultimate_beneficial_ownership: None,
        }
    }
}
impl Default for DocumentsSpecs {
    fn default() -> Self {
        Self::new()
    }
}
