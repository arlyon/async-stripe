/// An active entitlement describes access to a feature for a customer.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct EntitlementsActiveEntitlement {
    /// The [Feature](https://docs.stripe.com/api/entitlements/feature) that the customer is entitled to.
    pub feature: stripe_types::Expandable<stripe_shared::EntitlementsFeature>,
    /// Unique identifier for the object.
    pub id: stripe_misc::EntitlementsActiveEntitlementId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
    /// A unique key you provide as your own system identifier. This may be up to 80 characters.
    pub lookup_key: String,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for EntitlementsActiveEntitlement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("EntitlementsActiveEntitlement").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: EntitlementsActiveEntitlementBuilder {
                    feature: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                    lookup_key: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "feature" => Deserialize::begin(&mut self.builder.feature),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                "lookup_key" => Deserialize::begin(&mut self.builder.lookup_key),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(feature), Some(id), Some(livemode), Some(lookup_key)) = (
                self.builder.feature.take(),
                self.builder.id.take(),
                self.builder.livemode,
                self.builder.lookup_key.take(),
            ) else {
                return Ok(());
            };
            *self.out = Some(EntitlementsActiveEntitlement { feature, id, livemode, lookup_key });
            Ok(())
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
