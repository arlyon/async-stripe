use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `ValueListItem` object, removing it from its parent value list.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct DeleteRadarValueListItem {
    item: stripe_fraud::RadarValueListItemId,
}
impl DeleteRadarValueListItem {
    /// Construct a new `DeleteRadarValueListItem`.
    pub fn new(item: impl Into<stripe_fraud::RadarValueListItemId>) -> Self {
        Self { item: item.into() }
    }
}
impl DeleteRadarValueListItem {
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

impl StripeRequest for DeleteRadarValueListItem {
    type Output = stripe_fraud::DeletedRadarValueListItem;

    fn build(&self) -> RequestBuilder {
        let item = &self.item;
        RequestBuilder::new(StripeMethod::Delete, format!("/radar/value_list_items/{item}"))
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct ListRadarValueListItemBuilder {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    value_list: String,
}
impl ListRadarValueListItemBuilder {
    fn new(value_list: impl Into<String>) -> Self {
        Self {
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            value: None,
            value_list: value_list.into(),
        }
    }
}
/// Returns a list of `ValueListItem` objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct ListRadarValueListItem {
    inner: ListRadarValueListItemBuilder,
}
impl ListRadarValueListItem {
    /// Construct a new `ListRadarValueListItem`.
    pub fn new(value_list: impl Into<String>) -> Self {
        Self { inner: ListRadarValueListItemBuilder::new(value_list.into()) }
    }
    /// Only return items that were created during the given date interval.
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
    /// Return items belonging to the parent list whose value matches the specified value (using an "is like" match).
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.inner.value = Some(value.into());
        self
    }
}
impl ListRadarValueListItem {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_fraud::RadarValueListItem>>
    {
        stripe_client_core::ListPaginator::new_list("/radar/value_list_items", &self.inner)
    }
}

impl StripeRequest for ListRadarValueListItem {
    type Output = stripe_types::List<stripe_fraud::RadarValueListItem>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/radar/value_list_items").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct RetrieveRadarValueListItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveRadarValueListItemBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `ValueListItem` object.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct RetrieveRadarValueListItem {
    inner: RetrieveRadarValueListItemBuilder,
    item: stripe_fraud::RadarValueListItemId,
}
impl RetrieveRadarValueListItem {
    /// Construct a new `RetrieveRadarValueListItem`.
    pub fn new(item: impl Into<stripe_fraud::RadarValueListItemId>) -> Self {
        Self { item: item.into(), inner: RetrieveRadarValueListItemBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveRadarValueListItem {
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

impl StripeRequest for RetrieveRadarValueListItem {
    type Output = stripe_fraud::RadarValueListItem;

    fn build(&self) -> RequestBuilder {
        let item = &self.item;
        RequestBuilder::new(StripeMethod::Get, format!("/radar/value_list_items/{item}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
struct CreateRadarValueListItemBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    value: String,
    value_list: String,
}
impl CreateRadarValueListItemBuilder {
    fn new(value: impl Into<String>, value_list: impl Into<String>) -> Self {
        Self { expand: None, value: value.into(), value_list: value_list.into() }
    }
}
/// Creates a new `ValueListItem` object, which is added to the specified parent value list.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(feature = "deserialize_extra", derive(serde::Deserialize))]
pub struct CreateRadarValueListItem {
    inner: CreateRadarValueListItemBuilder,
}
impl CreateRadarValueListItem {
    /// Construct a new `CreateRadarValueListItem`.
    pub fn new(value: impl Into<String>, value_list: impl Into<String>) -> Self {
        Self { inner: CreateRadarValueListItemBuilder::new(value.into(), value_list.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl CreateRadarValueListItem {
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

impl StripeRequest for CreateRadarValueListItem {
    type Output = stripe_fraud::RadarValueListItem;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/radar/value_list_items").form(&self.inner)
    }
}
