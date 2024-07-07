/// A supplier of carbon removal.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deserialize", derive(serde::Deserialize))]
pub struct ClimateSupplier {
    /// Unique identifier for the object.
    pub id: stripe_misc::ClimateSupplierId,
    /// Link to a webpage to learn more about the supplier.
    pub info_url: String,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The locations in which this supplier operates.
    pub locations: Vec<stripe_misc::ClimateRemovalsLocation>,
    /// Name of this carbon removal supplier.
    pub name: String,
    /// The scientific pathway used for carbon removal.
    pub removal_pathway: ClimateSupplierRemovalPathway,
}
#[doc(hidden)]
pub struct ClimateSupplierBuilder {
    id: Option<stripe_misc::ClimateSupplierId>,
    info_url: Option<String>,
    livemode: Option<bool>,
    locations: Option<Vec<stripe_misc::ClimateRemovalsLocation>>,
    name: Option<String>,
    removal_pathway: Option<ClimateSupplierRemovalPathway>,
}

#[allow(unused_variables, clippy::match_single_binding, clippy::single_match)]
const _: () = {
    use miniserde::de::{Map, Visitor};
    use miniserde::json::Value;
    use miniserde::{make_place, Deserialize, Result};
    use stripe_types::miniserde_helpers::FromValueOpt;
    use stripe_types::{MapBuilder, ObjectDeser};

    make_place!(Place);

    impl Deserialize for ClimateSupplier {
        fn begin(out: &mut Option<Self>) -> &mut dyn Visitor {
            Place::new(out)
        }
    }

    struct Builder<'a> {
        out: &'a mut Option<ClimateSupplier>,
        builder: ClimateSupplierBuilder,
    }

    impl Visitor for Place<ClimateSupplier> {
        fn map(&mut self) -> Result<Box<dyn Map + '_>> {
            Ok(Box::new(Builder {
                out: &mut self.out,
                builder: ClimateSupplierBuilder::deser_default(),
            }))
        }
    }

    impl MapBuilder for ClimateSupplierBuilder {
        type Out = ClimateSupplier;
        fn key(&mut self, k: &str) -> Result<&mut dyn Visitor> {
            Ok(match k {
                "id" => Deserialize::begin(&mut self.id),
                "info_url" => Deserialize::begin(&mut self.info_url),
                "livemode" => Deserialize::begin(&mut self.livemode),
                "locations" => Deserialize::begin(&mut self.locations),
                "name" => Deserialize::begin(&mut self.name),
                "removal_pathway" => Deserialize::begin(&mut self.removal_pathway),

                _ => <dyn Visitor>::ignore(),
            })
        }

        fn deser_default() -> Self {
            Self {
                id: Deserialize::default(),
                info_url: Deserialize::default(),
                livemode: Deserialize::default(),
                locations: Deserialize::default(),
                name: Deserialize::default(),
                removal_pathway: Deserialize::default(),
            }
        }

        fn take_out(&mut self) -> Option<Self::Out> {
            Some(Self::Out {
                id: self.id.take()?,
                info_url: self.info_url.take()?,
                livemode: self.livemode?,
                locations: self.locations.take()?,
                name: self.name.take()?,
                removal_pathway: self.removal_pathway?,
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

    impl ObjectDeser for ClimateSupplier {
        type Builder = ClimateSupplierBuilder;
    }

    impl FromValueOpt for ClimateSupplier {
        fn from_value(v: Value) -> Option<Self> {
            let Value::Object(obj) = v else {
                return None;
            };
            let mut b = ClimateSupplierBuilder::deser_default();
            for (k, v) in obj {
                match k.as_str() {
                    "id" => b.id = Some(FromValueOpt::from_value(v)?),
                    "info_url" => b.info_url = Some(FromValueOpt::from_value(v)?),
                    "livemode" => b.livemode = Some(FromValueOpt::from_value(v)?),
                    "locations" => b.locations = Some(FromValueOpt::from_value(v)?),
                    "name" => b.name = Some(FromValueOpt::from_value(v)?),
                    "removal_pathway" => b.removal_pathway = Some(FromValueOpt::from_value(v)?),

                    _ => {}
                }
            }
            b.take_out()
        }
    }
};
#[cfg(feature = "serialize")]
impl serde::Serialize for ClimateSupplier {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = s.serialize_struct("ClimateSupplier", 7)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("info_url", &self.info_url)?;
        s.serialize_field("livemode", &self.livemode)?;
        s.serialize_field("locations", &self.locations)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("removal_pathway", &self.removal_pathway)?;

        s.serialize_field("object", "climate.supplier")?;
        s.end()
    }
}
/// The scientific pathway used for carbon removal.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ClimateSupplierRemovalPathway {
    BiomassCarbonRemovalAndStorage,
    DirectAirCapture,
    EnhancedWeathering,
}
impl ClimateSupplierRemovalPathway {
    pub fn as_str(self) -> &'static str {
        use ClimateSupplierRemovalPathway::*;
        match self {
            BiomassCarbonRemovalAndStorage => "biomass_carbon_removal_and_storage",
            DirectAirCapture => "direct_air_capture",
            EnhancedWeathering => "enhanced_weathering",
        }
    }
}

impl std::str::FromStr for ClimateSupplierRemovalPathway {
    type Err = stripe_types::StripeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ClimateSupplierRemovalPathway::*;
        match s {
            "biomass_carbon_removal_and_storage" => Ok(BiomassCarbonRemovalAndStorage),
            "direct_air_capture" => Ok(DirectAirCapture),
            "enhanced_weathering" => Ok(EnhancedWeathering),
            _ => Err(stripe_types::StripeParseError),
        }
    }
}
impl std::fmt::Display for ClimateSupplierRemovalPathway {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ClimateSupplierRemovalPathway {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
#[cfg(feature = "serialize")]
impl serde::Serialize for ClimateSupplierRemovalPathway {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl miniserde::Deserialize for ClimateSupplierRemovalPathway {
    fn begin(out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        crate::Place::new(out)
    }
}

impl miniserde::de::Visitor for crate::Place<ClimateSupplierRemovalPathway> {
    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        use std::str::FromStr;
        self.out = Some(ClimateSupplierRemovalPathway::from_str(s).map_err(|_| miniserde::Error)?);
        Ok(())
    }
}

stripe_types::impl_from_val_with_from_str!(ClimateSupplierRemovalPathway);
#[cfg(feature = "deserialize")]
impl<'de> serde::Deserialize<'de> for ClimateSupplierRemovalPathway {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use std::str::FromStr;
        let s: std::borrow::Cow<'de, str> = serde::Deserialize::deserialize(deserializer)?;
        Self::from_str(&s).map_err(|_| {
            serde::de::Error::custom("Unknown value for ClimateSupplierRemovalPathway")
        })
    }
}
impl stripe_types::Object for ClimateSupplier {
    type Id = stripe_misc::ClimateSupplierId;
    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn into_id(self) -> Self::Id {
        self.id
    }
}
stripe_types::def_id!(ClimateSupplierId);
