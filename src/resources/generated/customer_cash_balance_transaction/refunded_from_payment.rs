#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct RefundedFromPayment {
    /// The [Refund](https://stripe.com/docs/api/refunds/object) that moved these funds into the customer's cash balance.
    pub refund: crate::Expandable<crate::refund::Refund>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for RefundedFromPayment {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
