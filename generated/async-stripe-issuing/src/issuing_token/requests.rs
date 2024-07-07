use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Copy, Clone, Debug, serde::Serialize)]
struct ListIssuingTokenBuilder<'a> {
    card: &'a str,
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
    status: Option<stripe_shared::IssuingTokenStatus>,
}
impl<'a> ListIssuingTokenBuilder<'a> {
    fn new(card: &'a str) -> Self {
        Self {
            card,
            created: None,
            ending_before: None,
            expand: None,
            limit: None,
            starting_after: None,
            status: None,
        }
    }
}
/// Lists all Issuing `Token` objects for a given card.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ListIssuingToken<'a> {
    inner: ListIssuingTokenBuilder<'a>,
}
impl<'a> ListIssuingToken<'a> {
    /// Construct a new `ListIssuingToken`.
    pub fn new(card: &'a str) -> Self {
        Self { inner: ListIssuingTokenBuilder::new(card) }
    }
    /// Only return Issuing tokens that were created during the given date interval.
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
    /// Select Issuing tokens with the given status.
    pub fn status(mut self, status: stripe_shared::IssuingTokenStatus) -> Self {
        self.inner.status = Some(status);
        self
    }
}
impl ListIssuingToken<'_> {
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
    ) -> stripe_client_core::ListPaginator<stripe_types::List<stripe_shared::IssuingToken>> {
        stripe_client_core::ListPaginator::new_list("/issuing/tokens", self.inner)
    }
}

impl StripeRequest for ListIssuingToken<'_> {
    type Output = stripe_types::List<stripe_shared::IssuingToken>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/tokens").query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct RetrieveIssuingTokenBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIssuingTokenBuilder<'a> {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an Issuing `Token` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveIssuingToken<'a> {
    inner: RetrieveIssuingTokenBuilder<'a>,
    token: &'a stripe_shared::IssuingTokenId,
}
impl<'a> RetrieveIssuingToken<'a> {
    /// Construct a new `RetrieveIssuingToken`.
    pub fn new(token: &'a stripe_shared::IssuingTokenId) -> Self {
        Self { token, inner: RetrieveIssuingTokenBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl RetrieveIssuingToken<'_> {
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

impl StripeRequest for RetrieveIssuingToken<'_> {
    type Output = stripe_shared::IssuingToken;

    fn build(&self) -> RequestBuilder {
        let token = self.token;
        RequestBuilder::new(StripeMethod::Get, format!("/issuing/tokens/{token}"))
            .query(&self.inner)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
struct UpdateIssuingTokenBuilder<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<&'a [&'a str]>,
    status: UpdateIssuingTokenStatus,
}
impl<'a> UpdateIssuingTokenBuilder<'a> {
    fn new(status: UpdateIssuingTokenStatus) -> Self {
        Self { expand: None, status }
    }
}
/// Specifies which status the token should be updated to.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIssuingTokenStatus {
    Active,
    Deleted,
    Suspended,
}
impl UpdateIssuingTokenStatus {
    pub fn as_str(self) -> &'static str {
        use UpdateIssuingTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Suspended => "suspended",
        }
    }
}

impl std::str::FromStr for UpdateIssuingTokenStatus {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "suspended" => Ok(Suspended),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for UpdateIssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIssuingTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIssuingTokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for UpdateIssuingTokenStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s)
            .map_err(|_| serde::de::Error::custom("Unknown value for UpdateIssuingTokenStatus"))
    }
}
/// Attempts to update the specified Issuing `Token` object to the status specified.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateIssuingToken<'a> {
    inner: UpdateIssuingTokenBuilder<'a>,
    token: &'a stripe_shared::IssuingTokenId,
}
impl<'a> UpdateIssuingToken<'a> {
    /// Construct a new `UpdateIssuingToken`.
    pub fn new(token: &'a stripe_shared::IssuingTokenId, status: UpdateIssuingTokenStatus) -> Self {
        Self { token, inner: UpdateIssuingTokenBuilder::new(status) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: &'a [&'a str]) -> Self {
        self.inner.expand = Some(expand);
        self
    }
}
impl UpdateIssuingToken<'_> {
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

impl StripeRequest for UpdateIssuingToken<'_> {
    type Output = stripe_shared::IssuingToken;

    fn build(&self) -> RequestBuilder {
        let token = self.token;
        RequestBuilder::new(StripeMethod::Post, format!("/issuing/tokens/{token}"))
            .form(&self.inner)
    }
}
