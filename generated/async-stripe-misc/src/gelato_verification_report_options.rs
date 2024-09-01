#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoVerificationReportOptions {
    pub document: Option<stripe_misc::GelatoReportDocumentOptions>,
    pub id_number: Option<stripe_misc::GelatoReportIdNumberOptions>,
}
#[doc(hidden)]
pub struct GelatoVerificationReportOptionsBuilder {
    document: Option<Option<stripe_misc::GelatoReportDocumentOptions>>,
    id_number: Option<Option<stripe_misc::GelatoReportIdNumberOptions>>,
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

    impl Deserialize for GelatoVerificationReportOptions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<GelatoVerificationReportOptions>,
        builder: GelatoVerificationReportOptionsBuilder,
    }

    impl Visitor for Place<GelatoVerificationReportOptions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: GelatoVerificationReportOptionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for GelatoVerificationReportOptionsBuilder {
        type Out = GelatoVerificationReportOptions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "document" => Deserialize::begin(&mut self.document),
                "id_number" => Deserialize::begin(&mut self.id_number),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { document: Deserialize::default(), id_number: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(document), Some(id_number)) = (self.document.take(), self.id_number) else {
                return None;
            };
            Some(Self::Out { document, id_number })
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

    impl ObjectDeser for GelatoVerificationReportOptions {
        type Builder = GelatoVerificationReportOptionsBuilder;
    }

    impl FromValueOpt for GelatoVerificationReportOptions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = GelatoVerificationReportOptionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "document" => b.document = FromValueOpt::from_value(v),
                    "id_number" => b.id_number = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
