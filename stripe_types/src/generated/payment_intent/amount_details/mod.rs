#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct AmountDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<stripe_types::payment_intent::amount_details::tip::Tip>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for AmountDetails {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}

pub mod tip;
pub use tip::Tip;
