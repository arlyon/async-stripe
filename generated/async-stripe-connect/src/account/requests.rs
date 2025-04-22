use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// With <a href="/connect">Connect</a>, you can delete accounts you manage.
///
/// Test-mode accounts can be deleted at any time.
///
/// Live-mode accounts where Stripe is responsible for negative account balances cannot be deleted, which includes Standard accounts.
/// Live-mode accounts where your platform is liable for negative account balances, which includes Custom and Express accounts, can be deleted when all <a href="/api/balance/balance_object">balances</a> are zero.
///
/// If you want to delete your own account, use the [account information tab in your account settings](https://dashboard.stripe.com/settings/account) instead.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteAccount {
    account: stripe_shared::AccountId,
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
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveForMyAccountAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveForMyAccountAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveForMyAccountAccount {
    inner: RetrieveForMyAccountAccountBuilder,
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
#[derive(Clone, Debug, serde::Serialize)]
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
impl ListAccountBuilder {
    fn new() -> Self {
        Self { created: None, ending_before: None, expand: None, limit: None, starting_after: None }
    }
}
/// Returns a list of accounts connected to your platform via [Connect](https://stripe.com/docs/connect).
/// If you’re not a platform, the list is empty.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListAccount {
    inner: ListAccountBuilder,
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
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves the details of an account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveAccount {
    inner: RetrieveAccountBuilder,
    account: stripe_shared::AccountId,
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
#[derive(Clone, Debug, serde::Serialize)]
struct CapabilitiesAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl CapabilitiesAccountBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Returns a list of capabilities associated with the account.
/// The capabilities are returned sorted by creation date, with the most recent capability appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CapabilitiesAccount {
    inner: CapabilitiesAccountBuilder,
    account: stripe_shared::AccountId,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct PersonsAccount {
    inner: PersonsAccountBuilder,
    account: stripe_shared::AccountId,
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
#[derive(Clone, Debug, serde::Serialize)]
struct CreateAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile: Option<CreateAccountBusinessProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_type: Option<stripe_shared::AccountBusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<CapabilitiesParam>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
    /// Whether the business is a minority-owned, women-owned, and/or LGBTQI+-owned business.
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
/// Whether the business is a minority-owned, women-owned, and/or LGBTQI+-owned business.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    LgbtqiOwnedBusiness,
    MinorityOwnedBusiness,
    NoneOfTheseApply,
    PreferNotToAnswer,
    WomenOwnedBusiness,
}
impl CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    pub fn as_str(self) -> &'static str {
        use CreateAccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match self {
            LgbtqiOwnedBusiness => "lgbtqi_owned_business",
            MinorityOwnedBusiness => "minority_owned_business",
            NoneOfTheseApply => "none_of_these_apply",
            PreferNotToAnswer => "prefer_not_to_answer",
            WomenOwnedBusiness => "women_owned_business",
        }
    }
}

impl std::str::FromStr for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match s {
            "lgbtqi_owned_business" => Ok(LgbtqiOwnedBusiness),
            "minority_owned_business" => Ok(MinorityOwnedBusiness),
            "none_of_these_apply" => Ok(NoneOfTheseApply),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "women_owned_business" => Ok(WomenOwnedBusiness),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateAccountBusinessProfileMinorityOwnedBusinessDesignation",
            )
        })
    }
}
/// A publicly available mailing address for sending support issues to.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountBusinessProfileSupportAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
/// Information about the company or business.
/// This field is available for any `business_type`.
/// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
#[derive(Clone, Debug, serde::Serialize)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<RegistrationDateSpecs>,
    /// The identification number given to a company when it is registered or incorporated, if distinct from the identification number used for filing taxes.
    /// (Examples are the CIN for companies and LLP IN for partnerships in India, and the Company Registration Number in Hong Kong).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountCompanyAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountCompanyOwnershipExemptionReason {
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
}
impl CreateAccountCompanyOwnershipExemptionReason {
    pub fn as_str(self) -> &'static str {
        use CreateAccountCompanyOwnershipExemptionReason::*;
        match self {
            QualifiedEntityExceedsOwnershipThreshold => {
                "qualified_entity_exceeds_ownership_threshold"
            }
            QualifiesAsFinancialInstitution => "qualifies_as_financial_institution",
        }
    }
}

impl std::str::FromStr for CreateAccountCompanyOwnershipExemptionReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountCompanyOwnershipExemptionReason::*;
        match s {
            "qualified_entity_exceeds_ownership_threshold" => {
                Ok(QualifiedEntityExceedsOwnershipThreshold)
            }
            "qualifies_as_financial_institution" => Ok(QualifiesAsFinancialInstitution),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateAccountCompanyOwnershipExemptionReason",
            )
        })
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
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for CreateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Ok(Self::from_str(&s).unwrap())
    }
}
/// A hash of configuration describing the account controller's attributes.
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountControllerFees {
    /// A value indicating the responsible payer of Stripe fees on this account.
    /// Defaults to `account`.
    /// Learn more about [fee behavior on connected accounts](https://docs.stripe.com/connect/direct-charges-fee-payer-behavior).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<CreateAccountControllerFeesPayer>,
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountControllerFeesPayer {
    Account,
    Application,
}
impl CreateAccountControllerFeesPayer {
    pub fn as_str(self) -> &'static str {
        use CreateAccountControllerFeesPayer::*;
        match self {
            Account => "account",
            Application => "application",
        }
    }
}

impl std::str::FromStr for CreateAccountControllerFeesPayer {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountControllerFeesPayer::*;
        match s {
            "account" => Ok(Account),
            "application" => Ok(Application),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountControllerFeesPayer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateAccountControllerFeesPayer")
        })
    }
}
/// A hash of configuration for products that have negative balance liability, and whether Stripe or a Connect application is responsible for them.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountControllerLosses {
    /// A value indicating who is liable when this account can't pay back negative balances resulting from payments.
    /// Defaults to `stripe`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<CreateAccountControllerLossesPayments>,
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountControllerLossesPayments {
    Application,
    Stripe,
}
impl CreateAccountControllerLossesPayments {
    pub fn as_str(self) -> &'static str {
        use CreateAccountControllerLossesPayments::*;
        match self {
            Application => "application",
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for CreateAccountControllerLossesPayments {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountControllerLossesPayments::*;
        match s {
            "application" => Ok(Application),
            "stripe" => Ok(Stripe),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountControllerLossesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountControllerLossesPayments {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateAccountControllerLossesPayments")
        })
    }
}
/// A value indicating responsibility for collecting updated information when requirements on the account are due or change.
/// Defaults to `stripe`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountControllerRequirementCollection {
    Application,
    Stripe,
}
impl CreateAccountControllerRequirementCollection {
    pub fn as_str(self) -> &'static str {
        use CreateAccountControllerRequirementCollection::*;
        match self {
            Application => "application",
            Stripe => "stripe",
        }
    }
}

impl std::str::FromStr for CreateAccountControllerRequirementCollection {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountControllerRequirementCollection::*;
        match s {
            "application" => Ok(Application),
            "stripe" => Ok(Stripe),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountControllerRequirementCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountControllerRequirementCollection {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateAccountControllerRequirementCollection",
            )
        })
    }
}
/// A hash of configuration for Stripe-hosted dashboards.
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountControllerStripeDashboard {
    /// Whether this account should have access to the full Stripe Dashboard (`full`), to the Express Dashboard (`express`), or to no Stripe-hosted dashboard (`none`).
    /// Defaults to `full`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<CreateAccountControllerStripeDashboardType>,
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountControllerStripeDashboardType {
    Express,
    Full,
    None,
}
impl CreateAccountControllerStripeDashboardType {
    pub fn as_str(self) -> &'static str {
        use CreateAccountControllerStripeDashboardType::*;
        match self {
            Express => "express",
            Full => "full",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateAccountControllerStripeDashboardType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountControllerStripeDashboardType::*;
        match s {
            "express" => Ok(Express),
            "full" => Ok(Full),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountControllerStripeDashboardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateAccountControllerStripeDashboardType")
        })
    }
}
/// Information about the person represented by the account.
/// This field is null unless `business_type` is set to `individual`.
/// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
#[derive(Clone, Debug, serde::Serialize)]
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountIndividualAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountIndividualPoliticalExposure {
    Existing,
    None,
}
impl CreateAccountIndividualPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        use CreateAccountIndividualPoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
        }
    }
}

impl std::str::FromStr for CreateAccountIndividualPoliticalExposure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountIndividualPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for CreateAccountIndividualPoliticalExposure")
        })
    }
}
/// The individual's registered address.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccountIndividualRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateAccountSettingsInvoices {
    /// Whether payment methods should be saved when a payment is completed for a one-time invoices on a hosted invoice page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_payment_method_save: Option<CreateAccountSettingsInvoicesHostedPaymentMethodSave>,
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
/// Whether payment methods should be saved when a payment is completed for a one-time invoices on a hosted invoice page.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    Always,
    Never,
    Offer,
}
impl CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        use CreateAccountSettingsInvoicesHostedPaymentMethodSave::*;
        match self {
            Always => "always",
            Never => "never",
            Offer => "offer",
        }
    }
}

impl std::str::FromStr for CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountSettingsInvoicesHostedPaymentMethodSave::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            "offer" => Ok(Offer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateAccountSettingsInvoicesHostedPaymentMethodSave",
            )
        })
    }
}
/// Settings specific to the account's payouts.
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    /// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
    /// (required and applicable only if `interval` is `weekly`.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<CreateAccountSettingsPayoutsScheduleWeeklyAnchor>,
}
impl CreateAccountSettingsPayoutsSchedule {
    pub fn new() -> Self {
        Self { delay_days: None, interval: None, monthly_anchor: None, weekly_anchor: None }
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateAccountSettingsPayoutsScheduleDelayDays {
    Minimum,
    #[serde(untagged)]
    U32(u32),
}
/// How frequently available funds are paid out.
/// One of: `daily`, `manual`, `weekly`, or `monthly`.
/// Default is `daily`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountSettingsPayoutsScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
}
impl CreateAccountSettingsPayoutsScheduleInterval {
    pub fn as_str(self) -> &'static str {
        use CreateAccountSettingsPayoutsScheduleInterval::*;
        match self {
            Daily => "daily",
            Manual => "manual",
            Monthly => "monthly",
            Weekly => "weekly",
        }
    }
}

impl std::str::FromStr for CreateAccountSettingsPayoutsScheduleInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountSettingsPayoutsScheduleInterval::*;
        match s {
            "daily" => Ok(Daily),
            "manual" => Ok(Manual),
            "monthly" => Ok(Monthly),
            "weekly" => Ok(Weekly),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateAccountSettingsPayoutsScheduleInterval",
            )
        })
    }
}
/// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
/// (required and applicable only if `interval` is `weekly`.).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
}
impl CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    pub fn as_str(self) -> &'static str {
        use CreateAccountSettingsPayoutsScheduleWeeklyAnchor::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Saturday => "saturday",
            Sunday => "sunday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
        }
    }
}

impl std::str::FromStr for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    type Err = stripe_types::StripeParseError;
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
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for CreateAccountSettingsPayoutsScheduleWeeklyAnchor",
            )
        })
    }
}
/// The type of Stripe account to create. May be one of `custom`, `express` or `standard`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CreateAccountType {
    Custom,
    Express,
    Standard,
}
impl CreateAccountType {
    pub fn as_str(self) -> &'static str {
        use CreateAccountType::*;
        match self {
            Custom => "custom",
            Express => "express",
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for CreateAccountType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use CreateAccountType::*;
        match s {
            "custom" => Ok(Custom),
            "express" => Ok(Express),
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for CreateAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for CreateAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for CreateAccountType"))
    }
}
/// With [Connect](https://stripe.com/docs/connect), you can create Stripe accounts for your users.
/// To do this, you’ll first need to [register your platform](https://dashboard.stripe.com/account/applications/settings).
///
/// If you’ve already collected information for your connected accounts, you [can prefill that information](https://stripe.com/docs/connect/best-practices#onboarding) when.
/// creating the account.
/// Connect Onboarding won’t ask for the prefilled information during account onboarding.
/// You can prefill any information on the account.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateAccount {
    inner: CreateAccountBuilder,
}
impl CreateAccount {
    /// Construct a new `CreateAccount`.
    pub fn new() -> Self {
        Self { inner: CreateAccountBuilder::new() }
    }
    /// An [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account.
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
    pub fn capabilities(mut self, capabilities: impl Into<CapabilitiesParam>) -> Self {
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
    /// The type of Stripe account to create. May be one of `custom`, `express` or `standard`.
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
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_profile: Option<UpdateAccountBusinessProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    business_type: Option<stripe_shared::AccountBusinessType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    capabilities: Option<CapabilitiesParam>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
    /// Whether the business is a minority-owned, women-owned, and/or LGBTQI+-owned business.
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
/// Whether the business is a minority-owned, women-owned, and/or LGBTQI+-owned business.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    LgbtqiOwnedBusiness,
    MinorityOwnedBusiness,
    NoneOfTheseApply,
    PreferNotToAnswer,
    WomenOwnedBusiness,
}
impl UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match self {
            LgbtqiOwnedBusiness => "lgbtqi_owned_business",
            MinorityOwnedBusiness => "minority_owned_business",
            NoneOfTheseApply => "none_of_these_apply",
            PreferNotToAnswer => "prefer_not_to_answer",
            WomenOwnedBusiness => "women_owned_business",
        }
    }
}

impl std::str::FromStr for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation::*;
        match s {
            "lgbtqi_owned_business" => Ok(LgbtqiOwnedBusiness),
            "minority_owned_business" => Ok(MinorityOwnedBusiness),
            "none_of_these_apply" => Ok(NoneOfTheseApply),
            "prefer_not_to_answer" => Ok(PreferNotToAnswer),
            "women_owned_business" => Ok(WomenOwnedBusiness),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateAccountBusinessProfileMinorityOwnedBusinessDesignation",
            )
        })
    }
}
/// A publicly available mailing address for sending support issues to.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateAccountBusinessProfileSupportAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
/// Information about the company or business.
/// This field is available for any `business_type`.
/// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateAccountCompanyAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateAccountCompanyOwnershipExemptionReason {
    QualifiedEntityExceedsOwnershipThreshold,
    QualifiesAsFinancialInstitution,
}
impl UpdateAccountCompanyOwnershipExemptionReason {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountCompanyOwnershipExemptionReason::*;
        match self {
            QualifiedEntityExceedsOwnershipThreshold => {
                "qualified_entity_exceeds_ownership_threshold"
            }
            QualifiesAsFinancialInstitution => "qualifies_as_financial_institution",
        }
    }
}

impl std::str::FromStr for UpdateAccountCompanyOwnershipExemptionReason {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountCompanyOwnershipExemptionReason::*;
        match s {
            "qualified_entity_exceeds_ownership_threshold" => {
                Ok(QualifiedEntityExceedsOwnershipThreshold)
            }
            "qualifies_as_financial_institution" => Ok(QualifiesAsFinancialInstitution),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountCompanyOwnershipExemptionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateAccountCompanyOwnershipExemptionReason",
            )
        })
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
            v => Ok(Unknown(v.to_owned())),
        }
    }
}
impl std::fmt::Display for UpdateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountCompanyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Ok(Self::from_str(&s).unwrap())
    }
}
/// Information about the person represented by the account.
/// This field is null unless `business_type` is set to `individual`.
/// Once you create an [Account Link](/api/account_links) or [Account Session](/api/account_sessions), this property can only be updated for accounts where [controller.requirement_collection](/api/accounts/object#account_object-controller-requirement_collection) is `application`, which includes Custom accounts.
#[derive(Clone, Debug, serde::Serialize)]
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateAccountIndividualAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateAccountIndividualPoliticalExposure {
    Existing,
    None,
}
impl UpdateAccountIndividualPoliticalExposure {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountIndividualPoliticalExposure::*;
        match self {
            Existing => "existing",
            None => "none",
        }
    }
}

impl std::str::FromStr for UpdateAccountIndividualPoliticalExposure {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountIndividualPoliticalExposure::*;
        match s {
            "existing" => Ok(Existing),
            "none" => Ok(None),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountIndividualPoliticalExposure {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for UpdateAccountIndividualPoliticalExposure")
        })
    }
}
/// The individual's registered address.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateAccountIndividualRegisteredAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateAccountSettingsInvoices {
    /// The list of default Account Tax IDs to automatically include on invoices.
    /// Account Tax IDs get added when an invoice is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_account_tax_ids: Option<Vec<String>>,
    /// Whether payment methods should be saved when a payment is completed for a one-time invoices on a hosted invoice page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_payment_method_save: Option<UpdateAccountSettingsInvoicesHostedPaymentMethodSave>,
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
/// Whether payment methods should be saved when a payment is completed for a one-time invoices on a hosted invoice page.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    Always,
    Never,
    Offer,
}
impl UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountSettingsInvoicesHostedPaymentMethodSave::*;
        match self {
            Always => "always",
            Never => "never",
            Offer => "offer",
        }
    }
}

impl std::str::FromStr for UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountSettingsInvoicesHostedPaymentMethodSave::*;
        match s {
            "always" => Ok(Always),
            "never" => Ok(Never),
            "offer" => Ok(Offer),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountSettingsInvoicesHostedPaymentMethodSave {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateAccountSettingsInvoicesHostedPaymentMethodSave",
            )
        })
    }
}
/// Settings specific to the account's payouts.
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
    /// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
    /// (required and applicable only if `interval` is `weekly`.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_anchor: Option<UpdateAccountSettingsPayoutsScheduleWeeklyAnchor>,
}
impl UpdateAccountSettingsPayoutsSchedule {
    pub fn new() -> Self {
        Self { delay_days: None, interval: None, monthly_anchor: None, weekly_anchor: None }
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UpdateAccountSettingsPayoutsScheduleDelayDays {
    Minimum,
    #[serde(untagged)]
    U32(u32),
}
/// How frequently available funds are paid out.
/// One of: `daily`, `manual`, `weekly`, or `monthly`.
/// Default is `daily`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateAccountSettingsPayoutsScheduleInterval {
    Daily,
    Manual,
    Monthly,
    Weekly,
}
impl UpdateAccountSettingsPayoutsScheduleInterval {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountSettingsPayoutsScheduleInterval::*;
        match self {
            Daily => "daily",
            Manual => "manual",
            Monthly => "monthly",
            Weekly => "weekly",
        }
    }
}

impl std::str::FromStr for UpdateAccountSettingsPayoutsScheduleInterval {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateAccountSettingsPayoutsScheduleInterval::*;
        match s {
            "daily" => Ok(Daily),
            "manual" => Ok(Manual),
            "monthly" => Ok(Monthly),
            "weekly" => Ok(Weekly),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountSettingsPayoutsScheduleInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateAccountSettingsPayoutsScheduleInterval",
            )
        })
    }
}
/// The day of the week when available funds are paid out, specified as `monday`, `tuesday`, etc.
/// (required and applicable only if `interval` is `weekly`.).
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    Friday,
    Monday,
    Saturday,
    Sunday,
    Thursday,
    Tuesday,
    Wednesday,
}
impl UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    pub fn as_str(self) -> &'static str {
        use UpdateAccountSettingsPayoutsScheduleWeeklyAnchor::*;
        match self {
            Friday => "friday",
            Monday => "monday",
            Saturday => "saturday",
            Sunday => "sunday",
            Thursday => "thursday",
            Tuesday => "tuesday",
            Wednesday => "wednesday",
        }
    }
}

impl std::str::FromStr for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    type Err = stripe_types::StripeParseError;
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
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for UpdateAccountSettingsPayoutsScheduleWeeklyAnchor",
            )
        })
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateAccount {
    inner: UpdateAccountBuilder,
    account: stripe_shared::AccountId,
}
impl UpdateAccount {
    /// Construct a new `UpdateAccount`.
    pub fn new(account: impl Into<stripe_shared::AccountId>) -> Self {
        Self { account: account.into(), inner: UpdateAccountBuilder::new() }
    }
    /// An [account token](https://stripe.com/docs/api#create_account_token), used to securely provide details to the account.
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
    pub fn capabilities(mut self, capabilities: impl Into<CapabilitiesParam>) -> Self {
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
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
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
#[derive(Clone, Debug, serde::Serialize)]
struct RejectAccountBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    reason: String,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct RejectAccount {
    inner: RejectAccountBuilder,
    account: stripe_shared::AccountId,
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

#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct MonthlyEstimatedRevenueSpecs {
    /// A non-negative integer representing how much to charge in the [smallest currency unit](/currencies#zero-decimal).
    pub amount: i64,
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: stripe_types::Currency,
}
impl MonthlyEstimatedRevenueSpecs {
    pub fn new(amount: impl Into<i64>, currency: impl Into<stripe_types::Currency>) -> Self {
        Self { amount: amount.into(), currency: currency.into() }
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CapabilityParam {
    /// Passing true requests the capability for the account, if it is not already requested.
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}
impl CapabilityParam {
    pub fn new() -> Self {
        Self { requested: None }
    }
}
impl Default for CapabilityParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RegistrationDateSpecs {
    /// The day of registration, between 1 and 31.
    pub day: i64,
    /// The month of registration, between 1 and 12.
    pub month: i64,
    /// The four-digit year of registration.
    pub year: i64,
}
impl RegistrationDateSpecs {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct VerificationDocumentSpecs {
    /// The back of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    /// The front of a document returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `additional_verification`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct DocumentsParam {
    /// One or more document ids returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `account_requirement`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct AccountGroupsSpecs {
    /// The group the account is in to determine their payments pricing, and null if the account is on customized pricing.
    /// [See the Platform pricing tool documentation](https://stripe.com/docs/connect/platform-pricing-tools) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments_pricing: Option<String>,
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct DateOfBirthSpecs {
    /// The day of birth, between 1 and 31.
    pub day: i64,
    /// The month of birth, between 1 and 12.
    pub month: i64,
    /// The four-digit year of birth.
    pub year: i64,
}
impl DateOfBirthSpecs {
    pub fn new(day: impl Into<i64>, month: impl Into<i64>, year: impl Into<i64>) -> Self {
        Self { day: day.into(), month: month.into(), year: year.into() }
    }
}
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct PersonVerificationDocumentSpecs {
    /// The back of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    /// The front of an ID returned by a [file upload](https://stripe.com/docs/api#create_file) with a `purpose` value of `identity_document`.
    /// The uploaded file needs to be a color image (smaller than 8,000px by 8,000px), in JPG, PNG, or PDF format, and less than 10 MB in size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CapabilitiesParam {
    /// The acss_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit_payments: Option<CapabilityParam>,
    /// The affirm_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm_payments: Option<CapabilityParam>,
    /// The afterpay_clearpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay_payments: Option<CapabilityParam>,
    /// The alma_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alma_payments: Option<CapabilityParam>,
    /// The amazon_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amazon_pay_payments: Option<CapabilityParam>,
    /// The au_becs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit_payments: Option<CapabilityParam>,
    /// The bacs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<CapabilityParam>,
    /// The bancontact_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact_payments: Option<CapabilityParam>,
    /// The bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer_payments: Option<CapabilityParam>,
    /// The billie_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billie_payments: Option<CapabilityParam>,
    /// The blik_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik_payments: Option<CapabilityParam>,
    /// The boleto_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto_payments: Option<CapabilityParam>,
    /// The card_issuing capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CapabilityParam>,
    /// The card_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_payments: Option<CapabilityParam>,
    /// The cartes_bancaires_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cartes_bancaires_payments: Option<CapabilityParam>,
    /// The cashapp_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashapp_payments: Option<CapabilityParam>,
    /// The eps_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps_payments: Option<CapabilityParam>,
    /// The fpx_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx_payments: Option<CapabilityParam>,
    /// The gb_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb_bank_transfer_payments: Option<CapabilityParam>,
    /// The giropay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay_payments: Option<CapabilityParam>,
    /// The grabpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay_payments: Option<CapabilityParam>,
    /// The ideal_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal_payments: Option<CapabilityParam>,
    /// The india_international_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub india_international_payments: Option<CapabilityParam>,
    /// The jcb_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb_payments: Option<CapabilityParam>,
    /// The jp_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp_bank_transfer_payments: Option<CapabilityParam>,
    /// The kakao_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kakao_pay_payments: Option<CapabilityParam>,
    /// The klarna_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna_payments: Option<CapabilityParam>,
    /// The konbini_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini_payments: Option<CapabilityParam>,
    /// The kr_card_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr_card_payments: Option<CapabilityParam>,
    /// The legacy_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_payments: Option<CapabilityParam>,
    /// The link_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_payments: Option<CapabilityParam>,
    /// The mobilepay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobilepay_payments: Option<CapabilityParam>,
    /// The multibanco_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco_payments: Option<CapabilityParam>,
    /// The mx_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx_bank_transfer_payments: Option<CapabilityParam>,
    /// The naver_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub naver_pay_payments: Option<CapabilityParam>,
    /// The nz_bank_account_becs_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz_bank_account_becs_debit_payments: Option<CapabilityParam>,
    /// The oxxo_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo_payments: Option<CapabilityParam>,
    /// The p24_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24_payments: Option<CapabilityParam>,
    /// The pay_by_bank_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_by_bank_payments: Option<CapabilityParam>,
    /// The payco_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payco_payments: Option<CapabilityParam>,
    /// The paynow_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow_payments: Option<CapabilityParam>,
    /// The promptpay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay_payments: Option<CapabilityParam>,
    /// The revolut_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revolut_pay_payments: Option<CapabilityParam>,
    /// The samsung_pay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay_payments: Option<CapabilityParam>,
    /// The satispay_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub satispay_payments: Option<CapabilityParam>,
    /// The sepa_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_bank_transfer_payments: Option<CapabilityParam>,
    /// The sepa_debit_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<CapabilityParam>,
    /// The sofort_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort_payments: Option<CapabilityParam>,
    /// The swish_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swish_payments: Option<CapabilityParam>,
    /// The tax_reporting_us_1099_k capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_k: Option<CapabilityParam>,
    /// The tax_reporting_us_1099_misc capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting_us_1099_misc: Option<CapabilityParam>,
    /// The transfers capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<CapabilityParam>,
    /// The treasury capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<CapabilityParam>,
    /// The twint_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twint_payments: Option<CapabilityParam>,
    /// The us_bank_account_ach_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account_ach_payments: Option<CapabilityParam>,
    /// The us_bank_transfer_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_transfer_payments: Option<CapabilityParam>,
    /// The zip_payments capability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_payments: Option<CapabilityParam>,
}
impl CapabilitiesParam {
    pub fn new() -> Self {
        Self {
            acss_debit_payments: None,
            affirm_payments: None,
            afterpay_clearpay_payments: None,
            alma_payments: None,
            amazon_pay_payments: None,
            au_becs_debit_payments: None,
            bacs_debit_payments: None,
            bancontact_payments: None,
            bank_transfer_payments: None,
            billie_payments: None,
            blik_payments: None,
            boleto_payments: None,
            card_issuing: None,
            card_payments: None,
            cartes_bancaires_payments: None,
            cashapp_payments: None,
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
            promptpay_payments: None,
            revolut_pay_payments: None,
            samsung_pay_payments: None,
            satispay_payments: None,
            sepa_bank_transfer_payments: None,
            sepa_debit_payments: None,
            sofort_payments: None,
            swish_payments: None,
            tax_reporting_us_1099_k: None,
            tax_reporting_us_1099_misc: None,
            transfers: None,
            treasury: None,
            twint_payments: None,
            us_bank_account_ach_payments: None,
            us_bank_transfer_payments: None,
            zip_payments: None,
        }
    }
}
impl Default for CapabilitiesParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct VerificationSpecs {
    /// A document verifying the business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<VerificationDocumentSpecs>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
    /// One or more documents showing the company’s proof of registration with the national business registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_registration: Option<DocumentsParam>,
    /// One or more documents that demonstrate proof of ultimate beneficial ownership.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_of_ultimate_beneficial_ownership: Option<DocumentsParam>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct PersonVerificationSpecs {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocumentSpecs>,
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentSpecs>,
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct CardIssuingSettingsSpecs {
    /// Details on the account's acceptance of the [Stripe Issuing Terms and Disclosures](/issuing/connect/tos_acceptance).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<SettingsTermsOfServiceSpecs>,
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
#[derive(Clone, Debug, serde::Serialize)]
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
#[derive(Clone, Debug, serde::Serialize)]
pub struct TreasurySettingsSpecs {
    /// Details on the account's acceptance of the Stripe Treasury Services Agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<SettingsTermsOfServiceSpecs>,
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
