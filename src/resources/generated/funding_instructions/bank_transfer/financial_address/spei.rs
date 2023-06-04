/// SPEI Records contain Mexico bank account details per the SPEI format.
#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Spei {
    /// The three-digit bank code.
    pub bank_code: String,
    /// The short banking institution name.
    pub bank_name: String,
    /// The CLABE number.
    pub clabe: String,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Spei {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
