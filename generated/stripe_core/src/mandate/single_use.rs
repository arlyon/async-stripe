#[derive(Copy, Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct SingleUse {
    /// On a single use mandate, the amount of the payment.
    pub amount: i64,
    /// On a single use mandate, the currency of the payment.
    pub currency: stripe_types::Currency,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for SingleUse {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
