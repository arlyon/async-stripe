#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct PortalFlowsCouponOffer {
    /// The ID of the coupon to be offered.
    pub coupon: String,
}
