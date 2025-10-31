#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize))]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct RadarReviewResourceLocation {
    /// The city where the payment originated.
    pub city: Option<String>,
    /// Two-letter ISO code representing the country where the payment originated.
    pub country: Option<String>,
    /// The geographic latitude where the payment originated.
    pub latitude: Option<f64>,
    /// The geographic longitude where the payment originated.
    pub longitude: Option<f64>,
    /// The state/county/province/region where the payment originated.
    pub region: Option<String>,
}
#[doc(hidden)]
pub struct RadarReviewResourceLocationBuilder {
    city: Option<Option<String>>,
    country: Option<Option<String>>,
    latitude: Option<Option<f64>>,
    longitude: Option<Option<f64>>,
    region: Option<Option<String>>,
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

    impl Deserialize for RadarReviewResourceLocation {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<RadarReviewResourceLocation>,
        builder: RadarReviewResourceLocationBuilder,
    }

    impl Visitor for Place<RadarReviewResourceLocation> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: RadarReviewResourceLocationBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for RadarReviewResourceLocationBuilder {
        type Out = RadarReviewResourceLocation;
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
            let (Some(city), Some(country), Some(latitude), Some(longitude), Some(region)) = (
                self.city.take(),
                self.country.take(),
                self.latitude,
                self.longitude,
                self.region.take(),
            ) else {
                return None;
            };
            Some(Self::Out { city, country, latitude, longitude, region })
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

    impl ObjectDeser for RadarReviewResourceLocation {
        type Builder = RadarReviewResourceLocationBuilder;
    }

    impl FromValueOpt for RadarReviewResourceLocation {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = RadarReviewResourceLocationBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "city" => b.city = FromValueOpt::from_value(v),
                    "country" => b.country = FromValueOpt::from_value(v),
                    "latitude" => b.latitude = FromValueOpt::from_value(v),
                    "longitude" => b.longitude = FromValueOpt::from_value(v),
                    "region" => b.region = FromValueOpt::from_value(v),
                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
