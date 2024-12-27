// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ForwardingRequestId};
use crate::params::{Metadata, Object, Timestamp};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ForwardingRequest".
///
/// For more details see <https://stripe.com/docs/api/forwarding/request/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ForwardingRequest {
    /// Unique identifier for the object.
    pub id: ForwardingRequestId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The PaymentMethod to insert into the forwarded request.
    ///
    /// Forwarding previously consumed PaymentMethods is allowed.
    pub payment_method: String,

    /// The field kinds to be replaced in the forwarded request.
    pub replacements: Vec<ForwardingRequestReplacements>,

    /// Context about the request from Stripe's servers to the destination endpoint.
    pub request_context: Option<ForwardedRequestContext>,

    /// The request that was sent to the destination endpoint.
    ///
    /// We redact any sensitive fields.
    pub request_details: Option<ForwardedRequestDetails>,

    /// The response that the destination endpoint returned to us.
    ///
    /// We redact any sensitive fields.
    pub response_details: Option<ForwardedResponseDetails>,

    /// The destination URL for the forwarded request.
    ///
    /// Must be supported by the config.
    pub url: Option<String>,
}

impl Object for ForwardingRequest {
    type Id = ForwardingRequestId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "forwarding.request"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ForwardedRequestContext {

    /// The time it took in milliseconds for the destination endpoint to respond.
    pub destination_duration: i64,

    /// The IP address of the destination.
    pub destination_ip_address: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ForwardedRequestDetails {

    /// The body payload to send to the destination endpoint.
    pub body: String,

    /// The headers to include in the forwarded request.
    ///
    /// Can be omitted if no additional headers (excluding Stripe-generated ones such as the Content-Type header) should be included.
    pub headers: Vec<ForwardedRequestHeader>,

    /// The HTTP method used to call the destination endpoint.
    pub http_method: ForwardedRequestDetailsHttpMethod,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ForwardedRequestHeader {

    /// The header name.
    pub name: String,

    /// The header value.
    pub value: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ForwardedResponseDetails {

    /// The response body from the destination endpoint to Stripe.
    pub body: String,

    /// HTTP headers that the destination endpoint returned.
    pub headers: Vec<ForwardedRequestHeader>,

    /// The HTTP status code that the destination endpoint returned.
    pub status: i64,
}

/// An enum representing the possible values of an `ForwardedRequestDetails`'s `http_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ForwardedRequestDetailsHttpMethod {
    #[serde(rename = "POST")]
    Post,
}

impl ForwardedRequestDetailsHttpMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            ForwardedRequestDetailsHttpMethod::Post => "POST",
        }
    }
}

impl AsRef<str> for ForwardedRequestDetailsHttpMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ForwardedRequestDetailsHttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ForwardedRequestDetailsHttpMethod {
    fn default() -> Self {
        Self::Post
    }
}

/// An enum representing the possible values of an `ForwardingRequest`'s `replacements` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ForwardingRequestReplacements {
    CardCvc,
    CardExpiry,
    CardNumber,
    CardholderName,
    RequestSignature,
}

impl ForwardingRequestReplacements {
    pub fn as_str(self) -> &'static str {
        match self {
            ForwardingRequestReplacements::CardCvc => "card_cvc",
            ForwardingRequestReplacements::CardExpiry => "card_expiry",
            ForwardingRequestReplacements::CardNumber => "card_number",
            ForwardingRequestReplacements::CardholderName => "cardholder_name",
            ForwardingRequestReplacements::RequestSignature => "request_signature",
        }
    }
}

impl AsRef<str> for ForwardingRequestReplacements {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ForwardingRequestReplacements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ForwardingRequestReplacements {
    fn default() -> Self {
        Self::CardCvc
    }
}
