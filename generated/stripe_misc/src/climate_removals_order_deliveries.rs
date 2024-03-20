/// The delivery of a specified quantity of carbon for an order.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ClimateRemovalsOrderDeliveries {
    /// Time at which the delivery occurred. Measured in seconds since the Unix epoch.
    pub delivered_at: stripe_types::Timestamp,
    /// Specific location of this delivery.
    pub location: Option<stripe_misc::ClimateRemovalsLocation>,
    /// Quantity of carbon removal supplied by this delivery.
    pub metric_tons: String,
    /// Once retired, a URL to the registry entry for the tons from this delivery.
    pub registry_url: Option<String>,
    pub supplier: stripe_misc::ClimateSupplier,
}
