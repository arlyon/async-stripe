/// Metadata about the forwarded request.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ForwardedRequestContext {
    /// The time it took in milliseconds for the destination endpoint to respond.
    pub destination_duration: i64,
    /// The IP address of the destination.
    pub destination_ip_address: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ForwardedRequestContext {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ForwardedRequestContext").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ForwardedRequestContextBuilder {
    destination_duration: Option<i64>,
    destination_ip_address: Option<String>,
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

    impl Deserialize for ForwardedRequestContext {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ForwardedRequestContext>,
        builder: ForwardedRequestContextBuilder,
    }

    impl Visitor for Place<ForwardedRequestContext> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ForwardedRequestContextBuilder {
                    destination_duration: Deserialize::default(),
                    destination_ip_address: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "destination_duration" => {
                    Deserialize::begin(&mut self.builder.destination_duration)
                }
                "destination_ip_address" => {
                    Deserialize::begin(&mut self.builder.destination_ip_address)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(destination_duration), Some(destination_ip_address)) =
                (self.builder.destination_duration, self.builder.destination_ip_address.take())
            else {
                return Ok(());
            };
            *self.out =
                Some(ForwardedRequestContext { destination_duration, destination_ip_address });
            Ok(())
        }
    }
};
