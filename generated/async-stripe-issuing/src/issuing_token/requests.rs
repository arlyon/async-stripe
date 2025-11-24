use stripe_client_core::{
    RequestBuilder, StripeBlockingClient, StripeClient, StripeMethod, StripeRequest,
};

#[derive(Clone, Debug, serde::Serialize)]
struct ListIssuingTokenBuilder {
    card: String,
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
    status: Option<stripe_shared::IssuingTokenStatus>,
}
impl ListIssuingTokenBuilder {
    fn new(card: impl Into<String>) -> Self {
        Self {
            card: card.into(),
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
pub struct ListIssuingToken {
    inner: ListIssuingTokenBuilder,
}
impl ListIssuingToken {
    /// Construct a new `ListIssuingToken`.
    pub fn new(card: impl Into<String>) -> Self {
        Self { inner: ListIssuingTokenBuilder::new(card.into()) }
    }
    /// Only return Issuing tokens that were created during the given date interval.
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
    /// Select Issuing tokens with the given status.
    pub fn status(mut self, status: impl Into<stripe_shared::IssuingTokenStatus>) -> Self {
        self.inner.status = Some(status.into());
        self
    }
}
impl ListIssuingToken {
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
        stripe_client_core::ListPaginator::new_list("/issuing/tokens", &self.inner)
    }
}

impl StripeRequest for ListIssuingToken {
    type Output = stripe_types::List<stripe_shared::IssuingToken>;

    fn build(&self) -> RequestBuilder {
        RequestBuilder::new(StripeMethod::Get, "/issuing/tokens").query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct RetrieveIssuingTokenBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
}
impl RetrieveIssuingTokenBuilder {
    fn new() -> Self {
        Self { expand: None }
    }
}
/// Retrieves an Issuing `Token` object.
#[derive(Clone, Debug, serde::Serialize)]
pub struct RetrieveIssuingToken {
    inner: RetrieveIssuingTokenBuilder,
    token: stripe_shared::IssuingTokenId,
}
impl RetrieveIssuingToken {
    /// Construct a new `RetrieveIssuingToken`.
    pub fn new(token: impl Into<stripe_shared::IssuingTokenId>) -> Self {
        Self { token: token.into(), inner: RetrieveIssuingTokenBuilder::new() }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl RetrieveIssuingToken {
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

impl StripeRequest for RetrieveIssuingToken {
    type Output = stripe_shared::IssuingToken;

    fn build(&self) -> RequestBuilder {
        let token = &self.token;
        RequestBuilder::new(StripeMethod::Get, format!("/issuing/tokens/{token}"))
            .query(&self.inner)
    }
}
#[derive(Clone, Debug, serde::Serialize)]
struct UpdateIssuingTokenBuilder {
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<Vec<String>>,
    status: UpdateIssuingTokenStatus,
}
impl UpdateIssuingTokenBuilder {
    fn new(status: impl Into<UpdateIssuingTokenStatus>) -> Self {
        Self { expand: None, status: status.into() }
    }
}
/// Specifies which status the token should be updated to.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum UpdateIssuingTokenStatus {
    Active,
    Deleted,
    Suspended,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl UpdateIssuingTokenStatus {
    pub fn as_str(&self) -> &str {
        use UpdateIssuingTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Suspended => "suspended",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for UpdateIssuingTokenStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "suspended" => Ok(Suspended),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "UpdateIssuingTokenStatus");
                Ok(Unknown(v.to_owned()))
            }
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
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
/// Attempts to update the specified Issuing `Token` object to the status specified.
#[derive(Clone, Debug, serde::Serialize)]
pub struct UpdateIssuingToken {
    inner: UpdateIssuingTokenBuilder,
    token: stripe_shared::IssuingTokenId,
}
impl UpdateIssuingToken {
    /// Construct a new `UpdateIssuingToken`.
    pub fn new(
        token: impl Into<stripe_shared::IssuingTokenId>,
        status: impl Into<UpdateIssuingTokenStatus>,
    ) -> Self {
        Self { token: token.into(), inner: UpdateIssuingTokenBuilder::new(status.into()) }
    }
    /// Specifies which fields in the response should be expanded.
    pub fn expand(mut self, expand: impl Into<Vec<String>>) -> Self {
        self.inner.expand = Some(expand.into());
        self
    }
}
impl UpdateIssuingToken {
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

impl StripeRequest for UpdateIssuingToken {
    type Output = stripe_shared::IssuingToken;

    fn build(&self) -> RequestBuilder {
        let token = &self.token;
        RequestBuilder::new(StripeMethod::Post, format!("/issuing/tokens/{token}"))
            .form(&self.inner)
    }
}
