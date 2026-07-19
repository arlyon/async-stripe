#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeAlipay {
    pub data_string: Option<String>,
    pub native_url: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeAlipay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeAlipay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeAlipayBuilder {
    data_string: Option<Option<String>>,
    native_url: Option<Option<String>>,
    statement_descriptor: Option<Option<String>>,
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
                builder: SourceTypeAlipayBuilder {
                    data_string: Deserialize::default(),
                    native_url: Deserialize::default(),
                    statement_descriptor: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "data_string" => Deserialize::begin(&mut self.builder.data_string),
                "native_url" => Deserialize::begin(&mut self.builder.native_url),
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(data_string), Some(native_url), Some(statement_descriptor)) = (
                self.builder.data_string.take(),
                self.builder.native_url.take(),
                self.builder.statement_descriptor.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SourceTypeAlipay { data_string, native_url, statement_descriptor });
            Ok(())
        }
    }
};
