#[derive(Copy, Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for PackageDimensions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("PackageDimensions").finish_non_exhaustive()
    }
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
    dead_code,
    clippy::let_unit_value,
    clippy::match_single_binding,
    clippy::single_match
)]
const _: () = {
    use stripe_miniserde::de::{Map, Visitor};
    use stripe_miniserde::{Deserialize, Result, make_place};

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
                builder: PackageDimensionsBuilder {
                    height: Deserialize::default(),
                    length: Deserialize::default(),
                    weight: Deserialize::default(),
                    width: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "height" => Deserialize::begin(&mut self.builder.height),
                "length" => Deserialize::begin(&mut self.builder.length),
                "weight" => Deserialize::begin(&mut self.builder.weight),
                "width" => Deserialize::begin(&mut self.builder.width),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(height), Some(length), Some(weight), Some(width)) =
                (self.builder.height, self.builder.length, self.builder.weight, self.builder.width)
            else {
                return Ok(());
            };
            *self.out = Some(PackageDimensions { height, length, weight, width });
            Ok(())
        }
    }
};
