#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
pub maximum: Option<stripe_types::shipping_rate::delivery_estimate::delivery_estimate_bound::DeliveryEstimateBound>,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
pub minimum: Option<stripe_types::shipping_rate::delivery_estimate::delivery_estimate_bound::DeliveryEstimateBound>,

}
pub mod delivery_estimate_bound;
pub use delivery_estimate_bound::DeliveryEstimateBound;
