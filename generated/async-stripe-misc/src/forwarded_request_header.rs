/// Header data.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ForwardedRequestHeader {
    /// The header name.
    pub name: String,
    /// The header value.
    pub value: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ForwardedRequestHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ForwardedRequestHeader").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ForwardedRequestHeaderBuilder {
    name: Option<String>,
    value: Option<String>,
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

    impl Deserialize for ForwardedRequestHeader {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ForwardedRequestHeader>,
        builder: ForwardedRequestHeaderBuilder,
    }

    impl Visitor for Place<ForwardedRequestHeader> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ForwardedRequestHeaderBuilder {
                    name: Deserialize::default(),
                    value: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "name" => Deserialize::begin(&mut self.builder.name),
                "value" => Deserialize::begin(&mut self.builder.value),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(name), Some(value)) = (self.builder.name.take(), self.builder.value.take())
            else {
                return Ok(());
            };
            *self.out = Some(ForwardedRequestHeader { name, value });
            Ok(())
        }
    }
};
