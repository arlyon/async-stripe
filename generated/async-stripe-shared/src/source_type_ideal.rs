#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeIdeal {
    pub bank: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeIdeal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeIdeal").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeIdealBuilder {
    bank: Option<Option<String>>,
    bic: Option<Option<String>>,
    iban_last4: Option<Option<String>>,
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

    impl Deserialize for SourceTypeIdeal {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeIdeal>,
        builder: SourceTypeIdealBuilder,
    }

    impl Visitor for Place<SourceTypeIdeal> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeIdealBuilder {
                    bank: Deserialize::default(),
                    bic: Deserialize::default(),
                    iban_last4: Deserialize::default(),
                    statement_descriptor: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank" => Deserialize::begin(&mut self.builder.bank),
                "bic" => Deserialize::begin(&mut self.builder.bic),
                "iban_last4" => Deserialize::begin(&mut self.builder.iban_last4),
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(bank), Some(bic), Some(iban_last4), Some(statement_descriptor)) = (
                self.builder.bank.take(),
                self.builder.bic.take(),
                self.builder.iban_last4.take(),
                self.builder.statement_descriptor.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(SourceTypeIdeal { bank, bic, iban_last4, statement_descriptor });
            Ok(())
        }
    }
};
