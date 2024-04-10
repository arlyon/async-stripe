#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeAlipay {
    pub data_string: Option<String>,
    pub native_url: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[doc(hidden)]
pub struct SourceTypeAlipayBuilder {
    data_string: Option<Option<String>>,
    native_url: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for SourceTypeAlipay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeAlipay>,
        builder: SourceTypeAlipayBuilder,
    }

    impl Visitor for Place<SourceTypeAlipay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeAlipayBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeAlipayBuilder {
        type Out = SourceTypeAlipay;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "data_string" => Deserialize::begin(&mut self.data_string),
                "native_url" => Deserialize::begin(&mut self.native_url),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                data_string: Deserialize::default(),
                native_url: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                data_string: self.data_string.take()?,
                native_url: self.native_url.take()?,
                statement_descriptor: self.statement_descriptor.take()?,
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

    impl ObjectDeser for SourceTypeAlipay {
        type Builder = SourceTypeAlipayBuilder;
    }

    impl FromValueOpt for SourceTypeAlipay {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeAlipayBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "data_string" => b.data_string = Some(FromValueOpt::from_value(v)?),
                    "native_url" => b.native_url = Some(FromValueOpt::from_value(v)?),
                    "statement_descriptor" => {
                        b.statement_descriptor = Some(FromValueOpt::from_value(v)?)
                    }

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
