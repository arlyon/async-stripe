#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeGiropay {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeGiropay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeGiropay").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeGiropayBuilder {
    bank_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
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

    impl Deserialize for SourceTypeGiropay {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeGiropay>,
        builder: SourceTypeGiropayBuilder,
    }

    impl Visitor for Place<SourceTypeGiropay> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeGiropayBuilder {
                    bank_code: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    bic: Deserialize::default(),
                    statement_descriptor: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank_code" => Deserialize::begin(&mut self.builder.bank_code),
                "bank_name" => Deserialize::begin(&mut self.builder.bank_name),
                "bic" => Deserialize::begin(&mut self.builder.bic),
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bank_code), Some(bank_name), Some(bic), Some(statement_descriptor)) = (
                self.builder.bank_code.take(),
                self.builder.bank_name.take(),
                self.builder.bic.take(),
                self.builder.statement_descriptor.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SourceTypeGiropay { bank_code, bank_name, bic, statement_descriptor });
            Ok(())
        }
    }
};
