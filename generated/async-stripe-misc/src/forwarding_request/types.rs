/// Instructs Stripe to make a request on your behalf using the destination URL. The destination URL
/// is activated by Stripe at the time of onboarding. Stripe verifies requests with your credentials
/// provided during onboarding, and injects card details from the payment_method into the request.
///
/// Stripe redacts all sensitive fields and headers, including authentication credentials and card numbers,.
/// before storing the request and response data in the forwarding Request object, which are subject to a.
/// 30-day retention period.
///
/// You can provide a Stripe idempotency key to make sure that requests with the same key result in only one.
/// outbound request.
/// The Stripe idempotency key provided should be unique and different from any idempotency.
/// keys provided on the underlying third-party request.
///
/// Forwarding Requests are synchronous requests that return a response or time out according to
/// Stripe’s limits.
///
/// Related guide: [Forward card details to third-party API endpoints](https://docs.stripe.com/payments/forwarding).
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ForwardingRequest {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::ForwardingRequestId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Set of [key-value pairs](https://docs.stripe.com/api/metadata) that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// The PaymentMethod to insert into the forwarded request.
    /// Forwarding previously consumed PaymentMethods is allowed.
    pub payment_method: String,
    /// The field kinds to be replaced in the forwarded request.
    pub replacements: Vec<stripe_misc::ForwardingRequestReplacements>,
    /// Context about the request from Stripe's servers to the destination endpoint.
    pub request_context: Option<stripe_misc::ForwardedRequestContext>,
    /// The request that was sent to the destination endpoint. We redact any sensitive fields.
    pub request_details: Option<stripe_misc::ForwardedRequestDetails>,
    /// The response that the destination endpoint returned to us. We redact any sensitive fields.
    pub response_details: Option<stripe_misc::ForwardedResponseDetails>,
    /// The destination URL for the forwarded request. Must be supported by the config.
    pub url: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ForwardingRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ForwardingRequest").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ForwardingRequestBuilder {
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_misc::ForwardingRequestId>,
    livemode: Option<bool>,
    metadata: Option<Option<std::collections::HashMap<String, String>>>,
    payment_method: Option<String>,
    replacements: Option<Vec<stripe_misc::ForwardingRequestReplacements>>,
    request_context: Option<Option<stripe_misc::ForwardedRequestContext>>,
    request_details: Option<Option<stripe_misc::ForwardedRequestDetails>>,
    response_details: Option<Option<stripe_misc::ForwardedResponseDetails>>,
    url: Option<Option<String>>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

    make_place!(Place);

    impl Deserialize for ForwardingRequest {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ForwardingRequest>,
        builder: ForwardingRequestBuilder,
    }

    impl Visitor for Place<ForwardingRequest> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ForwardingRequestBuilder {
                    created: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    metadata: Deserialize::default(),
                    payment_method: Deserialize::default(),
                    replacements: Deserialize::default(),
                    request_context: Deserialize::default(),
                    request_details: Deserialize::default(),
                    response_details: Deserialize::default(),
                    url: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.builder.created),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "metadata" => Deserialize::begin(&mut self.builder.metadata),
                "payment_method" => Deserialize::begin(&mut self.builder.payment_method),
                "replacements" => Deserialize::begin(&mut self.builder.replacements),
                "request_context" => Deserialize::begin(&mut self.builder.request_context),
                "request_details" => Deserialize::begin(&mut self.builder.request_details),
                "response_details" => Deserialize::begin(&mut self.builder.response_details),
                "url" => Deserialize::begin(&mut self.builder.url),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(created),
                Some(id),
                Some(livemode),
                Some(metadata),
                Some(payment_method),
                Some(replacements),
                Some(request_context),
                Some(request_details),
                Some(response_details),
                Some(url),
            ) = (
                self.builder.created,
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.metadata.take(),
                self.builder.payment_method.take(),
                self.builder.replacements.take(),
                self.builder.request_context.take(),
                self.builder.request_details.take(),
                self.builder.response_details.take(),
                self.builder.url.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(ForwardingRequest {
                created,
                id,
                livemode,
                metadata,
                payment_method,
                replacements,
                request_context,
                request_details,
                response_details,
                url,
            });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ForwardingRequest {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ForwardingRequest", 11)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("payment_method", &self.payment_method)?;
        s.serialize_field("replacements", &self.replacements)?;
        s.serialize_field("request_context", &self.request_context)?;
        s.serialize_field("request_details", &self.request_details)?;
        s.serialize_field("response_details", &self.response_details)?;
        s.serialize_field("url", &self.url)?;

        s.serialize_field("object", "forwarding.request")?;
        s.end()
    }
}
impl stripe_types::Object for ForwardingRequest {
    type Id = stripe_misc::ForwardingRequestId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ForwardingRequestId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ForwardingRequestReplacements {
    CardCvc,
    CardExpiry,
    CardNumber,
    CardholderName,
    RequestSignature,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ForwardingRequestReplacements {
    pub fn as_str(&self) -> &str {
        use ForwardingRequestReplacements::*;
        match self {
            CardCvc => "card_cvc",
            CardExpiry => "card_expiry",
            CardNumber => "card_number",
            CardholderName => "cardholder_name",
            RequestSignature => "request_signature",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ForwardingRequestReplacements {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ForwardingRequestReplacements::*;
        match s {
            "card_cvc" => Ok(CardCvc),
            "card_expiry" => Ok(CardExpiry),
            "card_number" => Ok(CardNumber),
            "cardholder_name" => Ok(CardholderName),
            "request_signature" => Ok(RequestSignature),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ForwardingRequestReplacements"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ForwardingRequestReplacements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ForwardingRequestReplacements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ForwardingRequestReplacements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ForwardingRequestReplacements)).finish_non_exhaustive()
    }
}
impl serde::Serialize for ForwardingRequestReplacements {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ForwardingRequestReplacements {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ForwardingRequestReplacements> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ForwardingRequestReplacements::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ForwardingRequestReplacements {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
