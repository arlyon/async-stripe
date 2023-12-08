// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::{ClimateProductId};
use crate::params::{Object, Timestamp};
use crate::resources::{ClimateSupplier};
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "ClimateRemovalsProducts".
///
/// For more details see <https://stripe.com/docs/api/climate/product/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClimateProduct {
    /// Unique identifier for the object.
    ///
    /// For convenience, Climate product IDs are human-readable strings that start with `climsku_`.
    /// See [carbon removal inventory](https://stripe.com/docs/climate/orders/carbon-removal-inventory) for a list of available carbon removal products.
    pub id: ClimateProductId,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Current prices for a metric ton of carbon removal in a currency's smallest unit.
    pub current_prices_per_metric_ton: ClimateRemovalsProductsPrice,

    /// The year in which the carbon removal is expected to be delivered.
    pub delivery_year: Option<i64>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The quantity of metric tons available for reservation.
    pub metric_tons_available: String,

    /// The Climate product's name.
    pub name: String,

    /// The carbon removal suppliers that fulfill orders for this Climate product.
    pub suppliers: Vec<ClimateSupplier>,
}

impl Object for ClimateProduct {
    type Id = ClimateProductId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "climate.product"
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ClimateRemovalsProductsPrice {

    /// Fees for one metric ton of carbon removal in the currency's smallest unit.
    pub amount_fees: i64,

    /// Subtotal for one metric ton of carbon removal (excluding fees) in the currency's smallest unit.
    pub amount_subtotal: i64,

    /// Total for one metric ton of carbon removal (including fees) in the currency's smallest unit.
    pub amount_total: i64,
}
