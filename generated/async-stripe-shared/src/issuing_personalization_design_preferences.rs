#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPersonalizationDesignPreferences {
    /// Whether we use this personalization design to create cards when one isn't specified.
    /// A connected account uses the Connect platform's default design if no personalization design is set as the default design.
    pub is_default: bool,
    /// Whether this personalization design is used to create cards when one is not specified and a default for this connected account does not exist.
    pub is_platform_default: Option<bool>,
}
#[doc(hidden)]
pub struct IssuingPersonalizationDesignPreferencesBuilder {
    is_default: Option<bool>,
    is_platform_default: Option<Option<bool>>,
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

    impl Deserialize for IssuingPersonalizationDesignPreferences {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingPersonalizationDesignPreferences>,
        builder: IssuingPersonalizationDesignPreferencesBuilder,
    }

    impl Visitor for Place<IssuingPersonalizationDesignPreferences> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingPersonalizationDesignPreferencesBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingPersonalizationDesignPreferencesBuilder {
        type Out = IssuingPersonalizationDesignPreferences;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "is_default" => Deserialize::begin(&mut self.is_default),
                "is_platform_default" => Deserialize::begin(&mut self.is_platform_default),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { is_default: Deserialize::default(), is_platform_default: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(is_default), Some(is_platform_default)) =
                (self.is_default, self.is_platform_default)
            else {
                return None;
            };
            Some(Self::Out { is_default, is_platform_default })
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

    impl ObjectDeser for IssuingPersonalizationDesignPreferences {
        type Builder = IssuingPersonalizationDesignPreferencesBuilder;
    }

    impl FromValueOpt for IssuingPersonalizationDesignPreferences {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingPersonalizationDesignPreferencesBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "is_default" => b.is_default = FromValueOpt::from_value(v),
                    "is_platform_default" => b.is_platform_default = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
