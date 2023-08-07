#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ListIssuingAuthorization<'a> {
    /// Only return authorizations that belong to the given card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<&'a str>,
    /// Only return authorizations that belong to the given cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<&'a str>,
    /// Only return authorizations that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<stripe_types::RangeQueryTs>,
    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<&'a str>,
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
    pub starting_after: Option<&'a str>,
    /// Only return authorizations with the given status.
    ///
    /// One of `pending`, `closed`, or `reversed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListIssuingAuthorizationStatus>,
}
impl<'a> ListIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
/// Only return authorizations with the given status.
///
/// One of `pending`, `closed`, or `reversed`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListIssuingAuthorizationStatus {
    Closed,
    Pending,
    Reversed,
}

impl ListIssuingAuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        use ListIssuingAuthorizationStatus::*;
        match self {
            Closed => "closed",
            Pending => "pending",
            Reversed => "reversed",
        }
    }
}

impl std::str::FromStr for ListIssuingAuthorizationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListIssuingAuthorizationStatus::*;
        match s {
            "closed" => Ok(Closed),
            "pending" => Ok(Pending),
            "reversed" => Ok(Reversed),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListIssuingAuthorizationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListIssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListIssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListIssuingAuthorizationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> stripe::PaginationParams for ListIssuingAuthorization<'a> {}
impl<'a> ListIssuingAuthorization<'a> {
    /// Returns a list of Issuing `Authorization` objects.
    ///
    /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::IssuingAuthorization>> {
        client.get_query("/issuing/authorizations", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::IssuingAuthorization> {
        stripe::ListPaginator::from_params("/issuing/authorizations", self)
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveIssuingAuthorization<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveIssuingAuthorization<'a> {
    /// Retrieves an Issuing `Authorization` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &stripe_types::issuing_authorization::IssuingAuthorizationId,
    ) -> stripe::Response<stripe_types::IssuingAuthorization> {
        client.get_query(
            &format!("/issuing/authorizations/{authorization}", authorization = authorization),
            self,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct UpdateIssuingAuthorization<'a> {
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
}
impl<'a> UpdateIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> UpdateIssuingAuthorization<'a> {
    /// Updates the specified Issuing `Authorization` object by setting the values of the parameters passed.
    ///
    /// Any parameters not provided will be left unchanged.
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &stripe_types::issuing_authorization::IssuingAuthorizationId,
    ) -> stripe::Response<stripe_types::IssuingAuthorization> {
        client.send_form(
            &format!("/issuing/authorizations/{authorization}", authorization = authorization),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct ApproveIssuingAuthorization<'a> {
    /// If the authorization's `pending_request.is_amount_controllable` property is `true`, you may provide this value to control how much to hold for the authorization.
    ///
    /// Must be positive (use [`decline`](https://stripe.com/docs/api/issuing/authorizations/decline) to decline an authorization request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
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
}
impl<'a> ApproveIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> ApproveIssuingAuthorization<'a> {
    /// Approves a pending Issuing `Authorization` object.
    ///
    /// This request should be made within the timeout window of the [real-time authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations) flow.
    /// You can also respond directly to the webhook request to approve an authorization (preferred).
    /// More details can be found [here](https://stripe.com/docs/issuing/controls/real-time-authorizations#authorization-handling).
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &stripe_types::issuing_authorization::IssuingAuthorizationId,
    ) -> stripe::Response<stripe_types::IssuingAuthorization> {
        client.send_form(
            &format!(
                "/issuing/authorizations/{authorization}/approve",
                authorization = authorization
            ),
            self,
            http_types::Method::Post,
        )
    }
}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct DeclineIssuingAuthorization<'a> {
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
}
impl<'a> DeclineIssuingAuthorization<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> DeclineIssuingAuthorization<'a> {
    /// Declines a pending Issuing `Authorization` object.
    ///
    /// This request should be made within the timeout window of the [real time authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations) flow. You can also respond directly to the webhook request to decline an authorization (preferred).
    /// More details can be found [here](https://stripe.com/docs/issuing/controls/real-time-authorizations#authorization-handling).
    pub fn send(
        &self,
        client: &stripe::Client,
        authorization: &stripe_types::issuing_authorization::IssuingAuthorizationId,
    ) -> stripe::Response<stripe_types::IssuingAuthorization> {
        client.send_form(
            &format!(
                "/issuing/authorizations/{authorization}/decline",
                authorization = authorization
            ),
            self,
            http_types::Method::Post,
        )
    }
}
