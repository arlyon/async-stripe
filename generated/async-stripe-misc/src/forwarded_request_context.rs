/// Metadata about the forwarded request.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ForwardedRequestContext {
    /// The time it took in milliseconds for the destination endpoint to respond.
    pub destination_duration: i64,
    /// The IP address of the destination.
    pub destination_ip_address: String,
}
#[doc(hidden)]
pub struct ForwardedRequestContextBuilder {
    destination_duration: Option<i64>,
    destination_ip_address: Option<String>,
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
                builder: ForwardedRequestContextBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ForwardedRequestContextBuilder {
        type Out = ForwardedRequestContext;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "destination_duration" => Deserialize::begin(&mut self.destination_duration),
                "destination_ip_address" => Deserialize::begin(&mut self.destination_ip_address),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                destination_duration: Deserialize::default(),
                destination_ip_address: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(destination_duration), Some(destination_ip_address)) =
                (self.destination_duration, self.destination_ip_address.take())
            else {
                return None;
            };
            Some(Self::Out { destination_duration, destination_ip_address })
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for ForwardedRequestContext {
        type Builder = ForwardedRequestContextBuilder;
    }

    impl FromValueOpt for ForwardedRequestContext {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ForwardedRequestContextBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "destination_duration" => b.destination_duration = FromValueOpt::from_value(v),
                    "destination_ip_address" => {
                        b.destination_ip_address = FromValueOpt::from_value(v)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
