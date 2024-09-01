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
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ForwardingRequest {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Unique identifier for the object.
    pub id: stripe_misc::ForwardingRequestId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
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
#[doc(hidden)]
pub struct ForwardingRequestBuilder {
    created: Option<stripe_types::Timestamp>,
    id: Option<stripe_misc::ForwardingRequestId>,
    livemode: Option<bool>,
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: ForwardingRequestBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ForwardingRequestBuilder {
        type Out = ForwardingRequest;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "created" => Deserialize::begin(&mut self.created),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "payment_method" => Deserialize::begin(&mut self.payment_method),
                "replacements" => Deserialize::begin(&mut self.replacements),
                "request_context" => Deserialize::begin(&mut self.request_context),
                "request_details" => Deserialize::begin(&mut self.request_details),
                "response_details" => Deserialize::begin(&mut self.response_details),
                "url" => Deserialize::begin(&mut self.url),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                created: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                payment_method: Deserialize::default(),
                replacements: Deserialize::default(),
                request_context: Deserialize::default(),
                request_details: Deserialize::default(),
                response_details: Deserialize::default(),
                url: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(created),
                Some(id),
                Some(livemode),
                Some(payment_method),
                Some(replacements),
                Some(request_context),
                Some(request_details),
                Some(response_details),
                Some(url),
            ) = (
                self.created,
                self.id.take(),
                self.livemode,
                self.payment_method.take(),
                self.replacements.take(),
                self.request_context.take(),
                self.request_details.take(),
                self.response_details.take(),
                self.url.take(),
            )
            else {
                return None;
            };
            Some(Self::Out {
                created,
                id,
                livemode,
                payment_method,
                replacements,
                request_context,
                request_details,
                response_details,
                url,
            })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for ForwardingRequest {
        type Builder = ForwardingRequestBuilder;
    }

    impl FromValueOpt for ForwardingRequest {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ForwardingRequestBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "created" => b.created = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "payment_method" => b.payment_method = FromValueOpt::from_value(v),
                    "replacements" => b.replacements = FromValueOpt::from_value(v),
                    "request_context" => b.request_context = FromValueOpt::from_value(v),
                    "request_details" => b.request_details = FromValueOpt::from_value(v),
                    "response_details" => b.response_details = FromValueOpt::from_value(v),
                    "url" => b.url = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ForwardingRequest {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ForwardingRequest", 10)?;
        s.serialize_field("created", &self.created)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
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
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ForwardingRequestReplacements {
    CardCvc,
    CardExpiry,
    CardNumber,
    CardholderName,
}
impl ForwardingRequestReplacements {
    pub fn as_str(self) -> &'static str {
        use ForwardingRequestReplacements::*;
        match self {
            CardCvc => "card_cvc",
            CardExpiry => "card_expiry",
            CardNumber => "card_number",
            CardholderName => "cardholder_name",
        }
    }
}

impl std::str::FromStr for ForwardingRequestReplacements {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ForwardingRequestReplacements::*;
        match s {
            "card_cvc" => Ok(CardCvc),
            "card_expiry" => Ok(CardExpiry),
            "card_number" => Ok(CardNumber),
            "cardholder_name" => Ok(CardholderName),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ForwardingRequestReplacements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ForwardingRequestReplacements {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for ForwardingRequestReplacements {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ForwardingRequestReplacements> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ForwardingRequestReplacements::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ForwardingRequestReplacements);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ForwardingRequestReplacements {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ForwardingRequestReplacements")
        })
    }
}
