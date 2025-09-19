#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct SourceTypeIdeal {
    pub bank: Option<String>,
    pub bic: Option<String>,
    pub iban_last4: Option<String>,
    pub statement_descriptor: Option<String>,
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
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{Deserialize, Result, make_place};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

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
                builder: SourceTypeIdealBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for SourceTypeIdealBuilder {
        type Out = SourceTypeIdeal;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "bank" => Deserialize::begin(&mut self.bank),
                "bic" => Deserialize::begin(&mut self.bic),
                "iban_last4" => Deserialize::begin(&mut self.iban_last4),
                "statement_descriptor" => Deserialize::begin(&mut self.statement_descriptor),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                bank: Deserialize::default(),
                bic: Deserialize::default(),
                iban_last4: Deserialize::default(),
                statement_descriptor: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(bank), Some(bic), Some(iban_last4), Some(statement_descriptor)) = (
                self.bank.take(),
                self.bic.take(),
                self.iban_last4.take(),
                self.statement_descriptor.take(),
            ) else {
                return None;
            };
            Some(Self::Out { bank, bic, iban_last4, statement_descriptor })
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

    impl ObjectDeser for SourceTypeIdeal {
        type Builder = SourceTypeIdealBuilder;
    }

    impl FromValueOpt for SourceTypeIdeal {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = SourceTypeIdealBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "bank" => b.bank = FromValueOpt::from_value(v),
                    "bic" => b.bic = FromValueOpt::from_value(v),
                    "iban_last4" => b.iban_last4 = FromValueOpt::from_value(v),
                    "statement_descriptor" => b.statement_descriptor = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
