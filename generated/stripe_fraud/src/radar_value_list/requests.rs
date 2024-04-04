#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeleteRadarValueList {}
impl DeleteRadarValueList {
    pub fn new() -> Self {
        Self::default()
    }
}
impl DeleteRadarValueList {
    /// Deletes a `ValueList` object, also deleting any items contained within the value list.
    /// To be deleted, a value list must not be referenced in any rules.
    pub fn send(
        &self,
        client: &stripe::Client,
        value_list: &stripe_fraud::RadarValueListId,
    ) -> stripe::Response<stripe_fraud::DeletedRadarValueList> {
        client.send_form(
            &format!("/radar/value_lists/{value_list}"),
            self,
            http_types::Method::Delete,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListRadarValueList<'a> {
    /// The alias used to reference the value list when writing rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<&'a str>,
    /// A value contained within a value list - returns all value lists containing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<&'a str>,
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
}
impl<'a> ListRadarValueList<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ListRadarValueList<'a> {
    /// Returns a list of `ValueList` objects.
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_fraud::RadarValueList>> {
        client.get_query("/radar/value_lists", self)
    }
    pub fn paginate(
        self,
    ) -> stripe::ListPaginator<stripe_types::List<stripe_fraud::RadarValueList>> {
        stripe::ListPaginator::from_list_params("/radar/value_lists", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveRadarValueList<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveRadarValueList<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveRadarValueList<'a> {
    /// Retrieves a `ValueList` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        value_list: &stripe_fraud::RadarValueListId,
    ) -> stripe::Response<stripe_fraud::RadarValueList> {
        client.get_query(&format!("/radar/value_lists/{value_list}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateRadarValueList<'a> {
    /// The name of the value list for use in rules.
    pub alias: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Type of the items in the value list.
    /// One of `card_fingerprint`, `us_bank_account_fingerprint`, `sepa_debit_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    /// Use `string` if the item type is unknown or mixed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<stripe_fraud::RadarValueListItemType>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The human-readable name of the value list.
    pub name: &'a str,
}
impl<'a> CreateRadarValueList<'a> {
    pub fn new(alias: &'a str, name: &'a str) -> Self {
        Self { alias, expand: None, item_type: None, metadata: None, name }
    }
}
impl<'a> CreateRadarValueList<'a> {
    /// Creates a new `ValueList` object, which can then be referenced in rules.
    pub fn send(&self, client: &stripe::Client) -> stripe::Response<stripe_fraud::RadarValueList> {
        client.send_form("/radar/value_lists", self, http_types::Method::Post)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateRadarValueList<'a> {
    /// The name of the value list for use in rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The human-readable name of the value list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}
impl<'a> UpdateRadarValueList<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateRadarValueList<'a> {
    /// Updates a `ValueList` object by setting the values of the parameters passed.
    /// Any parameters not provided will be left unchanged.
    /// Note that `item_type` is immutable.
    pub fn send(
        &self,
        client: &stripe::Client,
        value_list: &stripe_fraud::RadarValueListId,
    ) -> stripe::Response<stripe_fraud::RadarValueList> {
        client.send_form(
            &format!("/radar/value_lists/{value_list}"),
            self,
            http_types::Method::Post,
        )
    }
}
