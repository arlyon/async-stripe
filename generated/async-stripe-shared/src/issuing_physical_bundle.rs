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
                builder: IssuingPhysicalBundleBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for IssuingPhysicalBundleBuilder {
        type Out = IssuingPhysicalBundle;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "features" => Deserialize::begin(&mut self.features),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "name" => Deserialize::begin(&mut self.name),
                "status" => Deserialize::begin(&mut self.status),
                "type" => Deserialize::begin(&mut self.type_),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { features: None, id: None, livemode: None, name: None, status: None, type_: None }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(features), Some(id), Some(livemode), Some(name), Some(status), Some(type_)) = (
                self.features.take(),
                self.id.take(),
                self.livemode,
                self.name.take(),
                self.status.take(),
                self.type_.take(),
            ) else {
                return None;
            };
            Some(Self::Out { features, id, livemode, name, status, type_ })
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

    impl ObjectDeser for IssuingPhysicalBundle {
        type Builder = IssuingPhysicalBundleBuilder;
    }

    impl FromValueOpt for IssuingPhysicalBundle {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = IssuingPhysicalBundleBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "features" => b.features = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),
                    "status" => b.status = FromValueOpt::from_value(v),
                    "type" => b.type_ = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
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
/// Whether this physical bundle can be used to create cards.
pub type IssuingPhysicalBundleStatus = stripe_shared::IssuingPhysicalBundleStatus;
/// Whether this physical bundle is a standard Stripe offering or custom-made for you.
pub type IssuingPhysicalBundleType = stripe_shared::IssuingPhysicalBundleType;
