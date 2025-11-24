#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductResourceJurisdiction {
    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    /// A human-readable name for the jurisdiction imposing the tax.
    pub display_name: String,
    /// Indicates the level of the jurisdiction imposing the tax.
    pub level: TaxProductResourceJurisdictionLevel,
    /// [ISO 3166-2 subdivision code](https://en.wikipedia.org/wiki/ISO_3166-2), without country prefix.
    /// For example, "NY" for New York, United States.
    pub state: Option<String>,
}
#[doc(hidden)]
pub struct TaxProductResourceJurisdictionBuilder {
    country: Option<String>,
    display_name: Option<String>,
    level: Option<TaxProductResourceJurisdictionLevel>,
    state: Option<Option<String>>,
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

    impl Deserialize for TaxProductResourceJurisdiction {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductResourceJurisdiction>,
        builder: TaxProductResourceJurisdictionBuilder,
    }

    impl Visitor for Place<TaxProductResourceJurisdiction> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductResourceJurisdictionBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductResourceJurisdictionBuilder {
        type Out = TaxProductResourceJurisdiction;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "country" => Deserialize::begin(&mut self.country),
                "display_name" => Deserialize::begin(&mut self.display_name),
                "level" => Deserialize::begin(&mut self.level),
                "state" => Deserialize::begin(&mut self.state),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                country: Deserialize::default(),
                display_name: Deserialize::default(),
                level: Deserialize::default(),
                state: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(country), Some(display_name), Some(level), Some(state)) = (
                self.country.take(),
                self.display_name.take(),
                self.level.take(),
                self.state.take(),
            ) else {
                return None;
            };
            Some(Self::Out { country, display_name, level, state })
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

    impl ObjectDeser for TaxProductResourceJurisdiction {
        type Builder = TaxProductResourceJurisdictionBuilder;
    }

    impl FromValueOpt for TaxProductResourceJurisdiction {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxProductResourceJurisdictionBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "country" => b.country = FromValueOpt::from_value(v),
                    "display_name" => b.display_name = FromValueOpt::from_value(v),
                    "level" => b.level = FromValueOpt::from_value(v),
                    "state" => b.state = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Indicates the level of the jurisdiction imposing the tax.
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductResourceJurisdictionLevel {
    City,
    Country,
    County,
    District,
    State,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductResourceJurisdictionLevel {
    pub fn as_str(&self) -> &str {
        use TaxProductResourceJurisdictionLevel::*;
        match self {
            City => "city",
            Country => "country",
            County => "county",
            District => "district",
            State => "state",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductResourceJurisdictionLevel {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductResourceJurisdictionLevel::*;
        match s {
            "city" => Ok(City),
            "country" => Ok(Country),
            "county" => Ok(County),
            "district" => Ok(District),
            "state" => Ok(State),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductResourceJurisdictionLevel"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductResourceJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductResourceJurisdictionLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductResourceJurisdictionLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductResourceJurisdictionLevel {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxProductResourceJurisdictionLevel> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxProductResourceJurisdictionLevel::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxProductResourceJurisdictionLevel);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductResourceJurisdictionLevel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
