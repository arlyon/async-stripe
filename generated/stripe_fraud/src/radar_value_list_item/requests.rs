use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

/// Deletes a `ValueListItem` object, removing it from its parent value list.
#[derive(Clone, Debug, serde::Serialize)]
pub struct DeleteRadarValueListItem<'a> {
    item: &'a stripe_fraud::RadarValueListItemId,
}
impl<'a> DeleteRadarValueListItem<'a> {
    /// Construct a new `DeleteRadarValueListItem`.
    pub fn new(item: &'a stripe_fraud::RadarValueListItemId) -> Self {
        Self { item }
    }
}
impl DeleteRadarValueListItem<'_> {
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

impl StripeRequest for DeleteRadarValueListItem<'_> {
    type Output = stripe_fraud::DeletedRadarValueListItem;

    fn build(&self) -> RequestBuilder {
        let item = self.item;
        RequestBuilder::new(StripeMethod::Delete, format!("/radar/value_list_items/{item}"))
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListRadarValueListItemBuilder<'a> {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<&'a str>,
    value_list: &'a str,
}
impl<'a> ListRadarValueListItemBuilder<'a> {
    fn new(value_list: &'a str) -> Self {
        Self {
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            value: None,
            value_list,
        }
    }
}
/// Returns a list of `ValueListItem` objects.
/// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListRadarValueListItem<'a> {
    inner: ListRadarValueListItemBuilder<'a>,
}
impl<'a> ListRadarValueListItem<'a> {
    /// Construct a new `ListRadarValueListItem`.
    pub fn new(value_list: &'a str) -> Self {
        Self { inner: ListRadarValueListItemBuilder::new(value_list) }
    }
    /// Only return items that were created during the given date interval.
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
    /// Return items belonging to the parent list whose value matches the specified value (using an "is like" match).
    pub fn value(mut self, value: &'a str) -> Self {
        self.inner.value = Some(value);
        self
    }
}
impl ListRadarValueListItem<'_> {
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
        stripe_client_core::ListPaginator::new_list("/radar/value_list_items", self.inner)
    }
}

impl StripeRequest for ListRadarValueListItem<'_> {
    type Output = stripe_types::List<stripe_fraud::RadarValueListItem>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/radar/value_list_items").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveRadarValueListItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveRadarValueListItemBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves a `ValueListItem` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveRadarValueListItem<'a> {
    inner: RetrieveRadarValueListItemBuilder<'a>,
    item: &'a stripe_fraud::RadarValueListItemId,
}
impl<'a> RetrieveRadarValueListItem<'a> {
    /// Construct a new `RetrieveRadarValueListItem`.
    pub fn new(item: &'a stripe_fraud::RadarValueListItemId) -> Self {
        Self { item, inner: RetrieveRadarValueListItemBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveRadarValueListItem<'_> {
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

impl StripeRequest for RetrieveRadarValueListItem<'_> {
    type Output = stripe_fraud::RadarValueListItem;

    fn build(&self) -> RequestBuilder {
        let item = self.item;
        RequestBuilder::new(StripeMethod::Get, format!("/radar/value_list_items/{item}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct CreateRadarValueListItemBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    value: &'a str,
    value_list: &'a str,
}
impl<'a> CreateRadarValueListItemBuilder<'a> {
    fn new(value: &'a str, value_list: &'a str) -> Self {
        Self { expand: None, value, value_list }
    }
}
/// Creates a new `ValueListItem` object, which is added to the specified parent value list.
#[derive(Clone, Debug, serde::Serialize)]
pub struct CreateRadarValueListItem<'a> {
    inner: CreateRadarValueListItemBuilder<'a>,
}
impl<'a> CreateRadarValueListItem<'a> {
    /// Construct a new `CreateRadarValueListItem`.
    pub fn new(value: &'a str, value_list: &'a str) -> Self {
        Self { inner: CreateRadarValueListItemBuilder::new(value, value_list) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl CreateRadarValueListItem<'_> {
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

impl StripeRequest for CreateRadarValueListItem<'_> {
    type Output = stripe_fraud::RadarValueListItem;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Post, "/radar/value_list_items").form(&self.inner)
    }
}
