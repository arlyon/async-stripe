#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteRadarValueListItem {}
impl DeleteRadarValueListItem {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteRadarValueListItem {
    /// Deletes a `ValueListItem` object, removing it from its parent value list.
    pub fn send(
        &self,
        client: &stripe::Client,
        item: &stripe_fraud::RadarValueListItemId,
    ) -> stripe::Response<stripe_fraud::DeletedRadarValueListItem> {
        client.send_form(
            &format!("/radar/value_list_items/{item}"),
            self,
            http_types::Method::Delete,
        )
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListRadarValueListItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// A limit on the number of objects to be returned.
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// A cursor for use in pagination.
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<&'a str>,
    /// Return items belonging to the parent list whose value matches the specified value (using an "is like" match).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<&'a str>,
    /// Identifier for the parent value list this item belongs to.
    pub value_list: &'a str,
}
impl<'a> ListRadarValueListItem<'a> {
    pub fn new(value_list: &'a str) -> Self {
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
impl<'a> ListRadarValueListItem<'a> {
    /// Returns a list of `ValueListItem` objects.
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_fraud::RadarValueListItem>> {
        client.get_query("/radar/value_list_items", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_fraud::RadarValueListItem>> {
        stripe::ListPaginator::from_list_params("/radar/value_list_items", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveRadarValueListItem<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveRadarValueListItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveRadarValueListItem<'a> {
    /// Retrieves a `ValueListItem` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        item: &stripe_fraud::RadarValueListItemId,
    ) -> stripe::Response<stripe_fraud::RadarValueListItem> {
        client.get_query(&format!("/radar/value_list_items/{item}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateRadarValueListItem<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The value of the item (whose type must match the type of the parent value list).
    pub value: &'a str,
    /// The identifier of the value list which the created item will be added to.
    pub value_list: &'a str,
}
impl<'a> CreateRadarValueListItem<'a> {
    pub fn new(value: &'a str, value_list: &'a str) -> Self {
        Self { expand: None, value, value_list }
    }
}
impl<'a> CreateRadarValueListItem<'a> {
    /// Creates a new `ValueListItem` object, which is added to the specified parent value list.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_fraud::RadarValueListItem> {
        client.send_form("/radar/value_list_items", self, http_types::Method::Post)
    }
}
