/// A supplier of carbon removal.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
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
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use ClimateSupplierRemovalPathway::*;
        match s {
            "biomass_carbon_removal_and_storage" => Ok(BiomassCarbonRemovalAndStorage),
            "direct_air_capture" => Ok(DirectAirCapture),
            "enhanced_weathering" => Ok(EnhancedWeathering),
            _ => Err(()),
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
impl serde::Serialize for ClimateSupplierRemovalPathway {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
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
}
stripe_types::def_id!(ClimateSupplierId);
