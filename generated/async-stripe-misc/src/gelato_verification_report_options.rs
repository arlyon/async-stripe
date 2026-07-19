#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct GelatoVerificationReportOptions {
    pub document: Option<stripe_misc::GelatoReportDocumentOptions>,
    pub id_number: Option<stripe_misc::GelatoReportIdNumberOptions>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for GelatoVerificationReportOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("GelatoVerificationReportOptions").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct GelatoVerificationReportOptionsBuilder {
    document: Option<Option<stripe_misc::GelatoReportDocumentOptions>>,
    id_number: Option<Option<stripe_misc::GelatoReportIdNumberOptions>>,
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
                builder: GelatoVerificationReportOptionsBuilder {
                    document: Deserialize::default(),
                    id_number: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "document" => Deserialize::begin(&mut self.builder.document),
                "id_number" => Deserialize::begin(&mut self.builder.id_number),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(document), Some(id_number)) =
                (self.builder.document.take(), self.builder.id_number)
            else {
                return Ok(());
            };
            *self.out = Some(GelatoVerificationReportOptions { document, id_number });
            Ok(())
        }
    }
};
