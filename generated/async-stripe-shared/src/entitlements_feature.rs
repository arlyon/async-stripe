/// A feature represents a monetizable ability or functionality in your system.
/// Features can be assigned to products, and when those products are purchased, Stripe will create an entitlement to the feature for the purchasing customer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EntitlementsFeature {
    /// Inactive features cannot be attached to new products and will not be returned from the features list endpoint.
    pub active: bool,
    /// Unique identifier for the object.
    pub id: stripe_shared::EntitlementsFeatureId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// A unique key you provide as your own system identifier. This may be up to 80 characters.
    pub lookup_key: String,
    /// Set of key-value pairs that you can attach to an object.
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: std::collections::HashMap<String, String>,
    /// The feature's name, for your own purpose, not meant to be displayable to the customer.
    pub name: String,
}
#[doc(hidden)]
pub struct EntitlementsFeatureBuilder {
    active: Option<bool>,
    id: Option<stripe_shared::EntitlementsFeatureId>,
    livemode: Option<bool>,
    lookup_key: Option<String>,
    metadata: Option<std::collections::HashMap<String, String>>,
    name: Option<String>,
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
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for EntitlementsFeature {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<EntitlementsFeature>,
        builder: EntitlementsFeatureBuilder,
    }

    impl Visitor for Place<EntitlementsFeature> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: EntitlementsFeatureBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for EntitlementsFeatureBuilder {
        type Out = EntitlementsFeature;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "active" => Deserialize::begin(&mut self.active),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "lookup_key" => Deserialize::begin(&mut self.lookup_key),
                "metadata" => Deserialize::begin(&mut self.metadata),
                "name" => Deserialize::begin(&mut self.name),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                active: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                lookup_key: Deserialize::default(),
                metadata: Deserialize::default(),
                name: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (
                Some(active),
                Some(id),
                Some(livemode),
                Some(lookup_key),
                Some(metadata),
                Some(name),
            ) = (
                self.active,
                self.id.take(),
                self.livemode,
                self.lookup_key.take(),
                self.metadata.take(),
                self.name.take(),
            )
            else {
                return None;
            };
            Some(Self::Out { active, id, livemode, lookup_key, metadata, name })
        }
    }

    impl<'a> Map for Builder<'a> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            self.builder.key(k)
        }

        fn finish(&mut self) -> Result<()> {
            *self.out = self.builder.take_out();
            Ok(())
        }
    }

    impl ObjectDeser for EntitlementsFeature {
        type Builder = EntitlementsFeatureBuilder;
    }

    impl FromValueOpt for EntitlementsFeature {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = EntitlementsFeatureBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "active" => b.active = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "lookup_key" => b.lookup_key = FromValueOpt::from_value(v),
                    "metadata" => b.metadata = FromValueOpt::from_value(v),
                    "name" => b.name = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for EntitlementsFeature {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("EntitlementsFeature", 7)?;
        s.serialize_field("active", &self.active)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("lookup_key", &self.lookup_key)?;
        s.serialize_field("metadata", &self.metadata)?;
        s.serialize_field("name", &self.name)?;

        s.serialize_field("object", "entitlements.feature")?;
        s.end()
    }
}
impl stripe_types::Object for EntitlementsFeature {
    type Id = stripe_shared::EntitlementsFeatureId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(EntitlementsFeatureId);
