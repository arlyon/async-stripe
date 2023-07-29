#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Installments {
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<
        stripe_types::payment_intent::payment_method_options::card::installments::plan::Plan,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Installments {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
