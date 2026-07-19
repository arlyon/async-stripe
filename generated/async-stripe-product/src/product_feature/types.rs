/// A product_feature represents an attachment between a feature and a product.
/// When a product is purchased that has a feature attached, Stripe will create an entitlement to the feature for the purchasing customer.
#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ProductFeature {
    pub entitlement_feature: stripe_shared::EntitlementsFeature,
    /// Unique identifier for the object.
    pub id: stripe_product::ProductFeatureId,
    /// If the object exists in live mode, the value is `true`.
    /// If the object exists in test mode, the value is `false`.
    pub livemode: bool,
}
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ProductFeature {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ProductFeature").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ProductFeatureBuilder {
    entitlement_feature: Option<stripe_shared::EntitlementsFeature>,
    id: Option<stripe_product::ProductFeatureId>,
    livemode: Option<bool>,
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

    impl Deserialize for ProductFeature {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ProductFeature>,
        builder: ProductFeatureBuilder,
    }

    impl Visitor for Place<ProductFeature> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ProductFeatureBuilder {
                    entitlement_feature: Deserialize::default(),
                    id: Deserialize::default(),
                    livemode: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "entitlement_feature" => Deserialize::begin(&mut self.builder.entitlement_feature),
                "id" => Deserialize::begin(&mut self.builder.id),
                "livemode" => Deserialize::begin(&mut self.builder.livemode),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(entitlement_feature), Some(id), Some(livemode)) = (
                self.builder.entitlement_feature.take(),
                self.builder.id.take(),
                self.builder.livemode,
            ) else {
                return Ok(());
            };
            *self.out = Some(ProductFeature { entitlement_feature, id, livemode });
            Ok(())
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ProductFeature {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ProductFeature", 4)?;
        s.serialize_field("entitlement_feature", &self.entitlement_feature)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("livemode", &self.livemode)?;

        s.serialize_field("object", "product_feature")?;
        s.end()
    }
}
impl stripe_types::Object for ProductFeature {
    type Id = stripe_product::ProductFeatureId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ProductFeatureId);
