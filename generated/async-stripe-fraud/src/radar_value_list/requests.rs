use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `ValueList` object, also deleting any items contained within the value list.
/// To be deleted, a value list must not be referenced in any rules.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteRadarValueList<'a> {
    value_list: &'a stripe_fraud::RadarValueListId,
}
impl<'a> DeleteRadarValueList<'a> {
    /// Construct a new `DeleteRadarValueList`.
    pub fn new(value_list: &'a stripe_fraud::RadarValueListId) -> Self {
        Self { value_list }
    }
}
impl DeleteRadarValueList<'_> {
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

impl StripeRequest for DeleteRadarValueList<'_> {
    type Output = stripe_fraud::DeletedRadarValueList;

    fn build(&self) -> RequestBuilder {
        let value_list = self.value_list;
        RequestBuilder::new(StripeMethod::Delete, format!("/radar/value_lists/{value_list}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListRadarValueListBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<stripe_types::RangeQueryTs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ending_before: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    starting_after: Option<&'a str>,
}
impl<'a> ListRadarValueListBuilder<'a> {
    fn new() -> Self {
        Self {
            alias: None,
            contains: None,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
        }
    }
}
/// Returns a list of `ValueList` objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListRadarValueList<'a> {
    inner: ListRadarValueListBuilder<'a>,
}
impl<'a> ListRadarValueList<'a> {
    /// Construct a new `ListRadarValueList`.
    pub fn new() -> Self {
        Self { inner: ListRadarValueListBuilder::new() }
    }
    /// The alias used to reference the value list when writing rules.
    pub fn alias(mut self, alias: &'a str) -> Self {
        self.inner.alias = Some(alias);
        self
    }
    /// A value contained within a value list - returns all value lists containing this value.
    pub fn contains(mut self, contains: &'a str) -> Self {
        self.inner.contains = Some(contains);
        self
    }
    /// Only return value lists that were created during the given date interval.
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
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    pub fn starting_after(mut self, starting_after: &'a str) -> Self {
        self.inner.starting_after = Some(starting_after);
        self
    }
}
impl<'a> Default for ListRadarValueList<'a> {
    fn default() -> Self {
        Self::new()
    }
}
impl ListRadarValueList<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_fraud::RadarValueList>> {
        stripe_client_core::ListPaginator::new_list("/radar/value_lists", self.inner)
    }
}

impl StripeRequest for ListRadarValueList<'_> {
    type Output = stripe_types::List<stripe_fraud::RadarValueList>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/radar/value_lists").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveRadarValueListBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveRadarValueListBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `ValueList` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveRadarValueList<'a> {
    inner: RetrieveRadarValueListBuilder<'a>,
    value_list: &'a stripe_fraud::RadarValueListId,
}
impl<'a> RetrieveRadarValueList<'a> {
    /// Construct a new `RetrieveRadarValueList`.
    pub fn new(value_list: &'a stripe_fraud::RadarValueListId) -> Self {
        Self { value_list, inner: RetrieveRadarValueListBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveRadarValueList<'_> {
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

impl StripeRequest for RetrieveRadarValueList<'_> {
    type Output = stripe_fraud::RadarValueList;

    fn build(&self) -> RequestBuilder {
        let value_list = self.value_list;
        RequestBuilder::new(StripeMethod::Get, format!("/radar/value_lists/{value_list}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateRadarValueListBuilder<'a> {
    alias: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_type: Option<stripe_fraud::RadarValueListItemType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    name: &'a str,
}
impl<'a> CreateRadarValueListBuilder<'a> {
    fn new(alias: &'a str, name: &'a str) -> Self {
        Self { alias, expand: None, item_type: None, metadata: None, name }
    }
}
/// Creates a new `ValueList` object, which can then be referenced in rules.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateRadarValueList<'a> {
    inner: CreateRadarValueListBuilder<'a>,
}
impl<'a> CreateRadarValueList<'a> {
    /// Construct a new `CreateRadarValueList`.
    pub fn new(alias: &'a str, name: &'a str) -> Self {
        Self { inner: CreateRadarValueListBuilder::new(alias, name) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Type of the items in the value list.
    /// One of `card_fingerprint`, `us_bank_account_fingerprint`, `sepa_debit_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    /// Use `string` if the item type is unknown or mixed.
    pub fn item_type(mut self, item_type: stripe_fraud::RadarValueListItemType) -> Self {
        self.inner.item_type = Some(item_type);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
}
impl CreateRadarValueList<'_> {
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

impl StripeRequest for CreateRadarValueList<'_> {
    type Output = stripe_fraud::RadarValueList;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/radar/value_lists").form(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateRadarValueListBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<&'a std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
}
impl<'a> UpdateRadarValueListBuilder<'a> {
    fn new() -> Self {
        Self { alias: None, expand: None, metadata: None, name: None }
    }
}
/// Updates a `ValueList` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
/// Note that `item_type` is immutable.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateRadarValueList<'a> {
    inner: UpdateRadarValueListBuilder<'a>,
    value_list: &'a stripe_fraud::RadarValueListId,
}
impl<'a> UpdateRadarValueList<'a> {
    /// Construct a new `UpdateRadarValueList`.
    pub fn new(value_list: &'a stripe_fraud::RadarValueListId) -> Self {
        Self { value_list, inner: UpdateRadarValueListBuilder::new() }
    }
    /// The name of the value list for use in rules.
    pub fn alias(mut self, alias: &'a str) -> Self {
        self.inner.alias = Some(alias);
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    pub fn metadata(mut self, metadata: &'a std::collections::HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }
    /// The human-readable name of the value list.
    pub fn name(mut self, name: &'a str) -> Self {
        self.inner.name = Some(name);
        self
    }
}
impl UpdateRadarValueList<'_> {
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

impl StripeRequest for UpdateRadarValueList<'_> {
    type Output = stripe_fraud::RadarValueList;

    fn build(&self) -> RequestBuilder {
        let value_list = self.value_list;
        RequestBuilder::new(StripeMethod::Post, format!("/radar/value_lists/{value_list}"))
            .form(&self.inner)
    }
}
