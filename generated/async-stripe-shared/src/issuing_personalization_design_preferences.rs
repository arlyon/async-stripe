#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPersonalizationDesignPreferences {
    /// Whether we use this personalization design to create cards when one isn't specified.
    /// A connected account uses the Connect platform's default design if no personalization design is set as the default design.
    pub is_default: bool,
    /// Whether this personalization design is used to create cards when one is not specified and a default for this connected account does not exist.
    pub is_platform_default: Option<bool>,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPersonalizationDesignPreferences {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingPersonalizationDesignPreferences").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingPersonalizationDesignPreferencesBuilder {
    is_default: Option<bool>,
    is_platform_default: Option<Option<bool>>,
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
                builder: IssuingPersonalizationDesignPreferencesBuilder {
                    is_default: Deserialize::default(),
                    is_platform_default: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "is_default" => Deserialize::begin(&mut self.builder.is_default),
                "is_platform_default" => Deserialize::begin(&mut self.builder.is_platform_default),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(is_default), Some(is_platform_default)) =
                (self.builder.is_default, self.builder.is_platform_default)
            else {
                return Ok(());
            };
            *self.out =
                Some(IssuingPersonalizationDesignPreferences { is_default, is_platform_default });
            Ok(())
        }
    }
};
