#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Billing {
    pub address: stripe_types::address::Address,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Billing {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
