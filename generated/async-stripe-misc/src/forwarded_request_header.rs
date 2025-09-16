/// Header data.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ForwardedRequestHeader {
    /// The header name.
    pub name: String,
    /// The header value.
    pub value: String,
}
#[doc(hidden)]
pub struct ForwardedRequestHeaderBuilder {
    name: Option<String>,
    value: Option<String>,
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
                builder: ForwardedRequestHeaderBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ForwardedRequestHeaderBuilder {
        type Out = ForwardedRequestHeader;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "name" => Deserialize::begin(&mut self.name),
                "value" => Deserialize::begin(&mut self.value),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { name: Deserialize::default(), value: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(name), Some(value)) = (self.name.take(), self.value.take()) else {
                return None;
            };
            Some(Self::Out { name, value })
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

    impl ObjectDeser for ForwardedRequestHeader {
        type Builder = ForwardedRequestHeaderBuilder;
    }

    impl FromValueOpt for ForwardedRequestHeader {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ForwardedRequestHeaderBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "name" => b.name = FromValueOpt::from_value(v),
                    "value" => b.value = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
