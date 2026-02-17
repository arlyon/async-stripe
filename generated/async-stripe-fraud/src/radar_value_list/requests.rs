use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `ValueList` object, also deleting any items contained within the value list.
/// To be deleted, a value list must not be referenced in any rules.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteRadarValueList {
    value_list: stripe_fraud::RadarValueListId,
}
impl DeleteRadarValueList {
    /// Construct a new `DeleteRadarValueList`.
    pub fn new(value_list: impl Into<stripe_fraud::RadarValueListId>) -> Self {
        Self { value_list: value_list.into() }
    }
}
impl DeleteRadarValueList {
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

impl StripeRequest for DeleteRadarValueList {
    type Output = stripe_fraud::DeletedRadarValueList;

    fn build(&self) -> RequestBuilder {
        let value_list = &self.value_list;
        RequestBuilder::new(StripeMethod::Delete, format!("/radar/value_lists/{value_list}"))
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct ListRadarValueListBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contains: Option<String>,
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
impl ListRadarValueListBuilder {
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
pub struct ListRadarValueList {
    inner: ListRadarValueListBuilder,
}
impl ListRadarValueList {
    /// Construct a new `ListRadarValueList`.
    pub fn new() -> Self {
        Self { inner: ListRadarValueListBuilder::new() }
    }
    /// The alias used to reference the value list when writing rules.
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.inner.alias = Some(alias.into());
        self
    }
    /// A value contained within a value list - returns all value lists containing this value.
    pub fn contains(mut self, contains: impl Into<String>) -> Self {
        self.inner.contains = Some(contains.into());
        self
    }
    /// Only return value lists that were created during the given date interval.
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
impl Default for ListRadarValueList {
    fn default() -> Self {
        Self::new()
    }
}
impl ListRadarValueList {
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
        stripe_client_core::ListPaginator::new_list("/radar/value_lists", &self.inner)
    }
}

impl StripeRequest for ListRadarValueList {
    type Output = stripe_types::List<stripe_fraud::RadarValueList>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/radar/value_lists").query(&self.inner)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
struct RetrieveRadarValueListBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveRadarValueListBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `ValueList` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveRadarValueList {
    inner: RetrieveRadarValueListBuilder,
    value_list: stripe_fraud::RadarValueListId,
}
impl RetrieveRadarValueList {
    /// Construct a new `RetrieveRadarValueList`.
    pub fn new(value_list: impl Into<stripe_fraud::RadarValueListId>) -> Self {
        Self { value_list: value_list.into(), inner: RetrieveRadarValueListBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveRadarValueList {
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

impl StripeRequest for RetrieveRadarValueList {
    type Output = stripe_fraud::RadarValueList;

    fn build(&self) -> RequestBuilder {
        let value_list = &self.value_list;
        RequestBuilder::new(StripeMethod::Get, format!("/radar/value_lists/{value_list}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct CreateRadarValueListBuilder {
    alias: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    item_type: Option<stripe_fraud::RadarValueListItemType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    name: String,
}
impl CreateRadarValueListBuilder {
    fn new(alias: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            alias: alias.into(),
            expand: None,
            item_type: None,
            metadata: None,
            name: name.into(),
        }
    }
}
/// Creates a new `ValueList` object, which can then be referenced in rules.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateRadarValueList {
    inner: CreateRadarValueListBuilder,
}
impl CreateRadarValueList {
    /// Construct a new `CreateRadarValueList`.
    pub fn new(alias: impl Into<String>, name: impl Into<String>) -> Self {
        Self { inner: CreateRadarValueListBuilder::new(alias.into(), name.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
    /// Type of the items in the value list.
    /// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, `customer_id`, `sepa_debit_fingerprint`, or `us_bank_account_fingerprint`.
    /// Use `string` if the item type is unknown or mixed.
    pub fn item_type(mut self, item_type: impl Into<stripe_fraud::RadarValueListItemType>) -> Self {
        self.inner.item_type = Some(item_type.into());
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
}
impl CreateRadarValueList {
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

impl StripeRequest for CreateRadarValueList {
    type Output = stripe_fraud::RadarValueList;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/radar/value_lists").form(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateRadarValueListBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}
impl UpdateRadarValueListBuilder {
    fn new() -> Self {
        Self { alias: None, expand: None, metadata: None, name: None }
    }
}
/// Updates a `ValueList` object by setting the values of the parameters passed.
/// Any parameters not provided will be left unchanged.
/// Note that `item_type` is immutable.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateRadarValueList {
    inner: UpdateRadarValueListBuilder,
    value_list: stripe_fraud::RadarValueListId,
}
impl UpdateRadarValueList {
    /// Construct a new `UpdateRadarValueList`.
    pub fn new(value_list: impl Into<stripe_fraud::RadarValueListId>) -> Self {
        Self { value_list: value_list.into(), inner: UpdateRadarValueListBuilder::new() }
    }
    /// The name of the value list for use in rules.
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.inner.alias = Some(alias.into());
        self
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
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
    /// The human-readable name of the value list.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.inner.name = Some(name.into());
        self
    }
}
impl UpdateRadarValueList {
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

impl StripeRequest for UpdateRadarValueList {
    type Output = stripe_fraud::RadarValueList;

    fn build(&self) -> RequestBuilder {
        let value_list = &self.value_list;
        RequestBuilder::new(StripeMethod::Post, format!("/radar/value_lists/{value_list}"))
            .form(&self.inner)
    }
}
