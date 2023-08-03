#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ShippingRateDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    pub maximum: Option<stripe_types::ShippingRateDeliveryEstimateBound>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    pub minimum: Option<stripe_types::ShippingRateDeliveryEstimateBound>,
}
