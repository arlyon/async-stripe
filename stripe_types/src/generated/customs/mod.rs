#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Customs {
    /// A registration number used for customs in Europe.
    ///
    /// See [<https://www.gov.uk/eori>](https://www.gov.uk/eori) for the UK and [<https://ec.europa.eu/taxation_customs/business/customs-procedures-import-and-export/customs-procedures/economic-operators-registration-and-identification-number-eori_en>](https://ec.europa.eu/taxation_customs/business/customs-procedures-import-and-export/customs-procedures/economic-operators-registration-and-identification-number-eori_en) for the EU.
    pub eori_number: Option<String>,
}
