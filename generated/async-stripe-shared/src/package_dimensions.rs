#[derive(Copy, Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct PackageDimensions {
    /// Height, in inches.
    pub height: f64,
    /// Length, in inches.
    pub length: f64,
    /// Weight, in ounces.
    pub weight: f64,
    /// Width, in inches.
    pub width: f64,
}
#[doc(hidden)]
pub struct PackageDimensionsBuilder {
    height: Option<f64>,
    length: Option<f64>,
    weight: Option<f64>,
    width: Option<f64>,
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

    impl Deserialize for PackageDimensions {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<PackageDimensions>,
        builder: PackageDimensionsBuilder,
    }

    impl Visitor for Place<PackageDimensions> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: PackageDimensionsBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for PackageDimensionsBuilder {
        type Out = PackageDimensions;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "height" => Deserialize::begin(&mut self.height),
                "length" => Deserialize::begin(&mut self.length),
                "weight" => Deserialize::begin(&mut self.weight),
                "width" => Deserialize::begin(&mut self.width),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                height: Deserialize::default(),
                length: Deserialize::default(),
                weight: Deserialize::default(),
                width: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            let (Some(height), Some(length), Some(weight), Some(width)) =
                (self.height, self.length, self.weight, self.width)
            else {
                return None;
            };
            Some(Self::Out { height, length, weight, width })
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

    impl ObjectDeser for PackageDimensions {
        type Builder = PackageDimensionsBuilder;
    }

    impl FromValueOpt for PackageDimensions {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = PackageDimensionsBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "height" => b.height = FromValueOpt::from_value(v),
                    "length" => b.length = FromValueOpt::from_value(v),
                    "weight" => b.weight = FromValueOpt::from_value(v),
                    "width" => b.width = FromValueOpt::from_value(v),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
