impl stripe_core::radar::value_list::ValueList {
    /// Returns a list of `ValueList` objects.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn list(
        client: &stripe::Client,
        params: ListValueList,
    ) -> stripe::Response<stripe_types::List<stripe_core::radar::value_list::ValueList>> {
        client.get_query("/radar/value_lists", params)
    }
    /// Retrieves a `ValueList` object.
    pub fn retrieve(
        client: &stripe::Client,
        value_list: &str,
        params: RetrieveValueList,
    ) -> stripe::Response<stripe_core::radar::value_list::ValueList> {
        client
            .get_query(&format!("/radar/value_lists/{value_list}", value_list = value_list), params)
    }
    /// Creates a new `ValueList` object, which can then be referenced in rules.
    pub fn create(
        client: &stripe::Client,
        params: CreateValueList,
    ) -> stripe::Response<stripe_core::radar::value_list::ValueList> {
        client.send_form("/radar/value_lists", params, http_types::Method::Post)
    }
    /// Updates a `ValueList` object by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    /// Note that `item_type` is immutable.
    pub fn update(
        client: &stripe::Client,
        value_list: &str,
        params: UpdateValueList,
    ) -> stripe::Response<stripe_core::radar::value_list::ValueList> {
        client.send_form(
            &format!("/radar/value_lists/{value_list}", value_list = value_list),
            params,
            http_types::Method::Post,
        )
    }
    /// Deletes a `ValueList` object, also deleting any items contained within the value list.
    ///
    /// To be deleted, a value list must not be referenced in any rules.
    pub fn delete(
        client: &stripe::Client,
        value_list: &str,
    ) -> stripe::Response<stripe_core::radar::value_list::DeletedValueList> {
        client.send(
            &format!("/radar/value_lists/{value_list}", value_list = value_list),
            http_types::Method::Delete,
        )
    }
}
#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ListValueList<'a> {
    /// The alias used to reference the value list when writing rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<&'a str>,
    /// A value contained within a value list - returns all value lists containing this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<&'a str>,
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
}
impl<'a> ListValueList<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveValueList<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveValueList<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct CreateValueList<'a> {
    /// The name of the value list for use in rules.
    pub alias: &'a str,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Type of the items in the value list.
    ///
    /// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    /// Use `string` if the item type is unknown or mixed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<CreateValueListItemType>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The human-readable name of the value list.
    pub name: &'a str,
}
impl<'a> CreateValueList<'a> {
    pub fn new(alias: &'a str, name: &'a str) -> Self {
        Self {
            alias,
            expand: Default::default(),
            item_type: Default::default(),
            metadata: Default::default(),
            name,
        }
    }
}
/// Type of the items in the value list.
///
/// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
/// Use `string` if the item type is unknown or mixed.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateValueListItemType {
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CustomerId,
    Email,
    IpAddress,
    String,
}

impl CreateValueListItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardBin => "card_bin",
            Self::CardFingerprint => "card_fingerprint",
            Self::CaseSensitiveString => "case_sensitive_string",
            Self::Country => "country",
            Self::CustomerId => "customer_id",
            Self::Email => "email",
            Self::IpAddress => "ip_address",
            Self::String => "string",
        }
    }
}

impl std::str::FromStr for CreateValueListItemType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "card_bin" => Ok(Self::CardBin),
            "card_fingerprint" => Ok(Self::CardFingerprint),
            "case_sensitive_string" => Ok(Self::CaseSensitiveString),
            "country" => Ok(Self::Country),
            "customer_id" => Ok(Self::CustomerId),
            "email" => Ok(Self::Email),
            "ip_address" => Ok(Self::IpAddress),
            "string" => Ok(Self::String),

            _ => Err(()),
        }
    }
}

impl AsRef<str> for CreateValueListItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for CreateValueListItemType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateValueList<'a> {
    /// The name of the value list for use in rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<&'a str>,
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<&'a std::collections::HashMap<String, String>>,
    /// The human-readable name of the value list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<&'a str>,
}
impl<'a> UpdateValueList<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
