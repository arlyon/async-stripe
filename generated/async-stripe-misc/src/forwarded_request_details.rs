/// Details about the request forwarded to the destination endpoint.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ForwardedRequestDetails {
    /// The body payload to send to the destination endpoint.
    pub body: String,
    /// The headers to include in the forwarded request.
    /// Can be omitted if no additional headers (excluding Stripe-generated ones such as the Content-Type header) should be included.
    pub headers: Vec<stripe_misc::ForwardedRequestHeader>,
    /// The HTTP method used to call the destination endpoint.
    pub http_method: ForwardedRequestDetailsHttpMethod,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ForwardedRequestDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ForwardedRequestDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ForwardedRequestDetailsBuilder {
    body: Option<String>,
    headers: Option<Vec<stripe_misc::ForwardedRequestHeader>>,
    http_method: Option<ForwardedRequestDetailsHttpMethod>,
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

    impl Deserialize for ForwardedRequestDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ForwardedRequestDetails>,
        builder: ForwardedRequestDetailsBuilder,
    }

    impl Visitor for Place<ForwardedRequestDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ForwardedRequestDetailsBuilder {
                    body: Deserialize::default(),
                    headers: Deserialize::default(),
                    http_method: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "body" => Deserialize::begin(&mut self.builder.body),
                "headers" => Deserialize::begin(&mut self.builder.headers),
                "http_method" => Deserialize::begin(&mut self.builder.http_method),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(body), Some(headers), Some(http_method)) = (
                self.builder.body.take(),
                self.builder.headers.take(),
                self.builder.http_method.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(ForwardedRequestDetails { body, headers, http_method });
            Ok(())
        }
    }
};
/// The HTTP method used to call the destination endpoint.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum ForwardedRequestDetailsHttpMethod {
    Post,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl ForwardedRequestDetailsHttpMethod {
    pub fn as_str(&self) -> &str {
        use ForwardedRequestDetailsHttpMethod::*;
        match self {
            Post => "POST",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for ForwardedRequestDetailsHttpMethod {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ForwardedRequestDetailsHttpMethod::*;
        match s {
            "POST" => Ok(Post),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "ForwardedRequestDetailsHttpMethod"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for ForwardedRequestDetailsHttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for ForwardedRequestDetailsHttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ForwardedRequestDetailsHttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(ForwardedRequestDetailsHttpMethod)).finish_non_exhaustive()
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ForwardedRequestDetailsHttpMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for ForwardedRequestDetailsHttpMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<ForwardedRequestDetailsHttpMethod> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ForwardedRequestDetailsHttpMethod::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ForwardedRequestDetailsHttpMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
