/// Details about the response from the destination endpoint.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ForwardedResponseDetails {
    /// The response body from the destination endpoint to Stripe.
    pub body: String,
    /// HTTP headers that the destination endpoint returned.
    pub headers: Vec<stripe_misc::ForwardedRequestHeader>,
    /// The HTTP status code that the destination endpoint returned.
    pub status: i64,
}
#[doc(hidden)]
pub struct ForwardedResponseDetailsBuilder {
    body: Option<String>,
    headers: Option<Vec<stripe_misc::ForwardedRequestHeader>>,
    status: Option<i64>,
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

    impl Deserialize for ForwardedResponseDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ForwardedResponseDetails>,
        builder: ForwardedResponseDetailsBuilder,
    }

    impl Visitor for Place<ForwardedResponseDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ForwardedResponseDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ForwardedResponseDetailsBuilder {
        type Out = ForwardedResponseDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "body" => Deserialize::begin(&mut self.body),
                "headers" => Deserialize::begin(&mut self.headers),
                "status" => Deserialize::begin(&mut self.status),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                body: Deserialize::default(),
                headers: Deserialize::default(),
                status: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(body), Some(headers), Some(status)) =
                (self.body.take(), self.headers.take(), self.status)
            else {
                return None;
            };
            Some(Self::Out { body, headers, status })
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

    impl ObjectDeser for ForwardedResponseDetails {
        type Builder = ForwardedResponseDetailsBuilder;
    }

    impl FromValueOpt for ForwardedResponseDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ForwardedResponseDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "body" => b.body = FromValueOpt::from_value(v),
                    "headers" => b.headers = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
