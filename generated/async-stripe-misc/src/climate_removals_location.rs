#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ClimateRemovalsLocation {
    /// The city where the supplier is located.
    pub city: Option<String>,
    /// Two-letter ISO code representing the country where the supplier is located.
    pub country: String,
    /// The geographic latitude where the supplier is located.
    pub latitude: Option<f64>,
    /// The geographic longitude where the supplier is located.
    pub longitude: Option<f64>,
    /// The state/county/province/region where the supplier is located.
    pub region: Option<String>,
}
#[doc(hidden)]
pub struct ClimateRemovalsLocationBuilder {
    city: Option<Option<String>>,
    country: Option<String>,
    latitude: Option<Option<f64>>,
    longitude: Option<Option<f64>>,
    region: Option<Option<String>>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ClimateRemovalsLocation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ClimateRemovalsLocation>,
        builder: ClimateRemovalsLocationBuilder,
    }

    impl Visitor for Place<ClimateRemovalsLocation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ClimateRemovalsLocationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ClimateRemovalsLocationBuilder {
        type Out = ClimateRemovalsLocation;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "city" => Deserialize::begin(&mut self.city),
                "country" => Deserialize::begin(&mut self.country),
                "latitude" => Deserialize::begin(&mut self.latitude),
                "longitude" => Deserialize::begin(&mut self.longitude),
                "region" => Deserialize::begin(&mut self.region),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                city: Deserialize::default(),
                country: Deserialize::default(),
                latitude: Deserialize::default(),
                longitude: Deserialize::default(),
                region: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                city: self.city.take()?,
                country: self.country.take()?,
                latitude: self.latitude?,
                longitude: self.longitude?,
                region: self.region.take()?,
            })
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

    impl ObjectDeser for ClimateRemovalsLocation {
        type Builder = ClimateRemovalsLocationBuilder;
    }

    impl FromValueOpt for ClimateRemovalsLocation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ClimateRemovalsLocationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "city" => b.city = Some(FromValueOpt::from_value(v)?),
                    "country" => b.country = Some(FromValueOpt::from_value(v)?),
                    "latitude" => b.latitude = Some(FromValueOpt::from_value(v)?),
                    "longitude" => b.longitude = Some(FromValueOpt::from_value(v)?),
                    "region" => b.region = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
