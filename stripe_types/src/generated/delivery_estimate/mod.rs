#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    pub maximum: Option<stripe_types::delivery_estimate_bound::DeliveryEstimateBound>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    pub minimum: Option<stripe_types::delivery_estimate_bound::DeliveryEstimateBound>,
}
