/// A Climate product represents a type of carbon removal unit available for reservation.
/// You can retrieve it to see the current price and availability.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClimateProduct {
    /// Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: stripe_types::Timestamp,
    /// Current prices for a metric ton of carbon removal in a currency's smallest unit.
    pub current_prices_per_metric_ton:
        std::collections::HashMap<String, stripe_misc::ClimateRemovalsProductsPrice>,
    /// The year in which the carbon removal is expected to be delivered.
    pub delivery_year: Option<i64>,
    /// Unique identifier for the object. For convenience, Climate product IDs are human-readable strings
    /// that start with `climsku_`.
    /// See [carbon removal inventory](https://stripe.com/docs/climate/orders/carbon-removal-inventory).
    /// for a list of available carbon removal products.
    pub id: stripe_misc::ClimateProductId,
    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    /// The quantity of metric tons available for reservation.
    pub metric_tons_available: String,
    /// The Climate product's name.
    pub name: String,
    /// The carbon removal suppliers that fulfill orders for this Climate product.
    pub suppliers: Vec<stripe_misc::ClimateSupplier>,
}
impl stripe_types::Object for ClimateProduct {
    type Id = stripe_misc::ClimateProductId;
    fn id(&self) -> &Self::Id {
        &self.id
    }
}
stripe_types::def_id!(ClimateProductId);
