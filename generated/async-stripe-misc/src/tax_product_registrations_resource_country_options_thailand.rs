#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxProductRegistrationsResourceCountryOptionsThailand {
    /// Type of registration in `country`.
    #[cfg_attr(any(feature = "deserialize", feature = "serialize"), serde(rename = "type"))]
    pub type_: TaxProductRegistrationsResourceCountryOptionsThailandType,
}
#[doc(hidden)]
pub struct TaxProductRegistrationsResourceCountryOptionsThailandBuilder {
    type_: Option<TaxProductRegistrationsResourceCountryOptionsThailandType>,
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

    impl Deserialize for TaxProductRegistrationsResourceCountryOptionsThailand {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxProductRegistrationsResourceCountryOptionsThailand>,
        builder: TaxProductRegistrationsResourceCountryOptionsThailandBuilder,
    }

    impl Visitor for Place<TaxProductRegistrationsResourceCountryOptionsThailand> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder:
                    TaxProductRegistrationsResourceCountryOptionsThailandBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxProductRegistrationsResourceCountryOptionsThailandBuilder {
        type Out = TaxProductRegistrationsResourceCountryOptionsThailand;
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
            let (Some(type_),) = (self.type_.take(),) else {
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

    impl ObjectDeser for TaxProductRegistrationsResourceCountryOptionsThailand {
        type Builder = TaxProductRegistrationsResourceCountryOptionsThailandBuilder;
    }

    impl FromValueOpt for TaxProductRegistrationsResourceCountryOptionsThailand {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b =
                TaxProductRegistrationsResourceCountryOptionsThailandBuilder::deser_default();
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
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxProductRegistrationsResourceCountryOptionsThailandType {
    Simplified,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxProductRegistrationsResourceCountryOptionsThailandType {
    pub fn as_str(&self) -> &str {
        use TaxProductRegistrationsResourceCountryOptionsThailandType::*;
        match self {
            Simplified => "simplified",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxProductRegistrationsResourceCountryOptionsThailandType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxProductRegistrationsResourceCountryOptionsThailandType::*;
        match s {
            "simplified" => Ok(Simplified),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "TaxProductRegistrationsResourceCountryOptionsThailandType"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptionsThailandType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for TaxProductRegistrationsResourceCountryOptionsThailandType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxProductRegistrationsResourceCountryOptionsThailandType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxProductRegistrationsResourceCountryOptionsThailandType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor
    for crate::Place<TaxProductRegistrationsResourceCountryOptionsThailandType>
{
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(
            TaxProductRegistrationsResourceCountryOptionsThailandType::from_str(s)
                .expect("infallible"),
        );
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(
    TaxProductRegistrationsResourceCountryOptionsThailandType
);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxProductRegistrationsResourceCountryOptionsThailandType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
