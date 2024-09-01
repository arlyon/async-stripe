/// Details about the request forwarded to the destination endpoint.
#[derive(Clone, Debug)]
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
#[doc(hidden)]
pub struct ForwardedRequestDetailsBuilder {
    body: Option<String>,
    headers: Option<Vec<stripe_misc::ForwardedRequestHeader>>,
    http_method: Option<ForwardedRequestDetailsHttpMethod>,
}

#[allow(
    unused_variables,
    irrefutable_let_patterns,
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
                builder: ForwardedRequestDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ForwardedRequestDetailsBuilder {
        type Out = ForwardedRequestDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "body" => Deserialize::begin(&mut self.body),
                "headers" => Deserialize::begin(&mut self.headers),
                "http_method" => Deserialize::begin(&mut self.http_method),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                body: Deserialize::default(),
                headers: Deserialize::default(),
                http_method: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(body), Some(headers), Some(http_method)) =
                (self.body.take(), self.headers.take(), self.http_method)
            else {
                return None;
            };
            Some(Self::Out { body, headers, http_method })
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

    impl ObjectDeser for ForwardedRequestDetails {
        type Builder = ForwardedRequestDetailsBuilder;
    }

    impl FromValueOpt for ForwardedRequestDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ForwardedRequestDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "body" => b.body = FromValueOpt::from_value(v),
                    "headers" => b.headers = FromValueOpt::from_value(v),
                    "http_method" => b.http_method = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The HTTP method used to call the destination endpoint.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ForwardedRequestDetailsHttpMethod {
    Post,
}
impl ForwardedRequestDetailsHttpMethod {
    pub fn as_str(self) -> &'static str {
        use ForwardedRequestDetailsHttpMethod::*;
        match self {
            Post => "POST",
        }
    }
}

impl std::str::FromStr for ForwardedRequestDetailsHttpMethod {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ForwardedRequestDetailsHttpMethod::*;
        match s {
            "POST" => Ok(Post),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ForwardedRequestDetailsHttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ForwardedRequestDetailsHttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
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
impl miniserde::Deserialize for ForwardedRequestDetailsHttpMethod {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ForwardedRequestDetailsHttpMethod> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out =
            Some(ForwardedRequestDetailsHttpMethod::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ForwardedRequestDetailsHttpMethod);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ForwardedRequestDetailsHttpMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ForwardedRequestDetailsHttpMethod")
        })
    }
}
