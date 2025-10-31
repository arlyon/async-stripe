#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsDefault {
    /// Type of registration in `country`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsDefaultType,
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsDefaultBuilder {
    type_: Option<TaxProductRegistrationsResourceCountryOptionsDefaultType>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsDefault {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
        builder: TaxProductRegistrationsResourceCountryOptionsDefaultBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsDefault> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxProductRegistrationsResourceCountryOptionsDefaultBuilder::deser_default(
                ),
            }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsDefaultBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsDefault;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { type_: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(type_),) = (self.type_,) else {
                return None;
            };
            Some(Self::Out { type_ })
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsDefault {
        type Builder = TaxProductRegistrationsResourceCountryOptionsDefaultBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsDefault {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TaxProductRegistrationsResourceCountryOptionsDefaultBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
/// Type of registration in `country`.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TaxProductRegistrationsResourceCountryOptionsDefaultType {
    Standard,
}
impl TaxProductRegistrationsResourceCountryOptionsDefaultType {
    pub fn as_str(self) -> &'static str {
        use TaxProductRegistrationsResourceCountryOptionsDefaultType::*;
        match self {
            Standard => "standard",
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsDefaultType::*;
        match s {
            "standard" => Ok(Standard),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TaxProductRegistrationsResourceCountryOptionsDefaultType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductRegistrationsResourceCountryOptionsDefaultType::from_str(s)
                .map_err(|_| miniserde::Error)?,
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TaxProductRegistrationsResourceCountryOptionsDefaultType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsDefaultType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom(
                "Unknown value for TaxProductRegistrationsResourceCountryOptionsDefaultType",
            )
        })
    }
}
