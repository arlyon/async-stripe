#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeSofort {
    pub bank_code: Option<String>,
    pub bank_name: Option<String>,
    pub bic: Option<String>,
    pub country: Option<String>,
    pub iban_last4: Option<String>,
    pub preferred_language: Option<String>,
    pub statement_descriptor: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for SourceTypeSofort {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("SourceTypeSofort").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct SourceTypeSofortBuilder {
    bank_code: Option<Option<String>>,
    bank_name: Option<Option<String>>,
    bic: Option<Option<String>>,
    country: Option<Option<String>>,
    iban_last4: Option<Option<String>>,
    preferred_language: Option<Option<String>>,
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

    impl Deserialize for SourceTypeSofort {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<SourceTypeSofort>,
        builder: SourceTypeSofortBuilder,
    }

    impl Visitor for Place<SourceTypeSofort> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: SourceTypeSofortBuilder {
                    bank_code: Deserialize::default(),
                    bank_name: Deserialize::default(),
                    bic: Deserialize::default(),
                    country: Deserialize::default(),
                    iban_last4: Deserialize::default(),
                    preferred_language: Deserialize::default(),
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
                "country" => Deserialize::begin(&mut self.builder.country),
                "iban_last4" => Deserialize::begin(&mut self.builder.iban_last4),
                "preferred_language" => Deserialize::begin(&mut self.builder.preferred_language),
                "statement_descriptor" => {
                    Deserialize::begin(&mut self.builder.statement_descriptor)
                }
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (
                Some(bank_code),
                Some(bank_name),
                Some(bic),
                Some(country),
                Some(iban_last4),
                Some(preferred_language),
                Some(statement_descriptor),
            ) = (
                self.builder.bank_code.take(),
                self.builder.bank_name.take(),
                self.builder.bic.take(),
                self.builder.country.take(),
                self.builder.iban_last4.take(),
                self.builder.preferred_language.take(),
                self.builder.statement_descriptor.take(),
            )
            else {
                return Ok(());
            };
            *self.out = Some(SourceTypeSofort {
                bank_code,
                bank_name,
                bic,
                country,
                iban_last4,
                preferred_language,
                statement_descriptor,
            });
            Ok(())
        }
    }
};
