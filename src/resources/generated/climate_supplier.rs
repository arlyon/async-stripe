// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ClimateSupplierId};
use crate::params::{Object};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ClimateRemovalsSuppliers".
///
/// For more details see <https://stripe.com/docs/api/climate/supplier/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClimateSupplier {
    /// Unique identifier for the object.
    pub id: ClimateSupplierId,

    /// Link to a webpage to learn more about the supplier.
    pub info_url: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The locations in which this supplier operates.
    pub locations: Vec<ClimateRemovalsLocation>,

    /// Name of this carbon removal supplier.
    pub name: String,

    /// The scientific pathway used for carbon removal.
    pub removal_pathway: ClimateSupplierRemovalPathway,
}

impl Object for ClimateSupplier {
    type Id = ClimateSupplierId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "climate.supplier"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
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

/// An enum representing the possible values of an `ClimateSupplier`'s `removal_pathway` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ClimateSupplierRemovalPathway {
    BiomassCarbonRemovalAndStorage,
    DirectAirCapture,
    EnhancedWeathering,
}

impl ClimateSupplierRemovalPathway {
    pub fn as_str(self) -> &'static str {
        match self {
            ClimateSupplierRemovalPathway::BiomassCarbonRemovalAndStorage => "biomass_carbon_removal_and_storage",
            ClimateSupplierRemovalPathway::DirectAirCapture => "direct_air_capture",
            ClimateSupplierRemovalPathway::EnhancedWeathering => "enhanced_weathering",
        }
    }
}

impl AsRef<str> for ClimateSupplierRemovalPathway {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ClimateSupplierRemovalPathway {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
impl std::default::Default for ClimateSupplierRemovalPathway {
    fn default() -> Self {
        Self::BiomassCarbonRemovalAndStorage
    }
}
