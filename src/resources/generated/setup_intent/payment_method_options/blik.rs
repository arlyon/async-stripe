#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Blik {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options: Option<
        crate::setup_intent::payment_method_options::blik_mandate_options::BlikMandateOptions,
    >,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Blik {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
