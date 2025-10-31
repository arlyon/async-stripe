/// An active entitlement describes access to a feature for a customer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EntitlementsActiveEntitlement {
    /// The [Feature](https://stripe.com/docs/api/entitlements/feature) that the customer is entitled to.
    pub feature: stripe_types::Expandable<stripe_shared::EntitlementsFeature>,
    /// Unique identifier for the object.
    pub id: stripe_misc::EntitlementsActiveEntitlementId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// A unique key you provide as your own system identifier. This may be up to 80 characters.
    pub lookup_key: String,
}
#[doc(hidden)]
pub struct EntitlementsActiveEntitlementBuilder {
    feature: Option<stripe_types::Expandable<stripe_shared::EntitlementsFeature>>,
    id: Option<stripe_misc::EntitlementsActiveEntitlementId>,
    livemode: Option<bool>,
    lookup_key: Option<String>,
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

    impl Deserialize for EntitlementsActiveEntitlement {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<EntitlementsActiveEntitlement>,
        builder: EntitlementsActiveEntitlementBuilder,
    }

    impl Visitor for Place<EntitlementsActiveEntitlement> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: EntitlementsActiveEntitlementBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for EntitlementsActiveEntitlementBuilder {
        type Out = EntitlementsActiveEntitlement;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "feature" => Deserialize::begin(&mut self.feature),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "lookup_key" => Deserialize::begin(&mut self.lookup_key),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                feature: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
                lookup_key: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(feature), Some(id), Some(livemode), Some(lookup_key)) =
                (self.feature.take(), self.id.take(), self.livemode, self.lookup_key.take())
            else {
                return None;
            };
            Some(Self::Out { feature, id, livemode, lookup_key })
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

    impl ObjectDeser for EntitlementsActiveEntitlement {
        type Builder = EntitlementsActiveEntitlementBuilder;
    }

    impl FromValueOpt for EntitlementsActiveEntitlement {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = EntitlementsActiveEntitlementBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "feature" => b.feature = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),
                    "lookup_key" => b.lookup_key = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for EntitlementsActiveEntitlement {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("EntitlementsActiveEntitlement", 5)?;
        s.serialize_field("feature", &self.feature)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("lookup_key", &self.lookup_key)?;

        s.serialize_field("object", "entitlements.active_entitlement")?;
        s.end()
    }
}
impl stripe_types::Object for EntitlementsActiveEntitlement {
    type Id = stripe_misc::EntitlementsActiveEntitlementId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(EntitlementsActiveEntitlementId);
