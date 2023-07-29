#[derive(Clone, Debug, Default, serde::Serialize)]
#[cfg_attr(not(feature = "min-ser"), derive(serde::Deserialize))]
pub struct Customs {
    /// A registration number used for customs in Europe.
    ///
    /// See <https://www.gov.uk/eori> and <https://ec.europa.eu/taxation_customs/business/customs-procedures-import-and-export/customs-procedures/economic-operators-registration-and-identification-number-eori_en>.
    pub eori_number: Option<String>,
}
#[cfg(feature = "min-ser")]
impl miniserde::Deserialize for Customs {
    fn begin(_out: &mut Option<Self>) -> &mut dyn miniserde::de::Visitor {
        todo!()
    }
}
