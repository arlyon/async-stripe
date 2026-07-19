#[derive(Clone)]
#[cfg_attr(not(feature = "redact-generated-debug"), derive(Debug))]
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
#[cfg(feature = "redact-generated-debug")]
impl std::fmt::Debug for ClimateRemovalsLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("ClimateRemovalsLocation").finish_non_exhaustive()
    }
}
#[doc(hidden)]
pub struct ClimateRemovalsLocationBuilder {
    city: Option<Option<String>>,
    country: Option<String>,
    latitude: Option<Option<f64>>,
    longitude: Option<Option<f64>>,
    region: Option<Option<String>>,
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
                builder: ClimateRemovalsLocationBuilder {
                    city: Deserialize::default(),
                    country: Deserialize::default(),
                    latitude: Deserialize::default(),
                    longitude: Deserialize::default(),
                    region: Deserialize::default(),
                },
            }))
        }
    }

    impl Map for Builder<'_> {
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "city" => Deserialize::begin(&mut self.builder.city),
                "country" => Deserialize::begin(&mut self.builder.country),
                "latitude" => Deserialize::begin(&mut self.builder.latitude),
                "longitude" => Deserialize::begin(&mut self.builder.longitude),
                "region" => Deserialize::begin(&mut self.builder.region),
                _ => <dyn Visitor>::ignore(),
            })
        }

        fn finish(&mut self) -> Result<()> {
            let (Some(city), Some(country), Some(latitude), Some(longitude), Some(region)) = (
                self.builder.city.take(),
                self.builder.country.take(),
                self.builder.latitude,
                self.builder.longitude,
                self.builder.region.take(),
            ) else {
                return Ok(());
            };
            *self.out =
                Some(ClimateRemovalsLocation { city, country, latitude, longitude, region });
            Ok(())
        }
    }
};
