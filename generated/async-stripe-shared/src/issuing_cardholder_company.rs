#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingCardholderCompany {
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingCardholderCompany {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingCardholderCompany").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingCardholderCompanyBuilder {
    tax_id_provided: Option<bool>,
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

    impl Deserialize for IssuingCardholderCompany {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingCardholderCompany>,
        builder: IssuingCardholderCompanyBuilder,
    }

    impl Visitor for Place<IssuingCardholderCompany> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingCardholderCompanyBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingCardholderCompanyBuilder {
        type Out = IssuingCardholderCompany;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "tax_id_provided" => Deserialize::begin(&mut self.tax_id_provided),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { tax_id_provided: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            match core::mem::replace(self, Self::deser_default()) {
                Self { tax_id_provided: Some(tax_id_provided) } => {
                    Some(Self::Out { tax_id_provided })
                }
                builder => {
                    *self = builder;
                    None
                }
            }
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = Some(self.builder.take_out().ok_or(miniserde::Error)?);
            Ok(())
        }
    }

    impl ObjectDeser for IssuingCardholderCompany {
        type Builder = IssuingCardholderCompanyBuilder;
    }

    impl FromValueOpt for IssuingCardholderCompany {
        fn from_value(v: Value) -> Option<Self> {
            stripe_types::miniserde_helpers::from_value_via_deserialize(v)
        }
    }
};
