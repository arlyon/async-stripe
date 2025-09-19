#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct CountrySpecVerificationFieldDetails {
    /// Additional fields which are only required for some users.
    pub additional: Vec<String>,
    /// Fields which every account must eventually provide.
    pub minimum: Vec<String>,
}
#[doc(hidden)]
pub struct CountrySpecVerificationFieldDetailsBuilder {
    additional: Option<Vec<String>>,
    minimum: Option<Vec<String>>,
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

    impl Deserialize for CountrySpecVerificationFieldDetails {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<CountrySpecVerificationFieldDetails>,
        builder: CountrySpecVerificationFieldDetailsBuilder,
    }

    impl Visitor for Place<CountrySpecVerificationFieldDetails> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: CountrySpecVerificationFieldDetailsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for CountrySpecVerificationFieldDetailsBuilder {
        type Out = CountrySpecVerificationFieldDetails;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "additional" => Deserialize::begin(&mut self.additional),
                "minimum" => Deserialize::begin(&mut self.minimum),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { additional: Deserialize::default(), minimum: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(additional), Some(minimum)) = (self.additional.take(), self.minimum.take())
            else {
                return None;
            };
            Some(Self::Out { additional, minimum })
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

    impl ObjectDeser for CountrySpecVerificationFieldDetails {
        type Builder = CountrySpecVerificationFieldDetailsBuilder;
    }

    impl FromValueOpt for CountrySpecVerificationFieldDetails {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = CountrySpecVerificationFieldDetailsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "additional" => b.additional = FromValueOpt::from_value(v),
                    "minimum" => b.minimum = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
