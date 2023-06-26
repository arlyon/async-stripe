impl stripe_core::radar::value_list_item::ValueListItem {
    /// Returns a list of `ValueListItem` objects.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn list(
        client: &stripe::Client,
        params: ListValueListItem,
    ) -> stripe::Response<stripe_types::List<stripe_core::radar::value_list_item::ValueListItem>>
    {
        client.get_query("/radar/value_list_items", params)
    }
    /// Retrieves a `ValueListItem` object.
    pub fn retrieve(
        client: &stripe::Client,
        item: &stripe_core::line_item::ItemId,
        params: RetrieveValueListItem,
    ) -> stripe::Response<stripe_core::radar::value_list_item::ValueListItem> {
        client.get_query(&format!("/radar/value_list_items/{item}", item = item), params)
    }
    /// Creates a new `ValueListItem` object, which is added to the specified parent value list.
    pub fn create(
        client: &stripe::Client,
        params: CreateValueListItem,
    ) -> stripe::Response<stripe_core::radar::value_list_item::ValueListItem> {
        client.send_form("/radar/value_list_items", params, http_types::Method::Post)
    }
    /// Deletes a `ValueListItem` object, removing it from its parent value list.
    pub fn delete(
        client: &stripe::Client,
        item: &stripe_core::line_item::ItemId,
    ) -> stripe::Response<stripe_core::radar::value_list_item::DeletedValueListItem> {
        client.send(
            &format!("/radar/value_list_items/{item}", item = item),
            http_types::Method::Delete,
        )
    }
}
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListValueListItem<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,
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
    pub starting_after: Option<String>,
    /// Return items belonging to the parent list whose value matches the specified value (using an "is like" match).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<&'a str>,
    /// Identifier for the parent value list this item belongs to.
    pub value_list: &'a str,
}
impl<'a> ListValueListItem<'a> {
    pub fn new(value_list: &'a str) -> Self {
        Self {
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            value: Default::default(),
            value_list,
        }
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveValueListItem<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveValueListItem<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateValueListItem<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// The value of the item (whose type must match the type of the parent value list).
    pub value: &'a str,
    /// The identifier of the value list which the created item will be added to.
    pub value_list: &'a str,
}
impl<'a> CreateValueListItem<'a> {
    pub fn new(value: &'a str, value_list: &'a str) -> Self {
        Self { expand: Default::default(), value, value_list }
    }
}
