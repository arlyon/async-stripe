/// Tax locations represent venues for services, tickets, or other product types.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct TaxLocation {
    pub address: stripe_shared::Address,
    /// A descriptive text providing additional context about the tax location.
    /// This can include information about the venue, types of events held, services available, or any relevant details for better identification (for example, "A spacious auditorium suitable for large concerts and events.").
    pub description: Option<String>,
    /// Unique identifier for the object.
    pub id: stripe_misc::TaxLocationId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// The type of tax location to be defined. Currently the only option is `performance`.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_misc::TaxLocationType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("TaxLocation").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct TaxLocationBuilder {
    address: Option<stripe_shared::Address>,
    description: Option<Option<String>>,
    id: Option<stripe_misc::TaxLocationId>,
    livemode: Option<bool>,
    type_: Option<stripe_misc::TaxLocationType>,
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

    impl Deserialize for TaxLocation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<TaxLocation>,
        builder: TaxLocationBuilder,
    }

    impl Visitor for Place<TaxLocation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: TaxLocationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for TaxLocationBuilder {
        type Out = TaxLocation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "address" => Deserialize::begin(&mut self.address),
                "description" => Deserialize::begin(&mut self.description),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { address: None, description: Some(None), id: None, livemode: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(address), Some(description), Some(id), Some(livemode), Some(type_)) = (
                self.address.take(),
                self.description.take(),
                self.id.take(),
                self.livemode,
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { address, description, id, livemode, type_ })
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

    impl ObjectDeser for TaxLocation {
        type Builder = TaxLocationBuilder;
    }

    impl FromValueOpt for TaxLocation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = TaxLocationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "address" => b.address = FromValueOpt::from_value(v),
                    "description" => b.description = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for TaxLocation {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("TaxLocation", 6)?;
        s.serialize_field("address", &self.address)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "tax.location")?;
        s.end()
    }
}
impl stripe_types::Object for TaxLocation {
    type Id = stripe_misc::TaxLocationId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(TaxLocationId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum TaxLocationType {
    Performance,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl TaxLocationType {
    pub fn as_str(&self) -> &str {
        use TaxLocationType::*;
        match self {
            Performance => "performance",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for TaxLocationType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use TaxLocationType::*;
        match s {
            "performance" => Ok(Performance),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "TaxLocationType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for TaxLocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for TaxLocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for TaxLocationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(TaxLocationType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for TaxLocationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for TaxLocationType {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<TaxLocationType> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(TaxLocationType::from_str(s).expect("infallible"));
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(TaxLocationType);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for TaxLocationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
