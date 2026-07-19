/// Details about the response from the destination endpoint.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ForwardedResponseDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ForwardedResponseDetails").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: ForwardedResponseDetailsBuilder {
                    body: Deserialize::default(),
                    headers: Deserialize::default(),
                    status: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "body" => Deserialize::begin(&mut self.builder.body),
                "headers" => Deserialize::begin(&mut self.builder.headers),
                "status" => Deserialize::begin(&mut self.builder.status),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(body), Some(headers), Some(status)) =
                (self.builder.body.take(), self.builder.headers.take(), self.builder.status)
            else {
                return Ok(());
            };
            *self.out = Some(ForwardedResponseDetails { body, headers, status });
            Ok(())
        }
    }
};
