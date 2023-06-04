#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct LinkedFlows {
    /// Set if there is an Issuing dispute associated with the DebitReversal.
    pub issuing_dispute: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for LinkedFlows {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
