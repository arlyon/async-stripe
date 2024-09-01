/// A product_feature represents an attachment between a feature and a product.
/// When a product is purchased that has a feature attached, Stripe will create an entitlement to the feature for the purchasing customer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ProductFeature {
    pub entitlement_feature: stripe_shared::EntitlementsFeature,
    /// Unique identifier for the object.
    pub id: stripe_product::ProductFeatureId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
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
                builder: ProductFeatureBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ProductFeatureBuilder {
        type Out = ProductFeature;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "entitlement_feature" => Deserialize::begin(&mut self.entitlement_feature),
                "id" => Deserialize::begin(&mut self.id),
                "livemode" => Deserialize::begin(&mut self.livemode),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                entitlement_feature: Deserialize::default(),
                id: Deserialize::default(),
                livemode: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(entitlement_feature), Some(id), Some(livemode)) =
                (self.entitlement_feature.take(), self.id.take(), self.livemode)
            else {
                return None;
            };
            Some(Self::Out { entitlement_feature, id, livemode })
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

    impl ObjectDeser for ProductFeature {
        type Builder = ProductFeatureBuilder;
    }

    impl FromValueOpt for ProductFeature {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ProductFeatureBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "entitlement_feature" => b.entitlement_feature = FromValueOpt::from_value(v),
                    "id" => b.id = FromValueOpt::from_value(v),
                    "livemode" => b.livemode = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
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
