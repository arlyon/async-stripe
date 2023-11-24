#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct ListIssuingNetworkToken<'a> {
    /// The Issuing card identifier to list tokens for.
    pub card: &'a str,
    /// Select Issuing tokens that were created during the given date interval.
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
    /// Select Issuing tokens with the given status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListIssuingNetworkTokenStatus>,
}
impl<'a> ListIssuingNetworkToken<'a> {
    pub fn new(card: &'a str) -> Self {
        Self {
            card,
            created: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            starting_after: Default::default(),
            status: Default::default(),
        }
    }
}
/// Select Issuing tokens with the given status.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ListIssuingNetworkTokenStatus {
    Active,
    Deleted,
    Requested,
    Suspended,
}

impl ListIssuingNetworkTokenStatus {
    pub fn as_str(self) -> &'static str {
        use ListIssuingNetworkTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Requested => "requested",
            Suspended => "suspended",
        }
    }
}

impl std::str::FromStr for ListIssuingNetworkTokenStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListIssuingNetworkTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "requested" => Ok(Requested),
            "suspended" => Ok(Suspended),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for ListIssuingNetworkTokenStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListIssuingNetworkTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ListIssuingNetworkTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for ListIssuingNetworkTokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> ListIssuingNetworkToken<'a> {
    /// Lists all Issuing `Token` objects for a given card.
    pub fn send(
        &self,
        client: &stripe::Client,
    ) -> stripe::Response<stripe_types::List<stripe_types::IssuingNetworkToken>> {
        client.get_query("/issuing/tokens", self)
    }
    pub fn paginate(self) -> stripe::ListPaginator<stripe_types::IssuingNetworkToken> {
        stripe::ListPaginator::from_params("/issuing/tokens", self)
    }
}
impl<'a> stripe::PaginationParams for ListIssuingNetworkToken<'a> {}
#[derive(Copy, Clone, Debug, Default, serde::Serialize)]
pub struct RetrieveIssuingNetworkToken<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
}
impl<'a> RetrieveIssuingNetworkToken<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}
impl<'a> RetrieveIssuingNetworkToken<'a> {
    /// Retrieves an Issuing `Token` object.
    pub fn send(
        &self,
        client: &stripe::Client,
        token: &stripe_types::issuing_network_token::IssuingTokenId,
    ) -> stripe::Response<stripe_types::IssuingNetworkToken> {
        client.get_query(&format!("/issuing/tokens/{token}"), self)
    }
}
#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct UpdateIssuingNetworkToken<'a> {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<&'a [&'a str]>,
    /// Specifies which status the token should be updated to.
    pub status: UpdateIssuingNetworkTokenStatus,
}
impl<'a> UpdateIssuingNetworkToken<'a> {
    pub fn new(status: UpdateIssuingNetworkTokenStatus) -> Self {
        Self { expand: Default::default(), status }
    }
}
/// Specifies which status the token should be updated to.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum UpdateIssuingNetworkTokenStatus {
    Active,
    Deleted,
    Suspended,
}

impl UpdateIssuingNetworkTokenStatus {
    pub fn as_str(self) -> &'static str {
        use UpdateIssuingNetworkTokenStatus::*;
        match self {
            Active => "active",
            Deleted => "deleted",
            Suspended => "suspended",
        }
    }
}

impl std::str::FromStr for UpdateIssuingNetworkTokenStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use UpdateIssuingNetworkTokenStatus::*;
        match s {
            "active" => Ok(Active),
            "deleted" => Ok(Deleted),
            "suspended" => Ok(Suspended),
            _ => Err(()),
        }
    }
}

impl AsRef<str> for UpdateIssuingNetworkTokenStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateIssuingNetworkTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for UpdateIssuingNetworkTokenStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl serde::Serialize for UpdateIssuingNetworkTokenStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'a> UpdateIssuingNetworkToken<'a> {
    /// Attempts to update the specified Issuing `Token` object to the status specified.
    pub fn send(
        &self,
        client: &stripe::Client,
        token: &stripe_types::issuing_network_token::IssuingTokenId,
    ) -> stripe::Response<stripe_types::IssuingNetworkToken> {
        client.send_form(&format!("/issuing/tokens/{token}"), self, http_types::Method::Post)
    }
}
