        /// Returns a list of Issuing `Authorization` objects.
        ///
        /// The objects are sorted in descending order by creation date, with the most recently created object appearing first.
pub fn list(client:&stripe::Client,params:ListAuthorization) -> stripe::Response<stripe_types::List<stripe_types::issuing::authorization::Authorization>> {
    client.get_query("/issuing/authorizations", params)
}
        /// Retrieves an Issuing `Authorization` object.
pub fn retrieve(client:&stripe::Client,authorization:&stripe_types::issuing::authorization::IssuingAuthorizationId,params:RetrieveAuthorization) -> stripe::Response<stripe_types::issuing::authorization::Authorization> {
    client.get_query(&format!("/issuing/authorizations/{authorization}", authorization=authorization), params)
}
        /// Updates the specified Issuing `Authorization` object by setting the values of the parameters passed.
        ///
        /// Any parameters not provided will be left unchanged.
pub fn update(client:&stripe::Client,authorization:&stripe_types::issuing::authorization::IssuingAuthorizationId,params:UpdateAuthorization) -> stripe::Response<stripe_types::issuing::authorization::Authorization> {
    client.send_form(&format!("/issuing/authorizations/{authorization}", authorization=authorization), params, http_types::Method::Post)
}
        /// Approves a pending Issuing `Authorization` object.
        ///
        /// This request should be made within the timeout window of the [real-time authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations) flow.
        /// You can also respond directly to the webhook request to approve an authorization (preferred).
        /// More details can be found [here](https://stripe.com/docs/issuing/controls/real-time-authorizations#authorization-handling).
pub fn approve(client:&stripe::Client,authorization:&stripe_types::issuing::authorization::IssuingAuthorizationId,params:ApproveAuthorization) -> stripe::Response<stripe_types::issuing::authorization::Authorization> {
    client.send_form(&format!("/issuing/authorizations/{authorization}/approve", authorization=authorization), params, http_types::Method::Post)
}
        /// Declines a pending Issuing `Authorization` object.
        ///
        /// This request should be made within the timeout window of the [real time authorization](https://stripe.com/docs/issuing/controls/real-time-authorizations) flow. You can also respond directly to the webhook request to decline an authorization (preferred).
        /// More details can be found [here](https://stripe.com/docs/issuing/controls/real-time-authorizations#authorization-handling).
pub fn decline(client:&stripe::Client,authorization:&stripe_types::issuing::authorization::IssuingAuthorizationId,params:DeclineAuthorization) -> stripe::Response<stripe_types::issuing::authorization::Authorization> {
    client.send_form(&format!("/issuing/authorizations/{authorization}/decline", authorization=authorization), params, http_types::Method::Post)
}
#[derive(Clone, Debug,Default,serde::Serialize,)]
pub struct ListAuthorization<'a> {
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
    /// Only return authorizations with the given status.
    ///
    /// One of `pending`, `closed`, or `reversed`.
#[serde(skip_serializing_if = "Option::is_none")]
pub status: Option<ListAuthorizationStatus>,

}
impl<'a> ListAuthorization<'a> {
    
            pub fn new() -> Self {
                Self::default()
            }
            
}
    /// Only return authorizations with the given status.
///
/// One of `pending`, `closed`, or `reversed`.
#[derive(Copy,Clone, Debug,Eq, PartialEq,)]
pub enum ListAuthorizationStatus {
Closed,
Pending,
Reversed,

}

impl ListAuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        use ListAuthorizationStatus::*;
        match self {
Closed => "closed",
Pending => "pending",
Reversed => "reversed",

        }
    }
}

impl std::str::FromStr for ListAuthorizationStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ListAuthorizationStatus::*;
        match s {
    "closed" => Ok(Closed),
"pending" => Ok(Pending),
"reversed" => Ok(Reversed),
_ => Err(())

        }
    }
}

impl AsRef<str> for ListAuthorizationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ListAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl serde::Serialize for ListAuthorizationStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
        serializer.serialize_str(self.as_str())
    }
}
#[derive(Copy,Clone, Debug,Default,serde::Serialize,)]
pub struct RetrieveAuthorization<'a> {
    /// Specifies which fields in the response should be expanded.
#[serde(skip_serializing_if = "Option::is_none")]
pub expand: Option<&'a [&'a str]>,

}
impl<'a> RetrieveAuthorization<'a> {
    
            pub fn new() -> Self {
                Self::default()
            }
            
}
    #[derive(Copy,Clone, Debug,Default,serde::Serialize,)]
pub struct UpdateAuthorization<'a> {
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
impl<'a> UpdateAuthorization<'a> {
    
            pub fn new() -> Self {
                Self::default()
            }
            
}
    #[derive(Copy,Clone, Debug,Default,serde::Serialize,)]
pub struct ApproveAuthorization<'a> {
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
impl<'a> ApproveAuthorization<'a> {
    
            pub fn new() -> Self {
                Self::default()
            }
            
}
    #[derive(Copy,Clone, Debug,Default,serde::Serialize,)]
pub struct DeclineAuthorization<'a> {
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
impl<'a> DeclineAuthorization<'a> {
    
            pub fn new() -> Self {
                Self::default()
            }
            
}
    