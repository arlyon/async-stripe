#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Konbini {
    /// If the payment succeeded, this contains the details of the convenience store where the payment was completed.
    pub store: Option<stripe_types::charge::payment_method_details::store::Store>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Konbini {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
