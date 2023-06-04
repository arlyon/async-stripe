#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct DeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    pub maximum: Option<
        crate::shipping_rate::delivery_estimate::delivery_estimate_bound::DeliveryEstimateBound,
    >,
    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    pub minimum: Option<
        crate::shipping_rate::delivery_estimate::delivery_estimate_bound::DeliveryEstimateBound,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for DeliveryEstimate {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod delivery_estimate_bound;
pub use delivery_estimate_bound::DeliveryEstimateBound;
