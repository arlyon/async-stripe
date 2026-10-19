#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PersonEthnicityDetails {
    /// The persons ethnicity
    pub ethnicity: Option<Vec<stripe_shared::Ethnicity>>,
    /// Please specify your origin, when other is selected.
    pub ethnicity_other: Option<String>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PersonEthnicityDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PersonEthnicityDetails").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct PersonEthnicityDetailsBuilder {
    ethnicity: Option<Option<Vec<stripe_shared::Ethnicity>>>,
    ethnicity_other: Option<Option<String>>,
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

    impl Deserialize for PersonEthnicityDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PersonEthnicityDetails>,
        builder: PersonEthnicityDetailsBuilder,
    }

    impl Visitor for Place<PersonEthnicityDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PersonEthnicityDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PersonEthnicityDetailsBuilder {
        type Out = PersonEthnicityDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "ethnicity" => Deserialize::begin(&mut self.ethnicity),
                "ethnicity_other" => Deserialize::begin(&mut self.ethnicity_other),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { ethnicity: Some(None), ethnicity_other: Some(None) }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(ethnicity), Some(ethnicity_other)) =
                (self.ethnicity.take(), self.ethnicity_other.take())
            else {
                return None;
            };
            Some(Self::Out { ethnicity, ethnicity_other })
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

    impl ObjectDeser for PersonEthnicityDetails {
        type Builder = PersonEthnicityDetailsBuilder;
    }

    impl FromValueOpt for PersonEthnicityDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PersonEthnicityDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "ethnicity" => b.ethnicity = FromValueOpt::from_value(v),
                    "ethnicity_other" => b.ethnicity_other = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// The persons ethnicity
pub type PersonEthnicityDetailsEthnicity = stripe_shared::Ethnicity;
