#[derive(Clone, Debug, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Tax {
    /// Amount of tax applied for this rate.
    pub amount: i64,
    pub rate: crate::tax_rate::TaxRate,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Tax {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
