/// A Physical Bundle represents the bundle of physical items - card stock, carrier letter, and envelope - that is shipped to a cardholder when you create a physical card.
#[derive(Clone, Eq, PartialEq)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct IssuingPhysicalBundle {
    pub features: stripe_shared::IssuingPhysicalBundleFeatures,
    /// Unique identifier for the object.
    pub id: stripe_shared::IssuingPhysicalBundleId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// Friendly display name.
    pub name: String,
    /// Whether this physical bundle can be used to create cards.
    pub status: stripe_shared::IssuingPhysicalBundleStatus,
    /// Whether this physical bundle is a standard Stripe offering or custom-made for you.
    #[cfg_attr(feature = "deserialize", serde(rename = "type"))]
    pub type_: stripe_shared::IssuingPhysicalBundleType,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPhysicalBundle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IssuingPhysicalBundle").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct IssuingPhysicalBundleBuilder {
    features: Option<stripe_shared::IssuingPhysicalBundleFeatures>,
    id: Option<stripe_shared::IssuingPhysicalBundleId>,
    livemode: Option<bool>,
    name: Option<String>,
    status: Option<stripe_shared::IssuingPhysicalBundleStatus>,
    type_: Option<stripe_shared::IssuingPhysicalBundleType>,
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

    impl Deserialize for IssuingPhysicalBundle {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<IssuingPhysicalBundle>,
        builder: IssuingPhysicalBundleBuilder,
    }

    impl Visitor for Place<IssuingPhysicalBundle> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: IssuingPhysicalBundleBuilder {
                    features: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    name: Deserialize::default(),
                    status: Deserialize::default(),
                    type_: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "features" => Deserialize::begin(&mut self.builder.features),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "name" => Deserialize::begin(&mut self.builder.name),
                "status" => Deserialize::begin(&mut self.builder.status),
                "type" => Deserialize::begin(&mut self.builder.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(features), Some(id), Some(livemode), Some(name), Some(status), Some(type_)) = (
                self.builder.features.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.name.take(),
                self.builder.status.take(),
                self.builder.type_.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(IssuingPhysicalBundle { features, id, livemode, name, status, type_ });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for IssuingPhysicalBundle {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("IssuingPhysicalBundle", 7)?;
        s.serialize_field("features", &self.features)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("status", &self.status)?;
        s.serialize_field("type", &self.type_)?;

        s.serialize_field("object", "issuing.physical_bundle")?;
        s.end()
    }
}
impl stripe_types::Object for IssuingPhysicalBundle {
    type Id = stripe_shared::IssuingPhysicalBundleId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(IssuingPhysicalBundleId);
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingPhysicalBundleStatus {
    Active,
    Inactive,
    Review,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingPhysicalBundleStatus {
    pub fn as_str(&self) -> &str {
        use IssuingPhysicalBundleStatus::*;
        match self {
            Active => "active",
            Inactive => "inactive",
            Review => "review",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingPhysicalBundleStatus {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPhysicalBundleStatus::*;
        match s {
            "active" => Ok(Active),
            "inactive" => Ok(Inactive),
            "review" => Ok(Review),
            v => {
                tracing::warn!(
                    "Unknown value '{}' for enum '{}'",
                    v,
                    "IssuingPhysicalBundleStatus"
                );
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingPhysicalBundleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingPhysicalBundleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPhysicalBundleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingPhysicalBundleStatus)).finish_non_exhaustive()
    }
}
impl serde::Serialize for IssuingPhysicalBundleStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingPhysicalBundleStatus {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingPhysicalBundleStatus> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingPhysicalBundleStatus::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPhysicalBundleStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
#[derive(Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum IssuingPhysicalBundleType {
    Custom,
    Standard,
    /// An unrecognized value from Stripe. Should not be used as a request parameter.
    Unknown(String),
}
impl IssuingPhysicalBundleType {
    pub fn as_str(&self) -> &str {
        use IssuingPhysicalBundleType::*;
        match self {
            Custom => "custom",
            Standard => "standard",
            Unknown(v) => v,
        }
    }
}

impl std::str::FromStr for IssuingPhysicalBundleType {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use IssuingPhysicalBundleType::*;
        match s {
            "custom" => Ok(Custom),
            "standard" => Ok(Standard),
            v => {
                tracing::warn!("Unknown value '{}' for enum '{}'", v, "IssuingPhysicalBundleType");
                Ok(Unknown(v.to_owned()))
            }
        }
    }
}
impl std::fmt::Display for IssuingPhysicalBundleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(not(feature = "redact-generated-debug"))]
impl std::fmt::Debug for IssuingPhysicalBundleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for IssuingPhysicalBundleType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct(stringify!(IssuingPhysicalBundleType)).finish_non_exhaustive()
    }
}
impl serde::Serialize for IssuingPhysicalBundleType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl stripe_miniserde::Deserialize for IssuingPhysicalBundleType {
    fn begin(out: &mut Option<Self>) -> &mut dyn stripe_miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl stripe_miniserde::de::Visitor for crate::Place<IssuingPhysicalBundleType> {
    fn string(&mut self, s: &str) -> stripe_miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(IssuingPhysicalBundleType::from_str(s).expect("infallible"));
        Ok(())
    }
}
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for IssuingPhysicalBundleType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Ok(Self::from_str(&s).expect("infallible"))
    }
}
