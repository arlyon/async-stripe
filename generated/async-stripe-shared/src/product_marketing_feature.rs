#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ProductMarketingFeature {
    /// The marketing feature name. Up to 80 characters long.
    pub name: Option<String>,
}
#[doc(hidden)]
pub struct ProductMarketingFeatureBuilder {
    name: Option<Option<String>>,
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

    impl Deserialize for ProductMarketingFeature {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ProductMarketingFeature>,
        builder: ProductMarketingFeatureBuilder,
    }

    impl Visitor for Place<ProductMarketingFeature> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ProductMarketingFeatureBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ProductMarketingFeatureBuilder {
        type Out = ProductMarketingFeature;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "name" => Deserialize::begin(&mut self.name),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self { name: Deserialize::default() }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(name),) = (self.name.take(),) else {
                return None;
            };
            Some(Self::Out { name })
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

    impl ObjectDeser for ProductMarketingFeature {
        type Builder = ProductMarketingFeatureBuilder;
    }

    impl FromValueOpt for ProductMarketingFeature {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ProductMarketingFeatureBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "name" => b.name = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
