use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct ListIssuingCardholderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::IssuingCardholderStatus>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::IssuingCardholderType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingCardholderBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingCardholderBuilder").finish_non_exhaustive()
    }
}
impl ListIssuingCardholderBuilder {
    fn new() -> Self {
        Self {
            created: None,
            email: None,
            ending_before: None,
            expand: None,
            limit: None,
            phone_number: None,
            starting_after: None,
            status: None,
            type_: None,
        }
    }
}
/// Returns a list of Issuing `Cardholder` objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct ListIssuingCardholder {
    inner: ListIssuingCardholderBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ListIssuingCardholder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ListIssuingCardholder").finish_non_exhaustive()
    }
}
impl ListIssuingCardholder {
    /// Construct a new `ListIssuingCardholder`.
    pub fn new() -> Self {
        Self { inner: ListIssuingCardholderBuilder::new() }
    }
    /// Only return cardholders that were created during the given date interval.
    pub fn created(mut self, created: impl Into<stripe_types::RangeQueryTs>) -> Self {
        self.inner.created = Some(created.into());
        self
    }
    /// Only return cardholders that have the given email address.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.inner.email = Some(email.into());
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
    /// Only return cardholders that have the given phone number.
    pub fn phone_number(mut self, phone_number: impl Into<String>) -> Self {
        self.inner.phone_number = Some(phone_number.into());
        self
    }
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: impl Into<String>) -> Self {
        self.inner.starting_after = Some(starting_after.into());
        self
    }
    /// Only return cardholders that have the given status. One of `active`, `inactive`, or `blocked`.
    pub fn status(mut self, status: impl Into<stripe_shared::IssuingCardholderStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
    /// Only return cardholders that have the given type. One of `individual` or `company`.
    pub fn type_(mut self, type_: impl Into<stripe_shared::IssuingCardholderType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl Default for ListIssuingCardholder {
    fn default() -> Self {
        Self::new()
    }
}
impl ListIssuingCardholder {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::IssuingCardholder>>
    {
        stripe_client_core::ListPaginator::new_list("/issuing/cardholders", &self.inner)
    }
}

impl StripeRequest for ListIssuingCardholder {
    type Output = stripe_types::List<stripe_shared::IssuingCardholder>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/cardholders").query(&self.inner)
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct RetrieveIssuingCardholderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingCardholderBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingCardholderBuilder").finish_non_exhaustive()
    }
}
impl RetrieveIssuingCardholderBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an Issuing `Cardholder` object.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RetrieveIssuingCardholder {
    inner: RetrieveIssuingCardholderBuilder,
    cardholder: stripe_shared::IssuingCardholderId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RetrieveIssuingCardholder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RetrieveIssuingCardholder").finish_non_exhaustive()
    }
}
impl RetrieveIssuingCardholder {
    /// Construct a new `RetrieveIssuingCardholder`.
    pub fn new(cardholder: impl Into<stripe_shared::IssuingCardholderId>) -> Self {
        Self { cardholder: cardholder.into(), inner: RetrieveIssuingCardholderBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIssuingCardholder {
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

impl StripeRequest for RetrieveIssuingCardholder {
    type Output = stripe_shared::IssuingCardholder;

    fn build(&self) -> RequestBuilder {
        let cardholder = &self.cardholder;
        RequestBuilder::new(StripeMethod::Get, format!("/issuing/cardholders/{cardholder}"))
            .query(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct CreateIssuingCardholderBuilder {
    billing: BillingSpecs,
    #[serde(skip_serializing_if = "Option::is_none")]
    company: Option<CompanyParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    individual: Option<IndividualParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locales: Option<Vec<stripe_shared::PreferredLocales>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spending_controls: Option<AuthorizationControlsParamV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::BillingMeterStatus>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    type_: Option<stripe_shared::IssuingCardholderType>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingCardholderBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingCardholderBuilder").finish_non_exhaustive()
    }
}
impl CreateIssuingCardholderBuilder {
    fn new(billing: impl Into<BillingSpecs>, name: impl Into<String>) -> Self {
        Self {
            billing: billing.into(),
            company: None,
            email: None,
            expand: None,
            individual: None,
            metadata: None,
            name: name.into(),
            phone_number: None,
            preferred_locales: None,
            spending_controls: None,
            status: None,
            type_: None,
        }
    }
}
/// Creates a new Issuing `Cardholder` object that can be issued cards.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CreateIssuingCardholder {
    inner: CreateIssuingCardholderBuilder,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CreateIssuingCardholder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CreateIssuingCardholder").finish_non_exhaustive()
    }
}
impl CreateIssuingCardholder {
    /// Construct a new `CreateIssuingCardholder`.
    pub fn new(billing: impl Into<BillingSpecs>, name: impl Into<String>) -> Self {
        Self { inner: CreateIssuingCardholderBuilder::new(billing.into(), name.into()) }
    }
    /// Additional information about a `company` cardholder.
    pub fn company(mut self, company: impl Into<CompanyParam>) -> Self {
        self.inner.company = Some(company.into());
        self
    }
    /// The cardholder's email address.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.inner.email = Some(email.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Additional information about an `individual` cardholder.
    pub fn individual(mut self, individual: impl Into<IndividualParam>) -> Self {
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
    /// The cardholder's phone number.
    /// This will be transformed to [E.164](https://en.wikipedia.org/wiki/E.164) if it is not provided in that format already.
    /// This is required for all cardholders who will be creating EU cards.
    /// See the [3D Secure documentation](https://docs.stripe.com/issuing/3d-secure#when-is-3d-secure-applied) for more details.
    pub fn phone_number(mut self, phone_number: impl Into<String>) -> Self {
        self.inner.phone_number = Some(phone_number.into());
        self
    }
    /// The cardholder’s preferred locales (languages), ordered by preference.
    /// Locales can be `de`, `en`, `es`, `fr`, or `it`.
    /// This changes the language of the [3D Secure flow](https://docs.stripe.com/issuing/3d-secure) and one-time password messages sent to the cardholder.
    pub fn preferred_locales(
        mut self,
        preferred_locales: impl Into<Vec<stripe_shared::PreferredLocales>>,
    ) -> Self {
        self.inner.preferred_locales = Some(preferred_locales.into());
        self
    }
    /// Rules that control spending across this cardholder's cards.
    /// Refer to our [documentation](https://docs.stripe.com/issuing/controls/spending-controls) for more details.
    pub fn spending_controls(
        mut self,
        spending_controls: impl Into<AuthorizationControlsParamV2>,
    ) -> Self {
        self.inner.spending_controls = Some(spending_controls.into());
        self
    }
    /// Specifies whether to permit authorizations on this cardholder's cards. Defaults to `active`.
    pub fn status(mut self, status: impl Into<stripe_shared::BillingMeterStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
    /// One of `individual` or `company`.
    /// See [Choose a cardholder type](https://docs.stripe.com/issuing/other/choose-cardholder) for more details.
    pub fn type_(mut self, type_: impl Into<stripe_shared::IssuingCardholderType>) -> Self {
        self.inner.type_ = Some(type_.into());
        self
    }
}
impl CreateIssuingCardholder {
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

impl StripeRequest for CreateIssuingCardholder {
    type Output = stripe_shared::IssuingCardholder;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/issuing/cardholders").form(&self.inner)
    }
}
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
struct UpdateIssuingCardholderBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    billing: Option<BillingSpecs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    company: Option<CompanyParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    individual: Option<IndividualParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preferred_locales: Option<Vec<stripe_shared::PreferredLocales>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    spending_controls: Option<AuthorizationControlsParamV2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<stripe_shared::BillingMeterStatus>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingCardholderBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingCardholderBuilder").finish_non_exhaustive()
    }
}
impl UpdateIssuingCardholderBuilder {
    fn new() -> Self {
        Self {
            billing: None,
            company: None,
            email: None,
            expand: None,
            individual: None,
            metadata: None,
            phone_number: None,
            preferred_locales: None,
            spending_controls: None,
            status: None,
        }
    }
}
/// Updates the specified Issuing `Cardholder` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct UpdateIssuingCardholder {
    inner: UpdateIssuingCardholderBuilder,
    cardholder: stripe_shared::IssuingCardholderId,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for UpdateIssuingCardholder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("UpdateIssuingCardholder").finish_non_exhaustive()
    }
}
impl UpdateIssuingCardholder {
    /// Construct a new `UpdateIssuingCardholder`.
    pub fn new(cardholder: impl Into<stripe_shared::IssuingCardholderId>) -> Self {
        Self { cardholder: cardholder.into(), inner: UpdateIssuingCardholderBuilder::new() }
    }
    /// The cardholder's billing address.
    pub fn billing(mut self, billing: impl Into<BillingSpecs>) -> Self {
        self.inner.billing = Some(billing.into());
        self
    }
    /// Additional information about a `company` cardholder.
    pub fn company(mut self, company: impl Into<CompanyParam>) -> Self {
        self.inner.company = Some(company.into());
        self
    }
    /// The cardholder's email address.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.inner.email = Some(email.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Additional information about an `individual` cardholder.
    pub fn individual(mut self, individual: impl Into<IndividualParam>) -> Self {
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
    /// The cardholder's phone number.
    /// This is required for all cardholders who will be creating EU cards.
    /// See the [3D Secure documentation](https://docs.stripe.com/issuing/3d-secure) for more details.
    pub fn phone_number(mut self, phone_number: impl Into<String>) -> Self {
        self.inner.phone_number = Some(phone_number.into());
        self
    }
    /// The cardholder’s preferred locales (languages), ordered by preference.
    /// Locales can be `de`, `en`, `es`, `fr`, or `it`.
    /// This changes the language of the [3D Secure flow](https://docs.stripe.com/issuing/3d-secure) and one-time password messages sent to the cardholder.
    pub fn preferred_locales(
        mut self,
        preferred_locales: impl Into<Vec<stripe_shared::PreferredLocales>>,
    ) -> Self {
        self.inner.preferred_locales = Some(preferred_locales.into());
        self
    }
    /// Rules that control spending across this cardholder's cards.
    /// Refer to our [documentation](https://docs.stripe.com/issuing/controls/spending-controls) for more details.
    pub fn spending_controls(
        mut self,
        spending_controls: impl Into<AuthorizationControlsParamV2>,
    ) -> Self {
        self.inner.spending_controls = Some(spending_controls.into());
        self
    }
    /// Specifies whether to permit authorizations on this cardholder's cards.
    pub fn status(mut self, status: impl Into<stripe_shared::BillingMeterStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl UpdateIssuingCardholder {
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

impl StripeRequest for UpdateIssuingCardholder {
    type Output = stripe_shared::IssuingCardholder;

    fn build(&self) -> RequestBuilder {
        let cardholder = &self.cardholder;
        RequestBuilder::new(StripeMethod::Post, format!("/issuing/cardholders/{cardholder}"))
            .form(&self.inner)
    }
}

#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct RequiredAddress {
    /// City, district, suburb, town, or village.
    pub city: String,
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// Address line 1, such as the street, PO Box, or company name.
    pub line1: String,
    /// Address line 2, such as the apartment, suite, unit, or building.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    /// ZIP or postal code.
    pub postal_code: String,
    /// State, county, province, or region ([ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for RequiredAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RequiredAddress").finish_non_exhaustive()
    }
}
impl RequiredAddress {
    pub fn new(
        city: impl Into<String>,
        country: impl Into<String>,
        line1: impl Into<String>,
        postal_code: impl Into<String>,
    ) -> Self {
        Self {
            city: city.into(),
            country: country.into(),
            line1: line1.into(),
            line2: None,
            postal_code: postal_code.into(),
            state: None,
        }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CompanyParam {
    /// The entity's business ID number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CompanyParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CompanyParam").finish_non_exhaustive()
    }
}
impl CompanyParam {
    pub fn new() -> Self {
        Self { tax_id: None }
    }
}
impl Default for CompanyParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct TermsAcceptanceParam {
    /// The Unix timestamp marking when the cardholder accepted the Authorized User Terms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<stripe_types::Timestamp>,
    /// The IP address from which the cardholder accepted the Authorized User Terms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// The user agent of the browser from which the cardholder accepted the Authorized User Terms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TermsAcceptanceParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TermsAcceptanceParam").finish_non_exhaustive()
    }
}
impl TermsAcceptanceParam {
    pub fn new() -> Self {
        Self { date: None, ip: None, user_agent: None }
    }
}
impl Default for TermsAcceptanceParam {
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
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PersonVerificationDocumentParam {
    /// The back of an ID returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<String>,
    /// The front of an ID returned by a [file upload](https://api.stripe.com#create_file) with a `purpose` value of `identity_document`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonVerificationDocumentParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonVerificationDocumentParam").finish_non_exhaustive()
    }
}
impl PersonVerificationDocumentParam {
    pub fn new() -> Self {
        Self { back: None, front: None }
    }
}
impl Default for PersonVerificationDocumentParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct SpendingLimitsParam {
    /// Maximum amount allowed to spend per interval.
    pub amount: i64,
    /// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    /// Omitting this field will apply the limit to all categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>,
    /// Interval (or event) to which the amount applies.
    pub interval: stripe_shared::IssuingCardSpendingLimitInterval,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SpendingLimitsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SpendingLimitsParam").finish_non_exhaustive()
    }
}
impl SpendingLimitsParam {
    pub fn new(
        amount: impl Into<i64>,
        interval: impl Into<stripe_shared::IssuingCardSpendingLimitInterval>,
    ) -> Self {
        Self { amount: amount.into(), categories: None, interval: interval.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct BillingSpecs {
    /// The cardholder’s billing address.
    pub address: RequiredAddress,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for BillingSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("BillingSpecs").finish_non_exhaustive()
    }
}
impl BillingSpecs {
    pub fn new(address: impl Into<RequiredAddress>) -> Self {
        Self { address: address.into() }
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct CardIssuingParam {
    /// Information about cardholder acceptance of Celtic [Authorized User Terms](https://docs.stripe.com/issuing/compliance-us#issuing-terms).
    /// Required for cards backed by a Celtic program.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_terms_acceptance: Option<TermsAcceptanceParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for CardIssuingParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CardIssuingParam").finish_non_exhaustive()
    }
}
impl CardIssuingParam {
    pub fn new() -> Self {
        Self { user_terms_acceptance: None }
    }
}
impl Default for CardIssuingParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct PersonVerificationParam {
    /// An identifying document, either a passport or local ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocumentParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonVerificationParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonVerificationParam").finish_non_exhaustive()
    }
}
impl PersonVerificationParam {
    pub fn new() -> Self {
        Self { document: None }
    }
}
impl Default for PersonVerificationParam {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct AuthorizationControlsParamV2 {
    /// Array of card presence statuses from which authorizations will be allowed.
    /// Possible options are `present`, `not_present`.
    /// All other statuses will be blocked.
    /// Cannot be set with `blocked_card_presences`.
    /// Provide an empty value to unset this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_card_presences: Option<Vec<stripe_shared::IssuingAuthorizationCardPresence>>,
    /// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
    /// All other categories will be blocked.
    /// Cannot be set with `blocked_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>,
    /// Array of strings containing representing countries from which authorizations will be allowed.
    /// Authorizations from merchants in all other countries will be declined.
    /// Country codes should be ISO 3166 alpha-2 country codes (e.g.
    /// `US`).
    /// Cannot be set with `blocked_merchant_countries`.
    /// Provide an empty value to unset this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_merchant_countries: Option<Vec<String>>,
    /// Array of card presence statuses from which authorizations will be declined.
    /// Possible options are `present`, `not_present`.
    /// Cannot be set with `allowed_card_presences`.
    /// Provide an empty value to unset this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_card_presences: Option<Vec<stripe_shared::IssuingAuthorizationCardPresence>>,
    /// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
    /// All other categories will be allowed.
    /// Cannot be set with `allowed_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<stripe_shared::IssuingCardSpendingLimitCategories>>,
    /// Array of strings containing representing countries from which authorizations will be declined.
    /// Country codes should be ISO 3166 alpha-2 country codes (e.g.
    /// `US`).
    /// Cannot be set with `allowed_merchant_countries`.
    /// Provide an empty value to unset this control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_merchant_countries: Option<Vec<String>>,
    /// Limit spending with amount-based rules that apply across this cardholder's cards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<SpendingLimitsParam>>,
    /// Currency of amounts within `spending_limits`. Defaults to your merchant country's currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits_currency: Option<stripe_types::Currency>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for AuthorizationControlsParamV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("AuthorizationControlsParamV2").finish_non_exhaustive()
    }
}
impl AuthorizationControlsParamV2 {
    pub fn new() -> Self {
        Self {
            allowed_card_presences: None,
            allowed_categories: None,
            allowed_merchant_countries: None,
            blocked_card_presences: None,
            blocked_categories: None,
            blocked_merchant_countries: None,
            spending_limits: None,
            spending_limits_currency: None,
        }
    }
}
impl Default for AuthorizationControlsParamV2 {
    fn default() -> Self {
        Self::new()
    }
}
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[derive(serde::Serialize)]
pub struct IndividualParam {
    /// Information related to the card_issuing program for this cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<CardIssuingParam>,
    /// The date of birth of this cardholder. Cardholders must be older than 13 years old.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<DateOfBirthSpecs>,
    /// The first name of this cardholder.
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The last name of this cardholder.
    /// Required before activating Cards.
    /// This field cannot contain any numbers, special characters (except periods, commas, hyphens, spaces and apostrophes) or non-latin letters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Government-issued ID document for this cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerificationParam>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IndividualParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndividualParam").finish_non_exhaustive()
    }
}
impl IndividualParam {
    pub fn new() -> Self {
        Self {
            card_issuing: None,
            dob: None,
            first_name: None,
            last_name: None,
            verification: None,
        }
    }
}
impl Default for IndividualParam {
    fn default() -> Self {
        Self::new()
    }
}
/// Only return cardholders that have the given status. One of `active`, `inactive`, or `blocked`.
pub type ListIssuingCardholderStatus = stripe_shared::IssuingCardholderStatus;
/// Only return cardholders that have the given type. One of `individual` or `company`.
pub type ListIssuingCardholderType = stripe_shared::IssuingCardholderType;
/// The cardholder’s preferred locales (languages), ordered by preference.
/// Locales can be `de`, `en`, `es`, `fr`, or `it`.
/// This changes the language of the [3D Secure flow](https://docs.stripe.com/issuing/3d-secure) and one-time password messages sent to the cardholder.
pub type CreateIssuingCardholderPreferredLocales = stripe_shared::PreferredLocales;
/// Array of card presence statuses from which authorizations will be allowed.
/// Possible options are `present`, `not_present`.
/// All other statuses will be blocked.
/// Cannot be set with `blocked_card_presences`.
/// Provide an empty value to unset this control.
pub type CreateIssuingCardholderSpendingControlsAllowedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
/// All other categories will be blocked.
/// Cannot be set with `blocked_categories`.
pub type CreateIssuingCardholderSpendingControlsAllowedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Array of card presence statuses from which authorizations will be declined.
/// Possible options are `present`, `not_present`.
/// Cannot be set with `allowed_card_presences`.
/// Provide an empty value to unset this control.
pub type CreateIssuingCardholderSpendingControlsBlockedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
/// All other categories will be allowed.
/// Cannot be set with `allowed_categories`.
pub type CreateIssuingCardholderSpendingControlsBlockedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) this limit applies to.
/// Omitting this field will apply the limit to all categories.
pub type CreateIssuingCardholderSpendingControlsSpendingLimitsCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Interval (or event) to which the amount applies.
pub type CreateIssuingCardholderSpendingControlsSpendingLimitsInterval =
    stripe_shared::IssuingCardSpendingLimitInterval;
/// Specifies whether to permit authorizations on this cardholder's cards. Defaults to `active`.
pub type CreateIssuingCardholderStatus = stripe_shared::BillingMeterStatus;
/// One of `individual` or `company`.
/// See [Choose a cardholder type](https://docs.stripe.com/issuing/other/choose-cardholder) for more details.
pub type CreateIssuingCardholderType = stripe_shared::IssuingCardholderType;
/// The cardholder’s preferred locales (languages), ordered by preference.
/// Locales can be `de`, `en`, `es`, `fr`, or `it`.
/// This changes the language of the [3D Secure flow](https://docs.stripe.com/issuing/3d-secure) and one-time password messages sent to the cardholder.
pub type UpdateIssuingCardholderPreferredLocales = stripe_shared::PreferredLocales;
/// Array of card presence statuses from which authorizations will be allowed.
/// Possible options are `present`, `not_present`.
/// All other statuses will be blocked.
/// Cannot be set with `blocked_card_presences`.
/// Provide an empty value to unset this control.
pub type UpdateIssuingCardholderSpendingControlsAllowedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
/// All other categories will be blocked.
/// Cannot be set with `blocked_categories`.
pub type UpdateIssuingCardholderSpendingControlsAllowedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Array of card presence statuses from which authorizations will be declined.
/// Possible options are `present`, `not_present`.
/// Cannot be set with `allowed_card_presences`.
/// Provide an empty value to unset this control.
pub type UpdateIssuingCardholderSpendingControlsBlockedCardPresences =
    stripe_shared::IssuingAuthorizationCardPresence;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
/// All other categories will be allowed.
/// Cannot be set with `allowed_categories`.
pub type UpdateIssuingCardholderSpendingControlsBlockedCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Array of strings containing [categories](https://docs.stripe.com/api#issuing_authorization_object-merchant_data-category) this limit applies to.
/// Omitting this field will apply the limit to all categories.
pub type UpdateIssuingCardholderSpendingControlsSpendingLimitsCategories =
    stripe_shared::IssuingCardSpendingLimitCategories;
/// Interval (or event) to which the amount applies.
pub type UpdateIssuingCardholderSpendingControlsSpendingLimitsInterval =
    stripe_shared::IssuingCardSpendingLimitInterval;
/// Specifies whether to permit authorizations on this cardholder's cards.
pub type UpdateIssuingCardholderStatus = stripe_shared::BillingMeterStatus;
/// The cardholder’s billing address.
pub type CreateIssuingCardholderBillingAddress = RequiredAddress;
/// Additional information about a `company` cardholder.
pub type CreateIssuingCardholderCompany = CompanyParam;
/// Information about cardholder acceptance of Celtic [Authorized User Terms](https://docs.stripe.com/issuing/compliance-us#issuing-terms).
/// Required for cards backed by a Celtic program.
pub type CreateIssuingCardholderIndividualCardIssuingUserTermsAcceptance = TermsAcceptanceParam;
/// The date of birth of this cardholder. Cardholders must be older than 13 years old.
pub type CreateIssuingCardholderIndividualDob = DateOfBirthSpecs;
/// An identifying document, either a passport or local ID card.
pub type CreateIssuingCardholderIndividualVerificationDocument = PersonVerificationDocumentParam;
/// Limit spending with amount-based rules that apply across this cardholder's cards.
pub type CreateIssuingCardholderSpendingControlsSpendingLimits = SpendingLimitsParam;
/// The cardholder’s billing address.
pub type UpdateIssuingCardholderBillingAddress = RequiredAddress;
/// Additional information about a `company` cardholder.
pub type UpdateIssuingCardholderCompany = CompanyParam;
/// Information about cardholder acceptance of Celtic [Authorized User Terms](https://docs.stripe.com/issuing/compliance-us#issuing-terms).
/// Required for cards backed by a Celtic program.
pub type UpdateIssuingCardholderIndividualCardIssuingUserTermsAcceptance = TermsAcceptanceParam;
/// The date of birth of this cardholder. Cardholders must be older than 13 years old.
pub type UpdateIssuingCardholderIndividualDob = DateOfBirthSpecs;
/// An identifying document, either a passport or local ID card.
pub type UpdateIssuingCardholderIndividualVerificationDocument = PersonVerificationDocumentParam;
/// Limit spending with amount-based rules that apply across this cardholder's cards.
pub type UpdateIssuingCardholderSpendingControlsSpendingLimits = SpendingLimitsParam;
/// The cardholder's billing address.
pub type CreateIssuingCardholderBilling = BillingSpecs;
/// Information related to the card_issuing program for this cardholder.
pub type CreateIssuingCardholderIndividualCardIssuing = CardIssuingParam;
/// Government-issued ID document for this cardholder.
pub type CreateIssuingCardholderIndividualVerification = PersonVerificationParam;
/// Rules that control spending across this cardholder's cards.
/// Refer to our [documentation](https://docs.stripe.com/issuing/controls/spending-controls) for more details.
pub type CreateIssuingCardholderSpendingControls = AuthorizationControlsParamV2;
/// The cardholder's billing address.
pub type UpdateIssuingCardholderBilling = BillingSpecs;
/// Information related to the card_issuing program for this cardholder.
pub type UpdateIssuingCardholderIndividualCardIssuing = CardIssuingParam;
/// Government-issued ID document for this cardholder.
pub type UpdateIssuingCardholderIndividualVerification = PersonVerificationParam;
/// Rules that control spending across this cardholder's cards.
/// Refer to our [documentation](https://docs.stripe.com/issuing/controls/spending-controls) for more details.
pub type UpdateIssuingCardholderSpendingControls = AuthorizationControlsParamV2;
/// Additional information about an `individual` cardholder.
pub type CreateIssuingCardholderIndividual = IndividualParam;
/// Additional information about an `individual` cardholder.
pub type UpdateIssuingCardholderIndividual = IndividualParam;
